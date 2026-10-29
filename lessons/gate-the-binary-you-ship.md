Gate the binary you ship, not the one you gated last week: any engine change after the gate needs its own run, however harmless the argument for it.

The native corpus gate ran a binary frozen at commit d37bbc5. Two commits later a control rule's LINK VELOCITY read was changed
to call the same mutable function C calls, replacing a read-only copy that had been justified by an argument ("the one-bit edit
cannot matter by then"). The WebAssembly gate ran the new engine, but the native results described the old one, and the results
README said later commits touched only WASI code, which was no longer true. The fix: `tools/final_gates.sh head` runs the binary
built at HEAD on the 221 set and every gated model with control rules, and the README states which binary each gate ran.
