//! In-memory files, for hosts that have no file system.
//!
//! Compiled only with the `memfs` feature. RustGIS runs a WebAssembly plugin
//! in a sandbox with no files, no clock and no environment
//! (`wasm32-unknown-unknown`, three host imports and nothing else), so the
//! engine's `fopen` cannot reach a disk there. With `memfs`, `cio::CFile` is
//! this type instead of the `std::fs` one: the host puts the input file in
//! the table with [`put`], runs the engine by file name exactly as on a
//! disk, and takes the report and output files back with [`get`].
//!
//! The default build does not compile this module at all; `cio::CFile` there
//! is the original `std::fs` type, unchanged.
//!
//! Everything here is safe Rust. The table is thread-local; a WebAssembly
//! plugin is single-threaded and RustGIS gives every call a fresh instance,
//! so one run never sees another's files.

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::cio::cstr;

type Buf = Rc<RefCell<Vec<u8>>>;

thread_local! {
    static FILES: RefCell<BTreeMap<Vec<u8>, Buf>> = RefCell::new(BTreeMap::new());
    static CLOCK: Cell<i64> = const { Cell::new(0) };
    static RESOLVED: RefCell<Vec<(Vec<u8>, Vec<u8>)>> = const { RefCell::new(Vec::new()) };
}

/// Names the engine asked for that were found only by their own name (or that name in another
/// case): (name asked for, file used), in the order they were opened. A host reports these so
/// the user can see which data file stood in for which name in the .inp.
pub fn resolved() -> Vec<(Vec<u8>, Vec<u8>)> {
    RESOLVED.with(|r| r.borrow().clone())
}

/// Run `f` on a file's bytes without copying them.
pub fn with<R>(name: &[u8], f: impl FnOnce(&[u8]) -> R) -> Option<R> {
    FILES.with(|t| t.borrow().get(&key(name)).map(|b| f(&b.borrow())))
}

fn key(name: &[u8]) -> Vec<u8> {
    cstr(name).to_vec()
}

/// The part of a name after its last `/` or `\\`.
fn base_name(k: &[u8]) -> &[u8] {
    match k.iter().rposition(|&c| c == b'/' || c == b'\\') {
        Some(i) => &k[i + 1..],
        None => k,
    }
}

/// Put (or replace) a file.
pub fn put(name: &[u8], data: Vec<u8>) {
    FILES.with(|f| {
        f.borrow_mut().insert(key(name), Rc::new(RefCell::new(data)));
    });
}

/// A copy of a file's bytes.
pub fn get(name: &[u8]) -> Option<Vec<u8>> {
    FILES.with(|f| f.borrow().get(&key(name)).map(|b| b.borrow().clone()))
}

/// Does the file exist?
pub fn exists(name: &[u8]) -> bool {
    FILES.with(|f| f.borrow().contains_key(&key(name)))
}

/// Every file's name and size, in name order.
pub fn list() -> Vec<(Vec<u8>, usize)> {
    FILES.with(|f| f.borrow().iter().map(|(k, v)| (k.clone(), v.borrow().len())).collect())
}

/// Remove every file.
pub fn clear() {
    FILES.with(|f| f.borrow_mut().clear());
    RESOLVED.with(|r| r.borrow_mut().clear());
}

/// Create an empty file only if none has the name (`open(O_CREAT|O_EXCL)`);
/// false if it exists already.
pub fn create_new(name: &[u8]) -> bool {
    FILES.with(|f| {
        let mut m = f.borrow_mut();
        let k = key(name);
        if k.is_empty() || m.contains_key(&k) {
            false
        } else {
            m.insert(k, Rc::new(RefCell::new(Vec::new())));
            true
        }
    })
}

/// The time `time(NULL)` answers, in seconds since the Unix epoch. A sandbox
/// has no clock, so it is whatever the host sets (0, 1 Jan 1970, unless set).
/// It reaches only the report's three wall-clock lines.
pub fn set_clock(t: i64) {
    CLOCK.with(|c| c.set(t));
}

pub fn clock() -> i64 {
    CLOCK.with(|c| c.get())
}

/// C `remove`.
pub fn remove(name: &[u8]) -> i32 {
    FILES.with(|f| if f.borrow_mut().remove(&key(name)).is_some() { 0 } else { -1 })
}

/// A C `FILE*` over an in-memory file, with the stdio rules SWMM relies on
/// that the `std::fs` version keeps: `fgets` keeps the newline, `feof`
/// turns true only after a read hits the end, `rewind` and `fseek` clear it,
/// and writing past the end fills the gap with zeros.
pub struct CFile {
    data: Buf,
    pos: usize,
    append: bool,
    eof: bool,
    err: bool,
}

impl CFile {
    /// `fopen`. Modes as SWMM uses them: "r"/"rt"/"rb", "w"/"wt"/"wb",
    /// "r+b", "w+b" (and "a", "a+").
    pub fn open(path: &[u8], mode: &str) -> Option<CFile> {
        let k = key(path);
        if k.is_empty() {
            return None;
        }
        let m = mode.replace(['t', 'b'], "");
        FILES.with(|f| {
            let mut files = f.borrow_mut();
            let data = match m.as_str() {
                // A file a model names with a folder ("C:\\data\\rain.dat", "../rain.dat")
                // is found by its own name when no file has the whole name: the host can
                // only hand over files by name.
                "r" | "r+" => match files.get(&k) {
                    Some(b) => b.clone(),
                    None => {
                        let base = base_name(&k).to_vec();
                        let hit = files
                            .iter()
                            .find(|(n, _)| **n == base)
                            .or_else(|| files.iter().find(|(n, _)| n.eq_ignore_ascii_case(&base)))?;
                        let (name, b) = (hit.0.clone(), hit.1.clone());
                        RESOLVED.with(|r| r.borrow_mut().push((k.clone(), name)));
                        b
                    }
                },
                "w" | "w+" => {
                    let b: Buf = Rc::new(RefCell::new(Vec::new()));
                    files.insert(k, b.clone());
                    b
                }
                "a" | "a+" => files.entry(k).or_insert_with(|| Rc::new(RefCell::new(Vec::new()))).clone(),
                _ => return None,
            };
            Some(CFile { data, pos: 0, append: m.starts_with('a'), eof: false, err: false })
        })
    }

