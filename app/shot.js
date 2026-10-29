// Headless check of the built page: boot, run, every tab, both themes, console errors.
//   node shot.js PAGE.html OUTDIR [width]
const { chromium } = require('playwright');
const path = require('path'), fs = require('fs');
(async () => {
  const [page_, outdir, w] = process.argv.slice(2);
  fs.mkdirSync(outdir, { recursive: true });
  const browser = await chromium.launch({ args: ['--disable-dev-shm-usage'] });
  const ctx = await browser.newContext({ viewport: { width: +(w || 1280), height: 900 } });
  const page = await ctx.newPage();
  const errors = [];
  page.on('console', m => { if (m.type() === 'error') errors.push(m.text()); });
  page.on('pageerror', e => errors.push('pageerror: ' + e.message));
  await page.goto('file://' + path.resolve(page_));
  await page.waitForFunction(() => document.querySelector('#runStatus').textContent === 'Done.', null, { timeout: 120000 });
  const verdict = await page.textContent('#verdictBig');
  console.log('verdict:', verdict, '|', (await page.textContent('#verdictLine')).slice(0, 160));
  await page.click('#welcomeClose').catch(() => {});
  await page.screenshot({ path: `${outdir}/run_dark.png`, fullPage: false });
  const res = { verdict };
  // every sample
  const n = await page.$$eval('#sampleSel option', o => o.length);
  for (let i = 0; i < n; i++) {
    await page.selectOption('#sampleSel', String(i));
    await page.waitForFunction(() => document.querySelector('#runStatus').textContent === 'Done.', null, { timeout: 300000 });
    const v = await page.textContent('#verdictBig');
    const nm = await page.$eval('#sampleSel', s => s.options[s.selectedIndex].textContent);
    console.log('sample', i, nm, '->', v);
  }
  await page.selectOption('#sampleSel', '0');
  await page.waitForFunction(() => document.querySelector('#runStatus').textContent === 'Done.', null, { timeout: 300000 });
  for (const v of ['many', 'inside', 'verify', 'about', 'doc']) {
    await page.click(`[data-view="${v}"]`);
    if (v === 'many') { await page.click('#manyBtn'); await page.waitForFunction(() => document.querySelector('#manyStatus').textContent === 'Done.' || /Failed/.test(document.querySelector('#manyStatus').textContent), null, { timeout: 600000 }); console.log('many:', await page.textContent('#manyFoot')); }
    if (v === 'inside') {
      await page.click('#storageBtn'); await page.waitForFunction(() => /Done|Failed/.test(document.querySelector('#storageStatus').textContent), null, { timeout: 120000 });
      console.log('storage:', (await page.textContent('#storageTable')).replace(/\s+/g, ' ').slice(0, 400));
      await page.click('#decksBtn'); await page.waitForFunction(() => /Done|Failed/.test(document.querySelector('#decksStatus').textContent), null, { timeout: 120000 });
      console.log('decks:', (await page.textContent('#decksTable')).replace(/\s+/g, ' ').slice(0, 600));
    }
    await page.waitForTimeout(300);
    await page.screenshot({ path: `${outdir}/${v}_dark.png`, fullPage: true });
  }
  await page.click('#theme-toggle button[data-t="light"]');
  for (const v of ['run', 'inside', 'verify', 'doc']) {
    await page.click(`[data-view="${v}"]`); await page.waitForTimeout(300);
    await page.screenshot({ path: `${outdir}/${v}_light.png`, fullPage: v !== 'doc' });
  }
  const sw = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  console.log('horizontal overflow px:', sw);
  console.log('console errors:', errors.length ? errors : 'none');
  await browser.close();
})();
