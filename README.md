Convenience functions for reading and writing integers/floats in various byte
orders such as big-endian and little-endian. This is meant to replace the old
methods defined on the standard library `Reader` and `Writer` traits.

Work in progress.


TODO
====
1. `f32` and `f64` support. (trivial)
2. Flesh out the README. (Install, examples, links to docs, limitations.)


Ideas?
======
Use the `rustc-serialize` infrastructure, but it is known to be Not Fast. So
I'm skeptical of how useful it would be. Basically, it would let you say
something like: `let n: u32 = rdr.decode::<BigEndian>()` as opposed to
`let n = rdr.read_u32::<BigEndian>()`. Doesn't seem like an obvious win.