    /// `fgets(buf, n, f)`: reads at most n-1 bytes, stopping after a newline.
    /// Returns the bytes read (with the newline, without a terminator), or
    /// None at end of file with nothing read.
    pub fn fgets(&mut self, n: usize) -> Option<Vec<u8>> {
        let mut line = Vec::new();
        if n <= 1 {
            return if n == 1 { Some(line) } else { None };
        }
        let d = self.data.borrow();
        while line.len() < n - 1 {
            if self.pos >= d.len() {
                self.eof = true;
                break;
            }
            let c = d[self.pos];
            self.pos += 1;
            line.push(c);
            if c == b'\n' {
                break;
            }
        }
        if line.is_empty() {
            None
        } else {
            Some(line)
        }
    }

    /// `fgetc`; None is EOF.
    pub fn fgetc(&mut self) -> Option<u8> {
        let d = self.data.borrow();
        if self.pos >= d.len() {
            self.eof = true;
            return None;
        }
        let c = d[self.pos];
        self.pos += 1;
        Some(c)
    }

    /// `fread` into `buf`; returns bytes read (C returns items; callers divide).
    pub fn read_bytes(&mut self, buf: &mut [u8]) -> usize {
        let d = self.data.borrow();
        // (a position past the end, after an fseek, reads nothing, as on a disk)
        let start = self.pos.min(d.len());
        let k = (d.len() - start).min(buf.len());
        buf[..k].copy_from_slice(&d[start..start + k]);
        self.pos += k;
        if k < buf.len() {
            self.eof = true;
        }
        k
    }

    /// `fwrite` / `fputs` / the output side of `fprintf`.
    pub fn write_bytes(&mut self, b: &[u8]) {
        let mut d = self.data.borrow_mut();
        if self.append {
            self.pos = d.len();
        }
        let end = self.pos + b.len();
        if d.len() < self.pos {
            d.resize(self.pos, 0);
        }
        let overlap = d.len().min(end) - self.pos;
        d[self.pos..self.pos + overlap].copy_from_slice(&b[..overlap]);
        d.extend_from_slice(&b[overlap..]);
        self.pos = end;
    }

    /// `ftell`.
    pub fn ftell(&mut self) -> i64 {
        self.pos as i64
    }

    /// `fseek(f, pos, SEEK_SET)`; clears the end-of-file flag.
    pub fn fseek_set(&mut self, pos: i64) -> i32 {
        self.eof = false;
        self.pos = pos.max(0) as usize;
        0
    }

    /// `fseek(f, off, SEEK_END)`.
    pub fn fseek_end(&mut self, off: i64) -> i32 {
        self.eof = false;
        let p = self.data.borrow().len() as i64 + off;
        if p < 0 {
            return -1;
        }
        self.pos = p as usize;
        0
    }

    /// `rewind`: seek to the start, clear end-of-file and error.
    pub fn rewind(&mut self) {
        self.fseek_set(0);
        self.err = false;
    }

    /// `feof`.
    pub fn feof(&self) -> bool {
        self.eof
    }

    /// `fflush` (nothing is buffered).
    pub fn fflush(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stdio_rules() {
        clear();
        let mut w = CFile::open(b"a.txt\0", "wt").unwrap();
        w.write_bytes(b"one\ntwo");
        drop(w);
        let mut r = CFile::open(b"a.txt", "rt").unwrap();
        assert_eq!(r.fgets(80).unwrap(), b"one\n");
        assert!(!r.feof());
        assert_eq!(r.fgets(80).unwrap(), b"two");
        assert!(r.feof());
        assert!(r.fgets(80).is_none());
        r.rewind();
        assert!(!r.feof());
        assert_eq!(r.fgetc(), Some(b'o'));
        assert!(CFile::open(b"missing", "r").is_none());
        let mut b = CFile::open(b"b.bin", "w+b").unwrap();
        b.fseek_set(4);
        b.write_bytes(&[9, 9]);
        b.fseek_set(0);
        let mut buf = [1u8; 8];
        assert_eq!(b.read_bytes(&mut buf), 6);
        assert_eq!(&buf[..6], &[0, 0, 0, 0, 9, 9]);
        assert!(b.feof());
        assert_eq!(b.fseek_end(-2), 0);
        assert_eq!(b.ftell(), 4);
        put(b"rain.dat", b"x".to_vec());
        assert!(CFile::open(b"C:\\data\\rain.dat", "r").is_some());
        assert!(CFile::open(b"../in/rain.dat", "r").is_some());
        assert!(CFile::open(b"../in/other.dat", "r").is_none());
        assert!(CFile::open(b"D:\\x\\RAIN.DAT", "r").is_some());
        assert_eq!(resolved().last().unwrap().1, b"rain.dat".to_vec());
        let mut r2 = CFile::open(b"rain.dat", "r").unwrap();
        r2.fseek_set(10);
        let mut small = [0u8; 4];
        assert_eq!(r2.read_bytes(&mut small), 0);
        assert!(r2.feof());
        assert_eq!(with(b"rain.dat", |b| b.len()), Some(1));
        assert!(create_new(b"c"));
        assert!(!create_new(b"c"));
        assert_eq!(remove(b"c"), 0);
        assert_eq!(remove(b"c"), -1);
    }
}
