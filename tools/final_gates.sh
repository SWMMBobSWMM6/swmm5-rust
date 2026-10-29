#!/bin/bash
# The rest of the native gates, resumable, on the frozen gated binary. One phase per call:
#   corpus     the 3,759-model corpus set (two jobs, 300 s cap)
#   interface  every gated model that SAVEs or USEs an interface file, again, one at a time
#              (run in parallel, two models can share one file in their folder)
#   head       the binary built at HEAD (ref/runswmm-rs-head), which differs from the gated one in how a
#              control rule reads a link's velocity (2dcf313) and in WASI-only code: the 221 set and every
#              other gated model with control rules, less the ten that hit the 300 s cap in the gated runs
#              (those run in the timeouts phase, which uses this same binary)
#   timeouts   every model that hit the 300 s cap, again, with the binary built at HEAD and a one-hour cap, two at a time on an
#              otherwise idle two-core machine (each run has a core to itself). IO-02_end-time-1e30
#              is left out: END_TIME 1e30 hours puts its end 4.2e28 days away, so EPA's own run does not end.
cd /home/claude/work
G=/home/claude/swmm5-rust/tools/gate.py
REF=/home/claude/swmm5-rust/ref/runswmm
RS=/home/claude/swmm5-rust/ref/runswmm-rs-gate
R=/home/claude/swmm5-rust/results
case "$1" in
corpus)
  python3 $G $REF $RS corpus1729 $R/rs_corpus.jsonl --list $R/corpus_set.txt --timeout 300 --jobs 2 >> $R/rs_corpus.log 2>&1 ;;
interface)
  python3 $G $REF $RS corpus1729 $R/rs_interface_serial.jsonl --list $R/interface_models.txt --timeout 1800 --jobs 1 > $R/rs_interface_serial.log 2>&1 ;;
head)
  python3 $G $REF /home/claude/swmm5-rust/ref/runswmm-rs-head corpus1729 $R/rs_head_check.jsonl --list $R/head_check.txt --timeout 1800 --jobs 2 > $R/rs_head_check.log 2>&1 ;;
timeouts)
  python3 - <<'PY'
import json
R = '/home/claude/swmm5-rust/results'
out = set()
for f in ('rs_221_v2.jsonl', 'rs_filled68.jsonl', 'rs_corpus.jsonl'):
    for l in open(f'{R}/{f}'):
        r = json.loads(l)
        if 'timeout' in (str(r.get('rc_ref')), str(r.get('rc_test'))) and 'IO-02_end-time-1e30' not in r['inp']:
            out.add('corpus1729/' + r['inp'])
open(f'{R}/timeouts.txt', 'w').write(''.join(m + '\n' for m in sorted(out)))
print(len(out), 'timeouts to rerun')
PY
  python3 $G $REF /home/claude/swmm5-rust/ref/runswmm-rs-head corpus1729 $R/rs_timeouts_rerun.jsonl --list $R/timeouts.txt --timeout 3600 --jobs 2 >> $R/rs_timeouts_rerun.log 2>&1 ;;
esac
echo "$1 done" >> $R/final_gates.log
