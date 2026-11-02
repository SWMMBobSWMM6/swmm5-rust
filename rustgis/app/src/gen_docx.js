// The Doc tab as a Word file, from the same source: the built page's APP.doc, with each figure
// screenshotted from the page (light theme) so the two carry the same pictures.
//   node gen_docx.js page.html out.docx
const fs = require('fs'), path = require('path');
const { chromium } = require('/opt/npm-tools/node_modules/playwright');
const D = require('/opt/npm-tools/node_modules/docx');
const { Document, Packer, Paragraph, TextRun, HeadingLevel, ImageRun, Table, TableRow, TableCell, WidthType, ShadingType,
  AlignmentType, ExternalHyperlink, LevelFormat, BorderStyle, Footer, PageNumber } = D;

(async () => {
  const [page, out] = process.argv.slice(2);
  const browser = await chromium.launch({ executablePath: '/opt/pw-browsers/chromium-1243/chrome-linux64/chrome' }).catch(() => chromium.launch());
  const p = await browser.newPage({ viewport: { width: 1100, height: 900 }, deviceScaleFactor: 2 });
  await p.goto('file://' + page);
  await p.evaluate(() => { try { localStorage.setItem('swmm5rust_rustgis_seen', '1'); } catch (e) {} });
  await p.click('#helpclose').catch(() => {});
  await p.click('#theme-toggle button[data-t="light"]');
  await p.click('#tabs button[data-v="doc"]');
  const doc = await p.evaluate(() => window.APP.doc);
  const figs = {};
  for (const el of await p.$$('#essay figure[data-fig]')) {
    const f = await el.getAttribute('data-fig'); const box = await el.$('.svgbox');
    const buf = await box.screenshot({ type: 'png' }); const bb = await box.boundingBox();
    figs[f] = { buf, w: bb.width, h: bb.height };
  }
  await browser.close();

  const FONT = 'Calibri', MONO = 'Consolas';
  function runs(text, opts = {}) {
    const out = []; const re = /`([^`]+)`|\*\*([^*]+)\*\*|\*([^*]+)\*|\[([^\]]+)\]\((https?:[^)]+)\)/g; let i = 0, m;
    while ((m = re.exec(text))) {
      if (m.index > i) out.push(new TextRun({ text: text.slice(i, m.index), font: FONT, ...opts }));
      if (m[1]) out.push(new TextRun({ text: m[1], font: MONO, size: 19, ...opts }));
      else if (m[2]) out.push(new TextRun({ text: m[2], bold: true, font: FONT, ...opts }));
      else if (m[3]) out.push(new TextRun({ text: m[3], italics: true, font: FONT, ...opts }));
      else out.push(new ExternalHyperlink({ link: m[5], children: [new TextRun({ text: m[4], style: 'Hyperlink', font: FONT, ...opts })] }));
      i = re.lastIndex;
    }
    if (i < text.length) out.push(new TextRun({ text: text.slice(i), font: FONT, ...opts }));
    return out;
  }
  const cap = t => new Paragraph({ spacing: { before: 60, after: 200 }, children: runs(t, { italics: true, size: 19, color: '4B5563' }) });
  const children = []; let listN = 0; const numbering = [];
  const contentW = 9360; // 6.5 in at 1440 dxa
  for (const b of doc) {
    if (b.t === 'title') children.push(new Paragraph({ heading: HeadingLevel.TITLE, children: [new TextRun({ text: b.x, font: FONT, bold: true, size: 52, color: '1E3A5F' })], spacing: { after: 120 } }));
    else if (b.t === 'by') children.push(new Paragraph({ children: runs(b.x, { color: '6B7280', size: 20 }), spacing: { after: 240 } }));
    else if (b.t === 'h2') children.push(new Paragraph({ heading: HeadingLevel.HEADING_1, children: [new TextRun({ text: b.x, font: FONT, bold: true, size: 32, color: '1E3A5F' })], spacing: { before: 360, after: 120 } }));
    else if (b.t === 'h3') children.push(new Paragraph({ heading: HeadingLevel.HEADING_2, children: [new TextRun({ text: b.x, font: FONT, bold: true, size: 26, color: '1E3A5F' })] }));
    else if (b.t === 'lede') children.push(new Paragraph({ children: runs(b.x, { size: 26 }), spacing: { after: 160 } }));
    else if (b.t === 'p') children.push(new Paragraph({ children: runs(b.x, { size: 22 }), spacing: { after: 140, line: 300 } }));
    else if (b.t === 'list') {
      const ref = 'bullets' + (++listN);
      numbering.push({ reference: ref, levels: [{ level: 0, format: LevelFormat.BULLET, text: '•', alignment: AlignmentType.LEFT, style: { paragraph: { indent: { left: 540, hanging: 270 } } } }] });
      for (const it of b.x) children.push(new Paragraph({ numbering: { reference: ref, level: 0 }, children: runs(it, { size: 22 }), spacing: { after: 80 } }));
    } else if (b.t === 'code') {
      for (const line of b.x.split('\n')) children.push(new Paragraph({ shading: { type: ShadingType.CLEAR, color: 'auto', fill: 'F3F4F6' }, spacing: { after: 0, line: 240 },
        children: [new TextRun({ text: line.length ? line : ' ', font: MONO, size: 16 })] }));
      if (b.cap) children.push(cap(b.cap)); else children.push(new Paragraph({ text: '' }));
    } else if (b.t === 'table') {
      const n = b.head.length, wt = b.w || Array(n).fill(1), sum = wt.reduce((p, q) => p + q, 0);
      const widths = wt.map(x => Math.floor(contentW * x / sum)), total = widths.reduce((p, q) => p + q, 0);
      const cell = (t, head, i) => new TableCell({ width: { size: widths[i], type: WidthType.DXA }, shading: head ? { type: ShadingType.CLEAR, color: 'auto', fill: 'E5EDF7' } : undefined,
        margins: { top: 60, bottom: 60, left: 100, right: 100 }, children: [new Paragraph({ children: runs(String(t), { size: 20, bold: !!head }) })] });
      children.push(new Table({ columnWidths: widths, width: { size: total, type: WidthType.DXA },
        rows: [new TableRow({ tableHeader: true, children: b.head.map((h, i) => cell(h, true, i)) }), ...b.rows.map(r => new TableRow({ children: r.map((c, i) => cell(c, false, i)) }))] }));
      if (b.cap) children.push(cap(b.cap));
    } else if (b.t === 'fig') {
      const f = figs[b.f]; if (!f) continue;
      const wPx = 624, hPx = Math.round(f.h * wPx / f.w); // 6.5 in at 96 px/in
      children.push(new Paragraph({ alignment: AlignmentType.CENTER, spacing: { before: 160 }, children: [new ImageRun({ type: 'png', data: f.buf, transformation: { width: wPx, height: hPx } })] }));
      children.push(cap(b.cap));
    }
  }
  const document = new Document({
    creator: 'SWMM Enablement (Robert Dickinson), with Claude', title: 'The Engine in the Box', description: 'swmm5-rust as a RustGIS plugin',
    styles: { default: { document: { run: { font: FONT, size: 22 } } } },
    numbering: { config: numbering },
    sections: [{ properties: { page: { size: { width: 12240, height: 15840 }, margin: { top: 1440, bottom: 1440, left: 1440, right: 1440 } } },
      footers: { default: new Footer({ children: [new Paragraph({ alignment: AlignmentType.CENTER, children: [new TextRun({ text: 'swmm5-rust in RustGIS · ', size: 16, color: '6B7280' }), new TextRun({ children: [PageNumber.CURRENT], size: 16, color: '6B7280' })] })] }) },
      children }],
  });
  fs.writeFileSync(out, await Packer.toBuffer(document));
  console.log(out, fs.statSync(out).size, 'bytes', Object.keys(figs).length, 'figures');
})().catch(e => { console.error(e); process.exit(1); });
