#!/bin/bash
# Native gates in sequence, on a frozen copy of the Rust binary:
#   1. the 221 smoke + coverage models
#   2. the 68 corpus models with FILLED_CIRCULAR conduits
#   3. the 3,759-model corpus set swmm5-cplus was gated on (every model under
#      125 KB plus a seeded sample of 72 larger ones), smallest first
# gate.py resumes from its .jsonl, so this can be restarted.
cd /home/claude/work
G=/home/claude/swmm5-rust/tools/gate.py
REF=/home/claude/swmm5-rust/ref/runswmm
RS=/home/claude/swmm5-rust/ref/runswmm-rs-gate
R=/home/claude/swmm5-rust/results
python3 $G $REF $RS corpus1729 $R/rs_221_v2.jsonl --list $R/wasm_list.txt --timeout 300 --jobs 2 > $R/rs_221_v2.log 2>&1
python3 $G $REF $RS corpus1729 $R/rs_filled68.jsonl --list $R/filled68.txt --timeout 300 --jobs 2 > $R/rs_filled68.log 2>&1
python3 $G $REF $RS corpus1729 $R/rs_corpus.jsonl --list $R/corpus_set.txt --timeout 300 --jobs 2 > $R/rs_corpus.log 2>&1
