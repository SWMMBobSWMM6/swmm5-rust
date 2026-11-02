// Headless check of the 1.1 page: the reviewer's five fixes and the additions, both themes, phone width.
//   node test_page.js page.html outdir
const { chromium } = require('/opt/npm-tools/node_modules/playwright');
const fs = require('fs'), path = require('path');
(async () => {
  const [file, out] = process.argv.slice(2); fs.mkdirSync(out, { recursive: true });
  const browser = await chromium.launch({ executablePath: '/opt/pw-browsers/chromium-1243/chrome-linux64/chrome' }).catch(() => chromium.launch());
  const ctx = await browser.newContext({ viewport: { width: 1360, height: 950 }, acceptDownloads: true });
  const page = await ctx.newPage(); const errs = [], R = {};
  page.on('console', m => { if ((m.type() === 'error' || m.type() === 'warning') && !/ERR_TUNNEL|fonts\.g/.test(m.text())) errs.push(m.type() + ': ' + m.text()); });
  page.on('pageerror', e => errs.push('pageerror: ' + e.message));
  await page.goto('file://' + file);
  const S = () => page.evaluate(() => { const s = window.__app.S; return { run: s.run && { id: s.run.id, model: s.run.bundle.model.name, err: s.run.reply && s.run.reply.error, series: !!s.run.series, same: s.run.series && s.run.series.same }, pending: !!s.pending }; });
  const until = (f, arg, t) => page.waitForFunction(f, arg, { timeout: t || 120000 });
  await until(() => window.__app && window.__app.S.run && !window.__app.S.pending);
  // keyboard: the help dialog opens on first visit with focus inside; Escape closes it
  R.helpFocus = await page.evaluate(() => document.activeElement && document.activeElement.id);
  await page.keyboard.press('Escape'); R.helpClosed = await page.evaluate(() => document.getElementById('help').hidden);
  R.first = await S(); R.runline = (await page.textContent('#runline')).replace(/\s+/g, ' ');
  await until(() => window.__app.S.run.series, null, 60000);
  R.summary = (await page.textContent('#runsum')).replace(/\s+/g, ' ');
  // hydrograph of node 9, then the time slider at the middle of the run
  await page.evaluate(() => window.__app.findId('9'));
  R.chart = await page.evaluate(() => !!document.querySelector('#tschart svg path'));
  R.chartText = (await page.textContent('#tschart')).replace(/\s+/g, ' ').slice(0, 220);
  await page.evaluate(() => { const sl = document.getElementById('tslider'); sl.value = Math.floor(sl.max / 2); sl.dispatchEvent(new Event('input')); });
  R.legend = (await page.textContent('#legend')).replace(/\s+/g, ' ').slice(0, 120);
  R.tsbox = (await page.textContent('#tsbody .box')).replace(/\s+/g, ' ').slice(0, 200);
  await page.screenshot({ path: out + '/map-time-dark.png' });
  // Verify, with the self-test
  await page.click('#t-verify'); await page.click('#verifybtn');
  await until(() => /identical|differ|stopped|did not finish/.test(document.getElementById('verifyout').textContent));
  R.verify = (await page.textContent('#verifyout')).replace(/\s+/g, ' ').slice(0, 700);
  await page.screenshot({ path: out + '/verify-dark.png', fullPage: true });
  // a superseded run: start Greenville, then switch to the pyswmm example at once
  await page.click('#t-map');
  await page.selectOption('#demo', '5'); await page.waitForTimeout(150); await page.selectOption('#demo', '1');
  await until(() => window.__app.S.run && !window.__app.S.pending && window.__app.S.run.bundle.model.name === 'swmm_example.inp');
  await page.waitForTimeout(1500);
  R.superseded = await S(); R.supersededRunline = (await page.textContent('#runline')).replace(/\s+/g, ' ');
  // cancel: start Greenville and cancel it
  await page.selectOption('#demo', '5'); await page.waitForTimeout(300); await page.click('#cancelbtn');
  R.cancelled = (await page.textContent('#runline')).replace(/\s+/g, ' '); R.afterCancel = await S();
  // the reviewer's external-rainfall case: the .inp names C:\Users\bob\storms\rain_ts1.dat
  await page.setInputFiles('#datafiles', path.join(__dirname, 'fixtures/rain_ts1.dat'));
  await page.setInputFiles('#file', path.join(__dirname, 'fixtures/swmm_example_rainfile.inp'));
  await until(() => window.__app.S.run && !window.__app.S.pending && window.__app.S.run.bundle.model.name === 'swmm_example_rainfile.inp');
  R.rainRun = await S(); R.rainMsgs = await page.evaluate(() => window.__app.S.messages.filter(m => /asked for|data files/.test(m)));
  await page.click('#t-verify'); await page.click('#verifybtn');
  await until(() => /identical|differ|stopped|did not finish/.test(document.getElementById('verifyout').textContent));
  R.rainVerify = (await page.textContent('#verifyout')).replace(/\s+/g, ' ').slice(0, 520);
  // the run bundle
  await page.click('#t-map');
  const [dl] = await Promise.all([page.waitForEvent('download'), page.evaluate(() => window.__app.X.bundle())]);
  const zp = out + '/' + dl.suggestedFilename(); await dl.saveAs(zp); R.bundle = zp;
  // a big layer: the 17,100-element deck mapped (Map only), the table drawing only what is in view
  await page.setInputFiles('#datafiles', []);
  await page.setInputFiles('#file', '/root/work/corpus/EPA/17100_H&H_Dummy_Link_Elements(2).inp');
  await page.waitForTimeout(400); await page.click('#mapbtn');
  await until(() => window.__app.S.run && !window.__app.S.pending && window.__app.S.run.tool === 'map-swmm', null, 180000);
  await page.evaluate(() => { const s = window.__app.S; s.active = (s.layers.find(l => l.kind === 'links') || s.layers[0]).name; });
  await page.click('#layers li:nth-child(2)');
  R.big = await page.evaluate(() => ({ layer: document.getElementById('tabname').textContent, rows: document.getElementById('tabcount').textContent, drawn: document.querySelectorAll('#attr tbody tr[data-i]').length }));
  await page.evaluate(() => { const sc = document.getElementById('tablescroll'); sc.scrollTop = sc.scrollHeight; sc.dispatchEvent(new Event('scroll')); });
  await page.waitForTimeout(200);
  R.bigBottom = await page.evaluate(() => { const r = [...document.querySelectorAll('#attr tbody tr[data-i]')]; return r.length ? r[r.length - 1].dataset.i : null; });
  await page.fill('#tabfilter', 'zz-no-such'); R.filtered = await page.textContent('#tabcount'); await page.fill('#tabfilter', '');
  // tabs by keyboard
  await page.focus('#t-map'); await page.keyboard.press('ArrowRight'); R.tabKey = await page.evaluate(() => document.querySelector('#tabs [aria-selected="true"]').id);
  // light theme, Doc, phone width on every tab
  await page.click('#theme-toggle button[data-t="light"]'); await page.click('#t-map');
  await page.selectOption('#demo', '0'); await until(() => window.__app.S.run && !window.__app.S.pending && window.__app.S.run.bundle.model.name === 'jason_dorn_storage_example.inp');
  await until(() => window.__app.S.run.series, null, 60000); await page.evaluate(() => window.__app.findId('9'));
  await page.screenshot({ path: out + '/map-light.png' });
  await page.setViewportSize({ width: 390, height: 844 }); await page.waitForTimeout(300);
  R.overflow = {};
  for (const v of ['map', 'plugin', 'verify', 'about', 'doc']) { await page.click('#t-' + v); await page.waitForTimeout(150); R.overflow[v] = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth); }
  await page.click('#t-map'); await page.screenshot({ path: out + '/phone-map.png', fullPage: true });
  R.console = errs;
  console.log(JSON.stringify(R, null, 1));
  await browser.close();
})().catch(e => { console.error(e); process.exit(1); });
