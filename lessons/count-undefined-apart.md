Where the reference engine's behaviour is undefined, there is no answer to match; count those models apart, never as matches or failures.

Six decks in the 221 set (IO-23, CRASH-02/03/04, CRASH-08 x2) make EPA read or write memory it does not own. EPA's answer changes with
the platform (native x86 vs WebAssembly) on three of them. `app/build.py` classifies them as `ub` from an explicit list; any other
difference is a real `diff`. Keep the list explicit so a new difference cannot hide in it.
