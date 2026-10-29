#!/usr/bin/env python3
"""Minimal reader for SWMM 5 binary .out files (EPA 5.2 layout)."""
import struct

class Out:
    def __init__(self, path):
        b = self.b = open(path, 'rb').read()
        i = lambda o: struct.unpack_from('<i', b, o)[0]
        self.magic, self.version, self.flowUnits, nS, nN, nL, nP = struct.unpack_from('<7i', b, 0)
        self.n = dict(sub=nS, node=nN, link=nL, poll=nP)
        idPos, propPos, resPos, nPer, err, magic2 = struct.unpack_from('<6i', b, len(b) - 24)
        self.nPer, self.err, self.resPos = nPer, err, resPos
        o = idPos
        ids = {}
        for k in ('sub', 'node', 'link', 'poll'):
            ids[k] = []
            for _ in range(self.n[k]):
                L = i(o); o += 4
                ids[k].append(b[o:o + L].decode('latin-1')); o += L
        self.ids = ids
        # reporting variables follow the properties; find them from propPos
        o = propPos
        o += 4 + 4 * i(o) + 4 * nS * 1 if False else 0
        o = propPos
        n = i(o); o += 4 + 4 * n + 4 * nS * n          # subcatch props (area)
        n = i(o); o += 4 + 4 * n + 4 * nN * n          # node props
        n = i(o); o += 4 + 4 * n + 4 * nL * n          # link props
        self.nv = {}
        for k in ('sub', 'node', 'link', 'sys'):
            n = i(o); self.nv[k] = n; o += 4 + 4 * n
        self.start = struct.unpack_from('<d', b, o)[0]; o += 8
        self.step = i(o)
        self.bytesPer = 8 + 4 * (nS * self.nv['sub'] + nN * self.nv['node'] + nL * self.nv['link'] + self.nv['sys'])

    def series(self, kind, idx, var):
        nS, nN, nL = self.n['sub'], self.n['node'], self.n['link']
        off = {'sub': 0, 'node': nS * self.nv['sub'], 'link': nS * self.nv['sub'] + nN * self.nv['node'],
               'sys': nS * self.nv['sub'] + nN * self.nv['node'] + nL * self.nv['link']}[kind]
        per = self.nv[kind]
        out = []
        for k in range(self.nPer):
            o = self.resPos + k * self.bytesPer + 8 + 4 * (off + (idx * per if kind != 'sys' else 0) + var)
            out.append(struct.unpack_from('<f', self.b, o)[0])
        return out

    def index(self, kind, name): return self.ids[kind].index(name)

    def all_values(self):
        """every float of every period, as one list per period (for whole-file comparisons)"""
        n = (self.bytesPer - 8) // 4
        return [struct.unpack_from(f'<{n}f', self.b, self.resPos + k * self.bytesPer + 8) for k in range(self.nPer)]
