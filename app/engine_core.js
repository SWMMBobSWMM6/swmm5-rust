// engine_core.js -- run SWMM wasm reactors (EPA 5.2.4 C, swmm5-cplus C++, swmm5-rust)
// in memory. Shared verbatim by the app's Web Worker and the node test harness.
//
// MemWasi: a WASI preview1 layer over an in-memory file table with geometric
// buffer growth. The C and C++ engines (wasi-libc) use 13 calls; the Rust
// engine's standard library adds six more (file and path stat, fd_tell,
// mkdir, and the environment), 19 in all.
(function (root) {
  'use strict';
  const E = { SUCCESS: 0, BADF: 8, EXIST: 20, INVAL: 28, ISDIR: 31, NOENT: 44, NOSYS: 52 };

  function MemWasi(initial) {
    const FS = new Map();                     // path (no leading '/') -> {buf, len}
    if (initial) for (const [k, v] of Object.entries(initial)) {
      const b = v instanceof Uint8Array ? v : new TextEncoder().encode(String(v));
      FS.set(norm(k), { buf: b, len: b.length });
    }
    const DIRS = new Set(['']);
    const fds = new Map();
    fds.set(0, { kind: 'stdio' }); fds.set(1, { kind: 'stdio' }); fds.set(2, { kind: 'stdio' });
    fds.set(3, { kind: 'dir', name: '' });
    let nextFd = 4, memory = null, stdout = '';
    const dec = new TextDecoder('latin1');
    function norm(p) { return String(p).replace(/^\/+/, '').replace(/^\.\//, ''); }
    function dv() { return new DataView(memory.buffer); }
    function u8() { return new Uint8Array(memory.buffer); }
    function str(ptr, len) { return dec.decode(u8().subarray(ptr, ptr + len)); }
    function ensure(f, need) {
      if (need <= f.buf.length) return;
      let cap = Math.max(need, f.buf.length * 2, 4096);
      const nb = new Uint8Array(cap); nb.set(f.buf.subarray(0, f.len)); f.buf = nb;
    }
    // WASI filestat: dev u64, ino u64, filetype u8 (+7), nlink u64, size u64, atim, mtim, ctim
    function filestat(buf, type, size) {
      const d = dv();
      for (let o = 0; o < 64; o += 8) d.setBigUint64(buf + o, 0n, true);
      d.setUint8(buf + 16, type); d.setBigUint64(buf + 24, 1n, true); d.setBigUint64(buf + 32, BigInt(size), true);
    }
    class ExitError extends Error { constructor(code) { super('exit ' + code); this.code = code; } }
    const imports = {
      fd_prestat_get(fd, buf) {
        if (fd !== 3) return E.BADF;
        const d = dv(); d.setUint8(buf, 0); d.setUint32(buf + 4, 1, true); return E.SUCCESS;
      },
      fd_prestat_dir_name(fd, path, len) {
        if (fd !== 3) return E.BADF; u8()[path] = 47; return E.SUCCESS;
      },
      fd_fdstat_get(fd, buf) {
        const h = fds.get(fd); if (!h) return E.BADF;
        const d = dv();
        d.setUint8(buf, h.kind === 'stdio' ? 2 : h.kind === 'dir' ? 3 : 4);
        d.setUint16(buf + 2, 0, true);
        d.setBigUint64(buf + 8, 0xFFFFFFFFFFFFFFFFn, true);
        d.setBigUint64(buf + 16, 0xFFFFFFFFFFFFFFFFn, true);
        return E.SUCCESS;
      },
      fd_fdstat_set_flags() { return E.SUCCESS; },
      fd_filestat_get(fd, buf) {
        const h = fds.get(fd); if (!h) return E.BADF;
        if (h.kind === 'file') filestat(buf, 4, h.file.len);
        else filestat(buf, h.kind === 'dir' ? 3 : 2, 0);
        return E.SUCCESS;
      },
      path_filestat_get(dirfd, flags, pathPtr, pathLen, buf) {
        const name = norm(str(pathPtr, pathLen));
        const f = FS.get(name);
        if (f) { filestat(buf, 4, f.len); return E.SUCCESS; }
        if (DIRS.has(name.replace(/\/+$/, ''))) { filestat(buf, 3, 0); return E.SUCCESS; }
        return E.NOENT;
      },
      path_create_directory(dirfd, pathPtr, pathLen) {
        const name = norm(str(pathPtr, pathLen)).replace(/\/+$/, '');
        if (DIRS.has(name) || FS.has(name)) return E.EXIST;
        DIRS.add(name); return E.SUCCESS;
      },
      path_open(dirfd, dirflags, pathPtr, pathLen, oflags, rb, ri, fdflags, fdOut) {
        const name = norm(str(pathPtr, pathLen));
        if (oflags & 2) {                                    // O_DIRECTORY
          if (!DIRS.has(name.replace(/\/+$/, ''))) return E.NOENT;
          const fd = nextFd++; fds.set(fd, { kind: 'dir', name }); dv().setUint32(fdOut, fd, true); return E.SUCCESS;
        }
        let f = FS.get(name);
        if (f && (oflags & 4)) return E.EXIST;               // O_EXCL
        if (!f) {
          if (DIRS.has(name)) return E.ISDIR;
          if (!(oflags & 1)) return E.NOENT;                 // no O_CREAT
          f = { buf: new Uint8Array(0), len: 0 }; FS.set(name, f);
        }
        if (oflags & 8) f.len = 0;                           // O_TRUNC
        const fd = nextFd++;
        fds.set(fd, { kind: 'file', name, file: f, pos: 0, append: !!(fdflags & 1) });
        dv().setUint32(fdOut, fd, true);
        return E.SUCCESS;
      },
      fd_close(fd) { if (fd > 3) fds.delete(fd); return E.SUCCESS; },
      fd_read(fd, iovs, n, nreadPtr) {
        const h = fds.get(fd); if (!h) return E.BADF;
        const d = dv(); let total = 0;
        if (h.kind === 'file') {
          const m = u8();
          for (let i = 0; i < n; i++) {
            const p = d.getUint32(iovs + 8 * i, true), l = d.getUint32(iovs + 8 * i + 4, true);
            const k = Math.max(0, Math.min(l, h.file.len - h.pos));
            m.set(h.file.buf.subarray(h.pos, h.pos + k), p);
            h.pos += k; total += k;
            if (k < l) break;
          }
        }
        d.setUint32(nreadPtr, total, true); return E.SUCCESS;
      },
      fd_write(fd, iovs, n, nwPtr) {
        const h = fds.get(fd); if (!h) return E.BADF;
        const d = dv(), m = u8(); let total = 0;
        for (let i = 0; i < n; i++) {
          const p = d.getUint32(iovs + 8 * i, true), l = d.getUint32(iovs + 8 * i + 4, true);
          if (h.kind === 'stdio') { if (stdout.length < 20000) stdout += dec.decode(m.subarray(p, p + l)); }
          else if (h.kind === 'file') {
            if (h.append) h.pos = h.file.len;
            ensure(h.file, h.pos + l);
            h.file.buf.set(m.subarray(p, p + l), h.pos);
            h.pos += l; if (h.pos > h.file.len) h.file.len = h.pos;
          }
          total += l;
        }
        d.setUint32(nwPtr, total, true); return E.SUCCESS;
      },
      fd_seek(fd, offset, whence, outPtr) {
        const h = fds.get(fd); if (!h || h.kind !== 'file') return E.BADF;
        const off = Number(offset);
        let np = whence === 0 ? off : whence === 1 ? h.pos + off : h.file.len + off;
        if (np < 0) return E.INVAL;
        h.pos = np; dv().setBigUint64(outPtr, BigInt(np), true); return E.SUCCESS;
      },
      fd_tell(fd, outPtr) {
        const h = fds.get(fd); if (!h || h.kind !== 'file') return E.BADF;
        dv().setBigUint64(outPtr, BigInt(h.pos), true); return E.SUCCESS;
      },
      path_unlink_file(dirfd, p, l) {
        const name = norm(str(p, l)); return FS.delete(name) ? E.SUCCESS : E.NOENT;
      },
      path_remove_directory(dirfd, p, l) {
        const name = norm(str(p, l)).replace(/\/+$/, ''); return DIRS.delete(name) ? E.SUCCESS : E.NOENT;
      },
      environ_sizes_get(countPtr, sizePtr) { const d = dv(); d.setUint32(countPtr, 0, true); d.setUint32(sizePtr, 0, true); return E.SUCCESS; },
      environ_get() { return E.SUCCESS; },
      clock_time_get(id, prec, outPtr) {
        dv().setBigUint64(outPtr, BigInt(Math.round(Date.now())) * 1000000n, true); return E.SUCCESS;
      },
      proc_exit(code) { throw new ExitError(code); },
    };
    return {
      imports, FS,
      bind(mem) { memory = mem; },
      read(name) { const f = FS.get(norm(name)); return f ? f.buf.slice(0, f.len) : null; },
      write(name, data) {
        const b = data instanceof Uint8Array ? data : new TextEncoder().encode(String(data));
        FS.set(norm(name), { buf: b, len: b.length });
      },
      unlink(name) { FS.delete(norm(name)); },
      get stdout() { return stdout; },
    };
  }

  // one wasm instance with its own memory file system. kind: 'epa' | 'cplus' | 'rust'
  function Engine(module, files) {
    const fs = MemWasi(files);
    const inst = new WebAssembly.Instance(module, { wasi_snapshot_preview1: fs.imports });
    fs.bind(inst.exports.memory);
    if (inst.exports._initialize) inst.exports._initialize();
    const ex = inst.exports;
    const kind = typeof ex.swmm5rs_create === 'function' ? 'rust' : typeof ex.swmm5cp_create === 'function' ? 'cplus' : 'epa';
    const P = kind === 'rust' ? 'swmm5rs_' : kind === 'cplus' ? 'swmm5cp_' : null;
    const malloc = ex.malloc || ex.swmm5rs_malloc, free = ex.free || ex.swmm5rs_free;
    // handle API: h(name) returns the engine's handle function (swmm5cp_* or swmm5rs_*)
    const h = (name) => ex[P + name];
    const enc = new TextEncoder();
    function cstr(s) {
      const b = enc.encode(s + '\0'); const p = malloc(b.length);
      new Uint8Array(ex.memory.buffer, p, b.length).set(b); return p;
    }
    function errText(hd) {
      const p = malloc(256);
      if (hd === undefined || !P) ex.swmm_getError(p, 255); else h('getError')(hd, p, 255);
      const m = new Uint8Array(ex.memory.buffer, p, 256);
      let s = ''; for (let i = 0; i < 256 && m[i]; i++) s += String.fromCharCode(m[i]);
      free(p); return s.trim();
    }
    const dbl = malloc(8);
    function f64() { return new Float64Array(ex.memory.buffer, dbl, 1)[0]; }
    return { ex, fs, kind, h, cstr, errText, dblPtr: dbl, f64, malloc, free };
  }

  const now = () => (typeof performance !== 'undefined' ? performance : Date).now();
  // the message a Rust panic prints (stderr) before the trap, when there is one
  function panicText(eng) { const m = /panicked at[^\n]*\n?[^\n]*/.exec(eng.fs.stdout); return m ? m[0].trim() : ''; }

  // whole run through the legacy swmm_run() (all three engines export it)
  function runLegacy(module, inpName, files) {
    const eng = Engine(module, files);
    const t0 = now();
    let rc, trap = '';
    try {
      rc = eng.ex.swmm_run(eng.cstr('/' + inpName), eng.cstr('/m.rpt'), eng.cstr('/m.out'));
    } catch (e) { rc = e && e.code !== undefined ? 'exit ' + e.code : 'trap'; trap = e && e.code === undefined ? String(e.message || e) : ''; }
    const ms = now() - t0;
    let version = 0; try { version = eng.ex.swmm_getVersion ? eng.ex.swmm_getVersion() : 0; } catch (e) {}
    return { rc, ms, trap, panic: panicText(eng), kind: eng.kind, rpt: eng.fs.read('m.rpt'), out: eng.fs.read('m.out'), version };
  }

  // whole run through a handle (swmm5cp_ or swmm5rs_); EPA falls back to runLegacy
  function runHandle(module, inpName, files) {
    const eng = Engine(module, files);
    if (eng.kind === 'epa') return runLegacy(module, inpName, files);
    const t0 = now();
    let rc, trap = '', hd = -1;
    try {
      hd = eng.h('create')();
      rc = eng.h('run')(hd, eng.cstr('/' + inpName), eng.cstr('/m.rpt'), eng.cstr('/m.out'));
    } catch (e) { rc = e && e.code !== undefined ? 'exit ' + e.code : 'trap'; trap = e && e.code === undefined ? String(e.message || e) : ''; }
    const ms = now() - t0;
    let version = 0;
    if (!trap) { try { version = eng.h('getVersion')(hd); eng.h('destroy')(hd); } catch (e) {} }
    return { rc, ms, trap, panic: panicText(eng), kind: eng.kind, rpt: eng.fs.read('m.rpt'), out: eng.fs.read('m.out'), version };
  }

  // ---------------------------------------------------------------- multi-model
  // Open every model in ONE instance (one handle each) and step them round-robin.
  // onProgress(i, steps, elapsedDays, done) is called every `every` steps.
  function runMulti(module, models, onProgress, every) {
    const fsFiles = {};
    models.forEach((m, i) => { fsFiles['m' + i + '.inp'] = m.inp; Object.assign(fsFiles, m.extra || {}); });
    const eng = Engine(module, fsFiles);
    const f = eng.h, H = [];
    for (let i = 0; i < models.length; i++) {
      const hd = f('create')();
      let rc = f('open')(hd, eng.cstr('/m' + i + '.inp'), eng.cstr('/m' + i + '.rpt'), eng.cstr('/m' + i + '.out'));
      if (!rc) rc = f('start')(hd, 1);
      H.push({ hd, rc, steps: 0, done: rc !== 0, t: 0 });
    }
    const live = f('count')();
    every = every || 2000;
    let anyLeft = true, round = 0;
    while (anyLeft) {
      anyLeft = false;
      for (let i = 0; i < H.length; i++) {
        const x = H[i]; if (x.done) continue;
        const rc = f('step')(x.hd, eng.dblPtr);
        x.steps++; x.t = eng.f64();
        if (rc || x.t <= 0) { x.done = true; if (rc) x.rc = rc; }
        else anyLeft = true;
        if (onProgress && (x.steps % every === 0 || x.done)) onProgress(i, x.steps, x.t, x.done);
      }
      round++;
    }
    const res = H.map((x, i) => {
      f('end')(x.hd);
      const err = x.rc ? eng.errText(x.hd) : '';
      // no report call: swmm_run() writes the time-series tables only for a scratch .out,
      // and these runs name their .out, so the reports match a solo swmm_run()
      f('close')(x.hd);
      f('destroy')(x.hd);
      return { rc: x.rc, steps: x.steps, err, rpt: eng.fs.read('m' + i + '.rpt'), out: eng.fs.read('m' + i + '.out') };
    });
    return { live, after: f('count')(), rounds: round, res };
  }

  root.SwmmEngineCore = { MemWasi, Engine, runLegacy, runHandle, runMulti };
  if (typeof module !== 'undefined' && module.exports) module.exports = root.SwmmEngineCore;
})(typeof self !== 'undefined' ? self : globalThis);
