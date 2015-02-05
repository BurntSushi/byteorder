This crate provides convenience methods for encoding and decoding numbers in
either big-endian or little-endian order. This is meant to replace the old
methods defined on the standard library `Reader` and `Writer` traits.

[![Build status](https://api.travis-ci.org/BurntSushi/byteorder.png)](https://travis-ci.org/BurntSushi/byteorder)

Licensed under the [UNLICENSE](http://unlicense.org).


### Documentation

[http://burntsushi.net/rustdoc/byteorder/](http://burntsushi.net/rustdoc/byteorder/).

The documentation includes examples.


### Installation

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/byteorder). The package is regularly
updated.  Add is to your `Cargo.toml` like so:

```toml
[dependencies]
byteorder = "*"
```

If you want to augment existing `Reader` and `Writer` types, then import the
extension methods like so:

```rust
extern crate byteorder;

use byteorder::{ReaderBytesExt, WriterBytesExt, BigEndian, LittleEndian};
```

For example:

```rust
use std::old_io::MemReader;
use byteorder::{BigEndian, ReaderBytesExt};

let mut rdr = MemReader::new(vec![2, 5, 3, 0]);
// Note that we use type parameters to indicate which kind of byte order
// we want!
assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
```

