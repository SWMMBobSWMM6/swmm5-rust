#!/bin/bash
# Page, then the .docx from the page's own essay and figures, then the page again with the .docx in it.
set -e
cd /root/work/app
rm -f dist/swmm5-rust-in-rustgis.docx
python3 build.py dist/stage1.html
node gen_docx.js "$PWD/dist/stage1.html" "$PWD/dist/swmm5-rust-in-rustgis.docx"
python3 /mnt/skills/public/docx/scripts/office/validate.py dist/swmm5-rust-in-rustgis.docx | tail -1
python3 build.py dist/swmm5-rust-rustgis.html
rm -f dist/stage1.html
mkdir -p /root/work/docxcheck/final && rm -f /root/work/docxcheck/final/*
python3 /mnt/skills/public/docx/scripts/office/soffice.py --headless --convert-to pdf --outdir /root/work/docxcheck/final dist/swmm5-rust-in-rustgis.docx >/dev/null 2>&1
pdftoppm -jpeg -r 50 /root/work/docxcheck/final/swmm5-rust-in-rustgis.pdf /root/work/docxcheck/final/page
ls /root/work/docxcheck/final | wc -l
