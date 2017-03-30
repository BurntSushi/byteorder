/*!
This crate provides convenience methods for encoding and decoding numbers
in either big-endian or little-endian order.

The organization of the crate is pretty simple. A trait, `ByteOrder`, specifies
byte conversion methods for each type of number in Rust (sans numbers that have
a platform dependent size like `usize` and `isize`). Two types, `BigEndian`
and `LittleEndian` implement these methods. Finally, `ReadBytesExt` and
`WriteBytesExt` provide convenience methods available to all types that
implement `Read` and `Write`.

# Examples

Read unsigned 16 bit big-endian integers from a `Read` type:

```rust
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
// Note that we use type parameters to indicate which kind of byte order
// we want!
assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
```

Write unsigned 16 bit little-endian integers to a `Write` type:

```rust
use byteorder::{LittleEndian, WriteBytesExt};

let mut wtr = vec![];
wtr.write_u16::<LittleEndian>(517).unwrap();
wtr.write_u16::<LittleEndian>(768).unwrap();
assert_eq!(wtr, vec![5, 2, 0, 3]);
```
*/

#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "i128", feature(i128_type))]
#![cfg_attr(all(feature = "i128", test), feature(i128))]
#![doc(html_root_url = "https://docs.rs/byteorder/1.0.0")]

#[cfg(feature = "std")]
extern crate core;

use core::fmt::Debug;
use core::hash::Hash;
use core::mem::transmute;
use core::ptr::copy_nonoverlapping;

#[cfg(feature = "std")]
pub use new::{ReadBytesExt, WriteBytesExt};

#[cfg(feature = "std")]
mod new;

#[inline]
fn extend_sign(val: u64, nbytes: usize) -> i64 {
    let shift = (8 - nbytes) * 8;
    (val << shift) as i64 >> shift
}

#[cfg(feature = "i128")]
#[inline]
fn extend_sign128(val: u128, nbytes: usize) -> i128 {
    let shift = (16 - nbytes) * 8;
    (val << shift) as i128 >> shift
}

#[inline]
fn unextend_sign(val: i64, nbytes: usize) -> u64 {
    let shift = (8 - nbytes) * 8;
    (val << shift) as u64 >> shift
}

#[cfg(feature = "i128")]
#[inline]
fn unextend_sign128(val: i128, nbytes: usize) -> u128 {
    let shift = (16 - nbytes) * 8;
    (val << shift) as u128 >> shift
}

#[inline]
fn pack_size(n: u64) -> usize {
    if n < 1 << 8 {
        1
    } else if n < 1 << 16 {
        2
    } else if n < 1 << 24 {
        3
    } else if n < 1 << 32 {
        4
    } else if n < 1 << 40 {
        5
    } else if n < 1 << 48 {
        6
    } else if n < 1 << 56 {
        7
    } else {
        8
    }
}

#[cfg(feature = "i128")]
#[inline]
fn pack_size128(n: u128) -> usize {
    if n < 1 << 8 {
        1
    } else if n < 1 << 16 {
        2
    } else if n < 1 << 24 {
        3
    } else if n < 1 << 32 {
        4
    } else if n < 1 << 40 {
        5
    } else if n < 1 << 48 {
        6
    } else if n < 1 << 56 {
        7
    } else if n < 1 << 64 {
        8
    } else if n < 1 << 72 {
        9
    } else if n < 1 << 80 {
        10
    } else if n < 1 << 88 {
        11
    } else if n < 1 << 96 {
        12
    } else if n < 1 << 104 {
        13
    } else if n < 1 << 112 {
        14
    } else if n < 1 << 120 {
        15
    } else {
        16
    }
}

mod private {
    /// Sealed stops crates other than byteorder from implementing any traits that use it.
    pub trait Sealed{}
    impl Sealed for super::LittleEndian {}
    impl Sealed for super::BigEndian {}
}

/// ByteOrder describes types that can serialize integers as bytes.
///
/// Note that `Self` does not appear anywhere in this trait's definition!
/// Therefore, in order to use it, you'll need to use syntax like
/// `T::read_u16(&[0, 1])` where `T` implements `ByteOrder`.
///
/// This crate provides two types that implement `ByteOrder`: `BigEndian`
/// and `LittleEndian`.
/// This trait is sealed and cannot be implemented for callers to avoid
/// breaking backwards compatibility when adding new derived traits.
///
/// # Examples
///
/// Write and read `u32` numbers in little endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, LittleEndian};
///
/// let mut buf = [0; 4];
/// LittleEndian::write_u32(&mut buf, 1_000_000);
/// assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
/// ```
///
/// Write and read `i16` numbers in big endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, BigEndian};
///
/// let mut buf = [0; 2];
/// BigEndian::write_i16(&mut buf, -50_000);
/// assert_eq!(-50_000, BigEndian::read_i16(&buf));
/// ```
pub trait ByteOrder
    : Clone + Copy + Debug + Default + Eq + Hash + Ord + PartialEq + PartialOrd + private::Sealed {
    /// Reads an unsigned 16 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 2`.
    fn read_u16(buf: &[u8]) -> u16;

    /// Reads an unsigned 32 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    ///
    /// # Examples
    ///
    /// Write and read `u32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 4];
    /// LittleEndian::write_u32(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
    /// ```
    fn read_u32(buf: &[u8]) -> u32;

    /// Reads an unsigned 64 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    ///
    /// # Examples
    ///
    /// Write and read `u64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 8];
    /// LittleEndian::write_u64(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u64(&buf));
    /// ```
    fn read_u64(buf: &[u8]) -> u64;

    /// Reads an unsigned 128 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 16`.
    ///
    /// # Examples
    ///
    /// Write and read `u128` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 16];
    /// LittleEndian::write_u128(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u128(&buf));
    /// ```
    #[cfg(feature = "i128")]
    fn read_u128(buf: &[u8]) -> u128;

    /// Reads an unsigned n-bytes integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `nbytes < 1` or `nbytes > 8` or
    /// `buf.len() < nbytes`
    ///
    /// # Examples
    ///
    /// Write and read an n-byte number in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_uint(&mut buf, 1_000_000, 3);
    /// assert_eq!(1_000_000, LittleEndian::read_uint(&buf, 3));
    /// ```
    fn read_uint(buf: &[u8], nbytes: usize) -> u64;

    /// Reads an unsigned n-bytes integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `nbytes < 1` or `nbytes > 16` or
    /// `buf.len() < nbytes`
    ///
    /// # Examples
    ///
    /// Write and read an n-byte number in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_uint128(&mut buf, 1_000_000, 3);
    /// assert_eq!(1_000_000, LittleEndian::read_uint128(&buf, 3));
    /// ```
    #[cfg(feature = "i128")]
    fn read_uint128(buf: &[u8], nbytes: usize) -> u128;

    /// Writes an unsigned 16 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 2`.
    ///
    /// # Examples
    ///
    /// Write and read `u16` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 2];
    /// LittleEndian::write_u16(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u16(&buf));
    /// ```
    fn write_u16(buf: &mut [u8], n: u16);

    /// Writes an unsigned 32 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    ///
    /// # Examples
    ///
    /// Write and read `u32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 4];
    /// LittleEndian::write_u32(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
    /// ```
    fn write_u32(buf: &mut [u8], n: u32);

    /// Writes an unsigned 64 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    ///
    /// # Examples
    ///
    /// Write and read `u64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 8];
    /// LittleEndian::write_u64(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u64(&buf));
    /// ```
    fn write_u64(buf: &mut [u8], n: u64);

    /// Writes an unsigned 128 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 16`.
    #[cfg(feature = "i128")]
    fn write_u128(buf: &mut [u8], n: u128);

    /// Writes an unsigned integer `n` to `buf` using only `nbytes`.
    ///
    /// # Panics
    ///
    /// If `n` is not representable in `nbytes`, or if `nbytes` is `> 8`, then
    /// this method panics.
    ///
    /// # Examples
    ///
    /// Write and read an n-byte number in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_uint(&mut buf, 1_000_000, 3);
    /// assert_eq!(1_000_000, LittleEndian::read_uint(&buf, 3));
    /// ```
    fn write_uint(buf: &mut [u8], n: u64, nbytes: usize);

    /// Writes an unsigned integer `n` to `buf` using only `nbytes`.
    ///
    /// # Panics
    ///
    /// If `n` is not representable in `nbytes`, or if `nbytes` is `> 16`, then
    /// this method panics.
    #[cfg(feature = "i128")]
    fn write_uint128(buf: &mut [u8], n: u128, nbytes: usize);

    /// Reads the first nbytes of a IEEE754 double-precision (8 bytes) floating point number and
    /// assumes the rest are zero.
    ///
    /// This is useful for formats which serialize floats as little-endian integers and elid any
    /// trailing zeros in the low bits to save space.
    /// The return value of read_float is always defined; signaling NaN's are turned into quiet
    /// NaN's.
    ///
    /// # Panics
    ///
    /// If `nbytes < 1` or `nbytes > 8` or `buf.len() < nbytes`
    ///
    /// # Examples
    ///
    /// Read 2 bytes into a double-precision float:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let buf = b"\xf0\x3f";
    /// assert_eq!(1.0, LittleEndian::read_float(buf, buf.len()));
    /// ```
    fn read_float(buf: &[u8], nbytes: usize) -> f64;

    /// Reads a signed 16 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 2`.
    ///
    /// # Examples
    ///
    /// Write and read `u16` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 2];
    /// LittleEndian::write_i16(&mut buf, -1_000);
    /// assert_eq!(-1_000, LittleEndian::read_i16(&buf));
    /// ```
    #[inline]
    fn read_i16(buf: &[u8]) -> i16 {
        Self::read_u16(buf) as i16
    }

    /// Reads a signed 32 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    ///
    /// # Examples
    ///
    /// Write and read `u32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 4];
    /// LittleEndian::write_i32(&mut buf, -1_000_000);
    /// assert_eq!(-1_000_000, LittleEndian::read_i32(&buf));
    /// ```
    #[inline]
    fn read_i32(buf: &[u8]) -> i32 {
        Self::read_u32(buf) as i32
    }

    /// Reads a signed 64 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    ///
    /// # Examples
    ///
    /// Write and read `u64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 8];
    /// LittleEndian::write_i64(&mut buf, -1_000_000_000);
    /// assert_eq!(-1_000_000_000, LittleEndian::read_i64(&buf));
    /// ```
    #[inline]
    fn read_i64(buf: &[u8]) -> i64 {
        Self::read_u64(buf) as i64
    }

    /// Reads a signed 128 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 16`.
    #[cfg(feature = "i128")]
    #[inline]
    fn read_i128(buf: &[u8]) -> i128 {
        Self::read_u128(buf) as i128
    }

    /// Reads a signed n-bytes integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `nbytes < 1` or `nbytes > 8` or
    /// `buf.len() < nbytes`
    ///
    /// # Examples
    ///
    /// Write and read n-length signed numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_int(&mut buf, -1_000, 3);
    /// assert_eq!(-1_000, LittleEndian::read_int(&buf, 3));
    /// ```
    #[inline]
    fn read_int(buf: &[u8], nbytes: usize) -> i64 {
        extend_sign(Self::read_uint(buf, nbytes), nbytes)
    }

    /// Reads a signed n-bytes integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `nbytes < 1` or `nbytes > 16` or
    /// `buf.len() < nbytes`
    #[cfg(feature = "i128")]
    #[inline]
    fn read_int128(buf: &[u8], nbytes: usize) -> i128 {
        extend_sign128(Self::read_uint128(buf, nbytes), nbytes)
    }

    /// Reads a IEEE754 single-precision (4 bytes) floating point number.
    ///
    /// The return value of is always defined; signaling NaN's are turned into quiet NaN's.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    ///
    /// # Examples
    ///
    /// Write and read `f32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let e = 2.71828;
    /// let mut buf = [0; 4];
    /// LittleEndian::write_f32(&mut buf, e);
    /// assert_eq!(e, LittleEndian::read_f32(&buf));
    /// ```
    #[inline]
    fn read_f32(buf: &[u8]) -> f32 {
        let mut u = Self::read_u32(buf);
        // The exponent is 1's  &&  the mantissa has at least one bit set (aka. is_nan):
        if (u & 0xFF<<23 == 0xFF<<23) && (u & 0x3FFFFF != 0) {
            u |= 1<<22;
        }
        unsafe { transmute(u) }
    }

    /// Reads a IEEE754 double-precision (8 bytes) floating point number.
    ///
    /// The return value of is always defined; signaling NaN's are turned into quiet NaN's.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    ///
    /// # Examples
    ///
    /// Write and read `f64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let phi = 1.6180339887;
    /// let mut buf = [0; 8];
    /// LittleEndian::write_f64(&mut buf, phi);
    /// assert_eq!(phi, LittleEndian::read_f64(&buf));
    /// ```
    #[inline]
    fn read_f64(buf: &[u8]) -> f64 {
        let mut u = Self::read_u64(buf);
        // The exponent is 1's  &&  the mantissa has at least one bit set (aka. is_nan):
        if (u & 0x7FF<<52 == 0x7FF<<52) && (u & 0x000FFFFFFFFFFFFF != 0) {
            u |= 1<<51;
        }
        unsafe { transmute(u) }
    }

    /// Writes a signed 16 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 2`.
    ///
    /// # Examples
    ///
    /// Write and read `u16` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 2];
    /// LittleEndian::write_i16(&mut buf, -1_000);
    /// assert_eq!(-1_000, LittleEndian::read_i16(&buf));
    /// ```
    #[inline]
    fn write_i16(buf: &mut [u8], n: i16) {
        Self::write_u16(buf, n as u16)
    }

    /// Writes a signed 32 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    ///
    /// # Examples
    ///
    /// Write and read `u32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 4];
    /// LittleEndian::write_i32(&mut buf, -1_000_000);
    /// assert_eq!(-1_000_000, LittleEndian::read_i32(&buf));
    /// ```
    #[inline]
    fn write_i32(buf: &mut [u8], n: i32) {
        Self::write_u32(buf, n as u32)
    }

    /// Writes a signed 64 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    ///
    /// # Examples
    ///
    /// Write and read `u64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 8];
    /// LittleEndian::write_i64(&mut buf, -1_000_000_000);
    /// assert_eq!(-1_000_000_000, LittleEndian::read_i64(&buf));
    /// ```
    #[inline]
    fn write_i64(buf: &mut [u8], n: i64) {
        Self::write_u64(buf, n as u64)
    }

    /// Writes a signed 128 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 16`.
    #[cfg(feature = "i128")]
    #[inline]
    fn write_i128(buf: &mut [u8], n: i128) {
        Self::write_u128(buf, n as u128)
    }

    /// Writes a signed integer `n` to `buf` using only `nbytes`.
    ///
    /// # Panics
    ///
    /// If `n` is not representable in `nbytes`, or if `nbytes` is `> 8`, then
    /// this method panics.
    ///
    /// # Examples
    ///
    /// Write and read an n-byte number in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_int(&mut buf, -1_000, 3);
    /// assert_eq!(-1_000, LittleEndian::read_int(&buf, 3));
    /// ```
    #[inline]
    fn write_int(buf: &mut [u8], n: i64, nbytes: usize) {
        Self::write_uint(buf, unextend_sign(n, nbytes), nbytes)
    }

    /// Writes a signed integer `n` to `buf` using only `nbytes`.
    ///
    /// # Panics
    ///
    /// If `n` is not representable in `nbytes`, or if `nbytes` is `> 16`, then
    /// this method panics.
    #[cfg(feature = "i128")]
    #[inline]
    fn write_int128(buf: &mut [u8], n: i128, nbytes: usize) {
        Self::write_uint128(buf, unextend_sign128(n, nbytes), nbytes)
    }

    /// Writes a IEEE754 single-precision (4 bytes) floating point number.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    ///
    /// # Examples
    ///
    /// Write and read `f32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let e = 2.71828;
    /// let mut buf = [0; 4];
    /// LittleEndian::write_f32(&mut buf, e);
    /// assert_eq!(e, LittleEndian::read_f32(&buf));
    /// ```
    #[inline]
    fn write_f32(buf: &mut [u8], n: f32) {
        Self::write_u32(buf, unsafe { transmute(n) })
    }

    /// Writes a IEEE754 double-precision (8 bytes) floating point number.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    ///
    /// # Examples
    ///
    /// Write and read `f64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let phi = 1.6180339887;
    /// let mut buf = [0; 8];
    /// LittleEndian::write_f64(&mut buf, phi);
    /// assert_eq!(phi, LittleEndian::read_f64(&buf));
    /// ```
    #[inline]
    fn write_f64(buf: &mut [u8], n: f64) {
        Self::write_u64(buf, unsafe { transmute(n) })
    }
}

/// Defines big-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// # Examples
///
/// Write and read `u32` numbers in big endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, BigEndian};
///
/// let mut buf = [0; 4];
/// BigEndian::write_u32(&mut buf, 1_000_000);
/// assert_eq!(1_000_000, BigEndian::read_u32(&buf));
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BigEndian {}

impl Default for BigEndian {
    fn default() -> BigEndian {
        panic!("BigEndian default")
    }
}

/// Defines little-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// # Examples
///
/// Write and read `u32` numbers in little endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, LittleEndian};
///
/// let mut buf = [0; 4];
/// LittleEndian::write_u32(&mut buf, 1_000_000);
/// assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LittleEndian {}

impl Default for LittleEndian {
    fn default() -> LittleEndian {
        panic!("LittleEndian default")
    }
}

/// Defines network byte order serialization.
///
/// Network byte order is defined by [RFC 1700][1] to be big-endian, and is
/// referred to in several protocol specifications.  This type is an alias of
/// BigEndian.
///
/// [1]: https://tools.ietf.org/html/rfc1700
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// # Examples
///
/// Write and read `i16` numbers in big endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, NetworkEndian, BigEndian};
///
/// let mut buf = [0; 2];
/// BigEndian::write_i16(&mut buf, -50_000);
/// assert_eq!(-50_000, NetworkEndian::read_i16(&buf));
/// ```
pub type NetworkEndian = BigEndian;

/// Defines system native-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
#[cfg(target_endian = "little")]
pub type NativeEndian = LittleEndian;

/// Defines system native-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
#[cfg(target_endian = "big")]
pub type NativeEndian = BigEndian;

macro_rules! read_num_bytes {
    ($ty:ty, $size:expr, $src:expr, $which:ident) => ({
        assert!($size == ::core::mem::size_of::<$ty>());
        assert!($size <= $src.len());
        let mut data: $ty = 0;
        unsafe {
            copy_nonoverlapping(
                $src.as_ptr(),
                &mut data as *mut $ty as *mut u8,
                $size);
        }
        data.$which()
    });
}

macro_rules! write_num_bytes {
    ($ty:ty, $size:expr, $n:expr, $dst:expr, $which:ident) => ({
        assert!($size <= $dst.len());
        unsafe {
            // N.B. https://github.com/rust-lang/rust/issues/22776
            let bytes = transmute::<_, [u8; $size]>($n.$which());
            copy_nonoverlapping((&bytes).as_ptr(), $dst.as_mut_ptr(), $size);
        }
    });
}

impl ByteOrder for BigEndian {
    #[inline]
    fn read_u16(buf: &[u8]) -> u16 {
        read_num_bytes!(u16, 2, buf, to_be)
    }

    #[inline]
    fn read_u32(buf: &[u8]) -> u32 {
        read_num_bytes!(u32, 4, buf, to_be)
    }

    #[inline]
    fn read_u64(buf: &[u8]) -> u64 {
        read_num_bytes!(u64, 8, buf, to_be)
    }

    #[cfg(feature = "i128")]
    #[inline]
    fn read_u128(buf: &[u8]) -> u128 {
        read_num_bytes!(u128, 16, buf, to_be)
    }

    #[inline]
    fn read_uint(buf: &[u8], nbytes: usize) -> u64 {
        assert!(1 <= nbytes && nbytes <= 8 && nbytes <= buf.len());
        let mut out = [0u8; 8];
        let ptr_out = out.as_mut_ptr();
        unsafe {
            copy_nonoverlapping(
                buf.as_ptr(), ptr_out.offset((8 - nbytes) as isize), nbytes);
            (*(ptr_out as *const u64)).to_be()
        }
    }

    #[cfg(feature = "i128")]
    #[inline]
    fn read_uint128(buf: &[u8], nbytes: usize) -> u128 {
        assert!(1 <= nbytes && nbytes <= 16 && nbytes <= buf.len());
        let mut out = [0u8; 16];
        let ptr_out = out.as_mut_ptr();
        unsafe {
            copy_nonoverlapping(
                buf.as_ptr(), ptr_out.offset((16 - nbytes) as isize), nbytes);
            (*(ptr_out as *const u128)).to_be()
        }
    }

    #[inline]
    fn read_float(buf: &[u8], nbytes: usize) -> f64 {
        assert!(1 <= nbytes && nbytes <= 8 && nbytes <= buf.len());
        let mut out = [0; 8];
        let ptr_out = out.as_mut_ptr();
        unsafe {
            copy_nonoverlapping(buf.as_ptr(), ptr_out.offset((8 - nbytes) as isize), nbytes);
            if (out[0] == 0x7F || out[0] == 0xFF) && ((out[1] & 0x0F) | out[2] | out[3] | out[4] | out[5] | out[6] | out[7] != 0) {
                out[1] |= 0x08;
            }
            transmute((*(ptr_out as *const u64)).to_be())
        }
    }

    #[inline]
    fn write_u16(buf: &mut [u8], n: u16) {
        write_num_bytes!(u16, 2, n, buf, to_be);
    }

    #[inline]
    fn write_u32(buf: &mut [u8], n: u32) {
        write_num_bytes!(u32, 4, n, buf, to_be);
    }

    #[inline]
    fn write_u64(buf: &mut [u8], n: u64) {
        write_num_bytes!(u64, 8, n, buf, to_be);
    }

    #[cfg(feature = "i128")]
    #[inline]
    fn write_u128(buf: &mut [u8], n: u128) {
        write_num_bytes!(u128, 16, n, buf, to_be);
    }

    #[inline]
    fn write_uint(buf: &mut [u8], n: u64, nbytes: usize) {
        assert!(pack_size(n) <= nbytes && nbytes <= 8);
        assert!(nbytes <= buf.len());
        unsafe {
            let bytes: [u8; 8] = transmute(n.to_be());
            copy_nonoverlapping(
                bytes.as_ptr().offset((8 - nbytes) as isize),
                buf.as_mut_ptr(),
                nbytes);
        }
    }

    #[cfg(feature = "i128")]
    #[inline]
    fn write_uint128(buf: &mut [u8], n: u128, nbytes: usize) {
        assert!(pack_size128(n) <= nbytes && nbytes <= 16);
        assert!(nbytes <= buf.len());
        unsafe {
            let bytes: [u8; 16] = transmute(n.to_be());
            copy_nonoverlapping(
                bytes.as_ptr().offset((16 - nbytes) as isize),
                buf.as_mut_ptr(),
                nbytes);
        }
    }
}

impl ByteOrder for LittleEndian {
    #[inline]
    fn read_u16(buf: &[u8]) -> u16 {
        read_num_bytes!(u16, 2, buf, to_le)
    }

    #[inline]
    fn read_u32(buf: &[u8]) -> u32 {
        read_num_bytes!(u32, 4, buf, to_le)
    }

    #[inline]
    fn read_u64(buf: &[u8]) -> u64 {
        read_num_bytes!(u64, 8, buf, to_le)
    }

    #[cfg(feature = "i128")]
    #[inline]
    fn read_u128(buf: &[u8]) -> u128 {
        read_num_bytes!(u128, 16, buf, to_le)
    }

    #[inline]
    fn read_uint(buf: &[u8], nbytes: usize) -> u64 {
        assert!(1 <= nbytes && nbytes <= 8 && nbytes <= buf.len());
        let mut out = [0u8; 8];
        let ptr_out = out.as_mut_ptr();
        unsafe {
            copy_nonoverlapping(buf.as_ptr(), ptr_out, nbytes);
            (*(ptr_out as *const u64)).to_le()
        }
    }

    #[cfg(feature = "i128")]
    #[inline]
    fn read_uint128(buf: &[u8], nbytes: usize) -> u128 {
        assert!(1 <= nbytes && nbytes <= 16 && nbytes <= buf.len());
        let mut out = [0u8; 16];
        let ptr_out = out.as_mut_ptr();
        unsafe {
            copy_nonoverlapping(buf.as_ptr(), ptr_out, nbytes);
            (*(ptr_out as *const u128)).to_le()
        }
    }

    #[inline]
    fn read_float(buf: &[u8], nbytes: usize) -> f64 {
        assert!(1 <= nbytes && nbytes <= 8 && nbytes <= buf.len());
        let mut out = [0; 8];
        let ptr_out = out.as_mut_ptr();
        unsafe {
            copy_nonoverlapping(buf.as_ptr(), ptr_out.offset((8 - nbytes) as isize), nbytes);
            if (out[7] == 0x7F || out[7] == 0xFF) && ((out[6] & 0x0F) | out[5] | out[4] | out[3] | out[2] | out[1] | out[0] != 0) {
                out[6] |= 0x08;
            }
            transmute((*(ptr_out as *const u64)).to_le())
        }
    }

    #[inline]
    fn write_u16(buf: &mut [u8], n: u16) {
        write_num_bytes!(u16, 2, n, buf, to_le);
    }

    #[inline]
    fn write_u32(buf: &mut [u8], n: u32) {
        write_num_bytes!(u32, 4, n, buf, to_le);
    }

    #[inline]
    fn write_u64(buf: &mut [u8], n: u64) {
        write_num_bytes!(u64, 8, n, buf, to_le);
    }

    #[cfg(feature = "i128")]
    #[inline]
    fn write_u128(buf: &mut [u8], n: u128) {
        write_num_bytes!(u128, 16, n, buf, to_le);
    }

    #[inline]
    fn write_uint(buf: &mut [u8], n: u64, nbytes: usize) {
        assert!(pack_size(n as u64) <= nbytes && nbytes <= 8);
        assert!(nbytes <= buf.len());
        unsafe {
            let bytes: [u8; 8] = transmute(n.to_le());
            copy_nonoverlapping(bytes.as_ptr(), buf.as_mut_ptr(), nbytes);
        }
    }

    #[cfg(feature = "i128")]
    #[inline]
    fn write_uint128(buf: &mut [u8], n: u128, nbytes: usize) {
        assert!(pack_size128(n as u128) <= nbytes && nbytes <= 16);
        assert!(nbytes <= buf.len());
        unsafe {
            let bytes: [u8; 16] = transmute(n.to_le());
            copy_nonoverlapping(bytes.as_ptr(), buf.as_mut_ptr(), nbytes);
        }
    }
}

#[cfg(test)]
mod test {
    extern crate quickcheck;
    extern crate rand;

    use self::rand::thread_rng;
    use self::quickcheck::{QuickCheck, StdGen, Testable};
    #[cfg(feature = "i128")] use self::quickcheck::{ Arbitrary, Gen };

    pub const U64_MAX: u64 = ::core::u64::MAX;
    pub const I64_MAX: u64 = ::core::i64::MAX as u64;

    macro_rules! calc_max {
        ($max:expr, $bytes:expr) => { calc_max!($max, $bytes, 8) };
        ($max:expr, $bytes:expr, $maxbytes:expr) => {
            ($max - 1) >> (8 * ($maxbytes - $bytes))
        };
    }

    #[derive(Clone, Debug)]
    pub struct Wi128<T>(pub T);

    #[cfg(feature = "i128")]
    impl<T: Clone> Wi128<T> {
        pub fn clone(&self) -> T {
            self.0.clone()
        }
    }

    impl<T: PartialEq> PartialEq<T> for Wi128<T> {
        fn eq(&self, other: &T) -> bool {
            self.0.eq(other)
        }
    }

    #[cfg(feature = "i128")]
    impl Arbitrary for Wi128<u128> {
        fn arbitrary<G: Gen>(gen: &mut G) -> Wi128<u128> {
            let max = calc_max!(::core::u128::MAX, gen.size(), 16);
            let output =
                (gen.gen::<u64>() as u128) |
                ((gen.gen::<u64>() as u128) << 64);
            Wi128(output & (max - 1))
        }
    }

    #[cfg(feature = "i128")]
    impl Arbitrary for Wi128<i128> {
        fn arbitrary<G: Gen>(gen: &mut G) -> Wi128<i128> {
            let max = calc_max!(::core::i128::MAX, gen.size(), 16);
            let output =
                (gen.gen::<i64>() as i128) |
                ((gen.gen::<i64>() as i128) << 64);
            Wi128(output & (max - 1))
        }
    }

    pub fn qc_sized<A: Testable>(f: A, size: u64) {
        QuickCheck::new()
            .gen(StdGen::new(thread_rng(), size as usize))
            .tests(1_00)
            .max_tests(10_000)
            .quickcheck(f);
    }

    macro_rules! qc_byte_order {
        ($name:ident, $ty_int:ty, $max:expr,
         $bytes:expr, $read:ident, $write:ident) => (
            mod $name {
                use {BigEndian, ByteOrder, NativeEndian, LittleEndian};
                #[allow(unused_imports)] use super::{ qc_sized, Wi128 };

                #[test]
                fn big_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut buf = [0; 16];
                        BigEndian::$write(&mut buf, n.clone(), $bytes);
                        n == BigEndian::$read(&mut buf[..$bytes], $bytes)
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max);
                }

                #[test]
                fn little_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut buf = [0; 16];
                        LittleEndian::$write(&mut buf, n.clone(), $bytes);
                        n == LittleEndian::$read(&mut buf[..$bytes], $bytes)
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max);
                }

                #[test]
                fn native_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut buf = [0; 16];
                        NativeEndian::$write(&mut buf, n.clone(), $bytes);
                        n == NativeEndian::$read(&mut buf[..$bytes], $bytes)
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max);
                }
            }
        );
        ($name:ident, $ty_int:ty, $max:expr,
         $read:ident, $write:ident) => (
            mod $name {
                use core::mem::size_of;
                use {BigEndian, ByteOrder, NativeEndian, LittleEndian};
                #[allow(unused_imports)] use super::{ qc_sized, Wi128 };

                #[test]
                fn big_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let bytes = size_of::<$ty_int>();
                        let mut buf = [0; 16];
                        BigEndian::$write(&mut buf[16 - bytes..], n.clone());
                        n == BigEndian::$read(&mut buf[16 - bytes..])
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max - 1);
                }

                #[test]
                fn little_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let bytes = size_of::<$ty_int>();
                        let mut buf = [0; 16];
                        LittleEndian::$write(&mut buf[..bytes], n.clone());
                        n == LittleEndian::$read(&mut buf[..bytes])
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max - 1);
                }

                #[test]
                fn native_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let bytes = size_of::<$ty_int>();
                        let mut buf = [0; 16];
                        NativeEndian::$write(&mut buf[..bytes], n.clone());
                        n == NativeEndian::$read(&mut buf[..bytes])
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max - 1);
                }
            }
        );
    }

    qc_byte_order!(prop_u16, u16, ::core::u16::MAX as u64, read_u16, write_u16);
    qc_byte_order!(prop_i16, i16, ::core::i16::MAX as u64, read_i16, write_i16);
    qc_byte_order!(prop_u32, u32, ::core::u32::MAX as u64, read_u32, write_u32);
    qc_byte_order!(prop_i32, i32, ::core::i32::MAX as u64, read_i32, write_i32);
    qc_byte_order!(prop_u64, u64, ::core::u64::MAX as u64, read_u64, write_u64);
    qc_byte_order!(prop_i64, i64, ::core::i64::MAX as u64, read_i64, write_i64);
    qc_byte_order!(prop_f32, f32, ::core::u64::MAX as u64, read_f32, write_f32);
    qc_byte_order!(prop_f64, f64, ::core::i64::MAX as u64, read_f64, write_f64);

    #[cfg(feature = "i128")]
    qc_byte_order!(prop_u128, Wi128<u128>, 16 + 1, read_u128, write_u128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_i128, Wi128<i128>, 16 + 1, read_i128, write_i128);

    qc_byte_order!(prop_uint_1,
        u64, calc_max!(super::U64_MAX, 1), 1, read_uint, write_uint);
    qc_byte_order!(prop_uint_2,
        u64, calc_max!(super::U64_MAX, 2), 2, read_uint, write_uint);
    qc_byte_order!(prop_uint_3,
        u64, calc_max!(super::U64_MAX, 3), 3, read_uint, write_uint);
    qc_byte_order!(prop_uint_4,
        u64, calc_max!(super::U64_MAX, 4), 4, read_uint, write_uint);
    qc_byte_order!(prop_uint_5,
        u64, calc_max!(super::U64_MAX, 5), 5, read_uint, write_uint);
    qc_byte_order!(prop_uint_6,
        u64, calc_max!(super::U64_MAX, 6), 6, read_uint, write_uint);
    qc_byte_order!(prop_uint_7,
        u64, calc_max!(super::U64_MAX, 7), 7, read_uint, write_uint);
    qc_byte_order!(prop_uint_8,
        u64, calc_max!(super::U64_MAX, 8), 8, read_uint, write_uint);

    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_1,
        Wi128<u128>, 1, 1, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_2,
        Wi128<u128>, 2, 2, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_3,
        Wi128<u128>, 3, 3, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_4,
        Wi128<u128>, 4, 4, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_5,
        Wi128<u128>, 5, 5, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_6,
        Wi128<u128>, 6, 6, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_7,
        Wi128<u128>, 7, 7, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_8,
        Wi128<u128>, 8, 8, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_9,
        Wi128<u128>, 9, 9, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_10,
        Wi128<u128>, 10, 10, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_11,
        Wi128<u128>, 11, 11, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_12,
        Wi128<u128>, 12, 12, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_13,
        Wi128<u128>, 13, 13, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_14,
        Wi128<u128>, 14, 14, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_15,
        Wi128<u128>, 15, 15, read_uint128, write_uint128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_uint128_16,
        Wi128<u128>, 16, 16, read_uint128, write_uint128);

    qc_byte_order!(prop_int_1,
        i64, calc_max!(super::I64_MAX, 1), 1, read_int, write_int);
    qc_byte_order!(prop_int_2,
        i64, calc_max!(super::I64_MAX, 2), 2, read_int, write_int);
    qc_byte_order!(prop_int_3,
        i64, calc_max!(super::I64_MAX, 3), 3, read_int, write_int);
    qc_byte_order!(prop_int_4,
        i64, calc_max!(super::I64_MAX, 4), 4, read_int, write_int);
    qc_byte_order!(prop_int_5,
        i64, calc_max!(super::I64_MAX, 5), 5, read_int, write_int);
    qc_byte_order!(prop_int_6,
        i64, calc_max!(super::I64_MAX, 6), 6, read_int, write_int);
    qc_byte_order!(prop_int_7,
        i64, calc_max!(super::I64_MAX, 7), 7, read_int, write_int);
    qc_byte_order!(prop_int_8,
        i64, calc_max!(super::I64_MAX, 8), 8, read_int, write_int);

    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_1,
        Wi128<i128>, 1, 1, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_2,
        Wi128<i128>, 2, 2, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_3,
        Wi128<i128>, 3, 3, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_4,
        Wi128<i128>, 4, 4, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_5,
        Wi128<i128>, 5, 5, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_6,
        Wi128<i128>, 6, 6, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_7,
        Wi128<i128>, 7, 7, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_8,
        Wi128<i128>, 8, 8, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_9,
        Wi128<i128>, 9, 9, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_10,
        Wi128<i128>, 10, 10, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_11,
        Wi128<i128>, 11, 11, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_12,
        Wi128<i128>, 12, 12, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_13,
        Wi128<i128>, 13, 13, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_14,
        Wi128<i128>, 14, 14, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_15,
        Wi128<i128>, 15, 15, read_int128, write_int128);
    #[cfg(feature = "i128")]
    qc_byte_order!(prop_int128_16,
        Wi128<i128>, 16, 16, read_int128, write_int128);


    // Test that all of the byte conversion functions panic when given a
    // buffer that is too small.
    //
    // These tests are critical to ensure safety, otherwise we might end up
    // with a buffer overflow.
    macro_rules! too_small {
        ($name:ident, $maximally_small:expr, $zero:expr,
         $read:ident, $write:ident) => (
            mod $name {
                use {BigEndian, ByteOrder, NativeEndian, LittleEndian};

                #[test]
                #[should_panic]
                fn read_big_endian() {
                    let buf = [0; $maximally_small];
                    BigEndian::$read(&buf);
                }

                #[test]
                #[should_panic]
                fn read_little_endian() {
                    let buf = [0; $maximally_small];
                    LittleEndian::$read(&buf);
                }

                #[test]
                #[should_panic]
                fn read_native_endian() {
                    let buf = [0; $maximally_small];
                    NativeEndian::$read(&buf);
                }

                #[test]
                #[should_panic]
                fn write_big_endian() {
                    let mut buf = [0; $maximally_small];
                    BigEndian::$write(&mut buf, $zero);
                }

                #[test]
                #[should_panic]
                fn write_little_endian() {
                    let mut buf = [0; $maximally_small];
                    LittleEndian::$write(&mut buf, $zero);
                }

                #[test]
                #[should_panic]
                fn write_native_endian() {
                    let mut buf = [0; $maximally_small];
                    NativeEndian::$write(&mut buf, $zero);
                }
            }
        );
        ($name:ident, $maximally_small:expr, $read:ident) => (
            mod $name {
                use {BigEndian, ByteOrder, NativeEndian, LittleEndian};

                #[test]
                #[should_panic]
                fn read_big_endian() {
                    let buf = [0; $maximally_small];
                    BigEndian::$read(&buf, $maximally_small + 1);
                }

                #[test]
                #[should_panic]
                fn read_little_endian() {
                    let buf = [0; $maximally_small];
                    LittleEndian::$read(&buf, $maximally_small + 1);
                }

                #[test]
                #[should_panic]
                fn read_native_endian() {
                    let buf = [0; $maximally_small];
                    NativeEndian::$read(&buf, $maximally_small + 1);
                }
            }
        );
    }

    too_small!(small_u16, 1, 0, read_u16, write_u16);
    too_small!(small_i16, 1, 0, read_i16, write_i16);
    too_small!(small_u32, 3, 0, read_u32, write_u32);
    too_small!(small_i32, 3, 0, read_i32, write_i32);
    too_small!(small_u64, 7, 0, read_u64, write_u64);
    too_small!(small_i64, 7, 0, read_i64, write_i64);
    too_small!(small_f32, 3, 0.0, read_f32, write_f32);
    too_small!(small_f64, 7, 0.0, read_f64, write_f64);
    #[cfg(feature = "i128")]
    too_small!(small_u128, 15, 0, read_u128, write_u128);
    #[cfg(feature = "i128")]
    too_small!(small_i128, 15, 0, read_i128, write_i128);

    too_small!(small_uint_1, 1, read_uint);
    too_small!(small_uint_2, 2, read_uint);
    too_small!(small_uint_3, 3, read_uint);
    too_small!(small_uint_4, 4, read_uint);
    too_small!(small_uint_5, 5, read_uint);
    too_small!(small_uint_6, 6, read_uint);
    too_small!(small_uint_7, 7, read_uint);

    #[cfg(feature = "i128")]
    too_small!(small_uint128_1, 1, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_2, 2, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_3, 3, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_4, 4, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_5, 5, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_6, 6, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_7, 7, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_8, 8, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_9, 9, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_10, 10, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_11, 11, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_12, 12, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_13, 13, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_14, 14, read_uint128);
    #[cfg(feature = "i128")]
    too_small!(small_uint128_15, 15, read_uint128);

    too_small!(small_int_1, 1, read_int);
    too_small!(small_int_2, 2, read_int);
    too_small!(small_int_3, 3, read_int);
    too_small!(small_int_4, 4, read_int);
    too_small!(small_int_5, 5, read_int);
    too_small!(small_int_6, 6, read_int);
    too_small!(small_int_7, 7, read_int);

    #[cfg(feature = "i128")]
    too_small!(small_int128_1, 1, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_2, 2, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_3, 3, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_4, 4, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_5, 5, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_6, 6, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_7, 7, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_8, 8, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_9, 9, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_10, 10, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_11, 11, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_12, 12, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_13, 13, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_14, 14, read_int128);
    #[cfg(feature = "i128")]
    too_small!(small_int128_15, 15, read_int128);

    too_small!(small_float_1, 1, read_float);
    too_small!(small_float_2, 2, read_float);
    too_small!(small_float_3, 3, read_float);
    too_small!(small_float_4, 4, read_float);
    too_small!(small_float_5, 5, read_float);
    too_small!(small_float_6, 6, read_float);
    too_small!(small_float_7, 7, read_float);

    #[test]
    fn uint_bigger_buffer() {
        use {ByteOrder, LittleEndian};
        let n = LittleEndian::read_uint(&[1, 2, 3, 4, 5, 6, 7, 8], 5);
        assert_eq!(n, 0x0504030201);
    }

    #[test]
    fn read_snan() {
        use {ByteOrder, BigEndian, LittleEndian};

        let sf = BigEndian::read_f32(&[0xFF, 0x80, 0x00, 0x01]);
        let sbits: u32 = unsafe { ::core::mem::transmute(sf) };

        // Check that this is the same value with the MSB of the fraction set (which should be a
        // valid qNaN)
        assert_eq!(sbits, 0xFFC00001);
        assert_eq!(sf.classify(), ::core::num::FpCategory::Nan);

        let df = BigEndian::read_f64(&[0x7F, 0xF0, 0, 0, 0, 0, 0, 0x01]);
        let dbits: u64 = unsafe { ::core::mem::transmute(df) };
        assert_eq!(dbits, 0x7FF8000000000001);
        assert_eq!(df.classify(), ::core::num::FpCategory::Nan);

        let bf = BigEndian::read_float(&[0x7F, 0xF0, 0, 0, 0, 0, 0, 0x01], 8);
        let bbits: u64 = unsafe { ::core::mem::transmute(bf) };
        assert_eq!(bbits, 0x7FF8000000000001);
        assert_eq!(bf.classify(), ::core::num::FpCategory::Nan);

        let lf = LittleEndian::read_float(&[0x01, 0, 0, 0, 0, 0, 0xF0, 0xFF], 8);
        let lbits: u64 = unsafe { ::core::mem::transmute(lf) };
        assert_eq!(lbits, 0xFFF8000000000001);
        assert_eq!(lf.classify(), ::core::num::FpCategory::Nan);
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod stdtests {
    macro_rules! calc_max {
        ($max:expr, $bytes:expr) => { ($max - 1) >> (8 * (8 - $bytes)) };
    }

    macro_rules! qc_bytes_ext {
        ($name:ident, $ty_int:ty, $max:expr,
         $bytes:expr, $read:ident, $write:ident) => (
            mod $name {
                use std::io::Cursor;
                use {
                    ReadBytesExt, WriteBytesExt,
                    BigEndian, NativeEndian, LittleEndian,
                };
                #[allow(unused_imports)] use test::{ qc_sized, Wi128 };

                #[test]
                fn big_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut wtr = vec![];
                        wtr.$write::<BigEndian>(n.clone()).unwrap();
                        let mut rdr = Vec::new();
                        rdr.extend(wtr[wtr.len()-$bytes..].iter().map(|&x| x));
                        let mut rdr = Cursor::new(rdr);
                        n == rdr.$read::<BigEndian>($bytes).unwrap()
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max);
                }

                #[test]
                fn little_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut wtr = vec![];
                        wtr.$write::<LittleEndian>(n.clone()).unwrap();
                        let mut rdr = Cursor::new(wtr);
                        n == rdr.$read::<LittleEndian>($bytes).unwrap()
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max);
                }

                #[test]
                fn native_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut wtr = vec![];
                        wtr.$write::<NativeEndian>(n.clone()).unwrap();
                        let mut rdr = Cursor::new(wtr);
                        n == rdr.$read::<NativeEndian>($bytes).unwrap()
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max);
                }
            }
        );
        ($name:ident, $ty_int:ty, $max:expr, $read:ident, $write:ident) => (
            mod $name {
                use std::io::Cursor;
                use {
                    ReadBytesExt, WriteBytesExt,
                    BigEndian, NativeEndian, LittleEndian,
                };
                #[allow(unused_imports)] use test::{ qc_sized, Wi128 };

                #[test]
                fn big_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut wtr = vec![];
                        wtr.$write::<BigEndian>(n.clone()).unwrap();
                        let mut rdr = Cursor::new(wtr);
                        n == rdr.$read::<BigEndian>().unwrap()
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max - 1);
                }

                #[test]
                fn little_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut wtr = vec![];
                        wtr.$write::<LittleEndian>(n.clone()).unwrap();
                        let mut rdr = Cursor::new(wtr);
                        n == rdr.$read::<LittleEndian>().unwrap()
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max - 1);
                }

                #[test]
                fn native_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut wtr = vec![];
                        wtr.$write::<NativeEndian>(n.clone()).unwrap();
                        let mut rdr = Cursor::new(wtr);
                        n == rdr.$read::<NativeEndian>().unwrap()
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max - 1);
                }
            }
        );
    }

    qc_bytes_ext!(prop_ext_u16,
        u16, ::std::u16::MAX as u64, read_u16, write_u16);
    qc_bytes_ext!(prop_ext_i16,
        i16, ::std::i16::MAX as u64, read_i16, write_i16);
    qc_bytes_ext!(prop_ext_u32,
        u32, ::std::u32::MAX as u64, read_u32, write_u32);
    qc_bytes_ext!(prop_ext_i32,
        i32, ::std::i32::MAX as u64, read_i32, write_i32);
    qc_bytes_ext!(prop_ext_u64,
        u64, ::std::u64::MAX as u64, read_u64, write_u64);
    qc_bytes_ext!(prop_ext_i64,
        i64, ::std::i64::MAX as u64, read_i64, write_i64);
    qc_bytes_ext!(prop_ext_f32,
        f32, ::std::u64::MAX as u64, read_f32, write_f32);
    qc_bytes_ext!(prop_ext_f64,
        f64, ::std::i64::MAX as u64, read_f64, write_f64);

    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_u128, Wi128<u128>, 16 + 1, read_u128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_i128, Wi128<i128>, 16 + 1, read_i128, write_i128);

    qc_bytes_ext!(prop_ext_uint_1,
        u64, calc_max!(::test::U64_MAX, 1), 1, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_2,
        u64, calc_max!(::test::U64_MAX, 2), 2, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_3,
        u64, calc_max!(::test::U64_MAX, 3), 3, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_4,
        u64, calc_max!(::test::U64_MAX, 4), 4, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_5,
        u64, calc_max!(::test::U64_MAX, 5), 5, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_6,
        u64, calc_max!(::test::U64_MAX, 6), 6, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_7,
        u64, calc_max!(::test::U64_MAX, 7), 7, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_8,
        u64, calc_max!(::test::U64_MAX, 8), 8, read_uint, write_u64);

    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_1,
        Wi128<u128>, 1, 1, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_2,
        Wi128<u128>, 2, 2, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_3,
        Wi128<u128>, 3, 3, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_4,
        Wi128<u128>, 4, 4, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_5,
        Wi128<u128>, 5, 5, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_6,
        Wi128<u128>, 6, 6, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_7,
        Wi128<u128>, 7, 7, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_8,
        Wi128<u128>, 8, 8, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_9,
        Wi128<u128>, 9, 9, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_10,
        Wi128<u128>, 10, 10, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_11,
        Wi128<u128>, 11, 11, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_12,
        Wi128<u128>, 12, 12, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_13,
        Wi128<u128>, 13, 13, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_14,
        Wi128<u128>, 14, 14, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_15,
        Wi128<u128>, 15, 15, read_uint128, write_u128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_uint128_16,
        Wi128<u128>, 16, 16, read_uint128, write_u128);

    qc_bytes_ext!(prop_ext_int_1,
        i64, calc_max!(::test::I64_MAX, 1), 1, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_2,
        i64, calc_max!(::test::I64_MAX, 2), 2, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_3,
        i64, calc_max!(::test::I64_MAX, 3), 3, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_4,
        i64, calc_max!(::test::I64_MAX, 4), 4, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_5,
        i64, calc_max!(::test::I64_MAX, 5), 5, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_6,
        i64, calc_max!(::test::I64_MAX, 6), 6, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_7,
        i64, calc_max!(::test::I64_MAX, 1), 7, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_8,
        i64, calc_max!(::test::I64_MAX, 8), 8, read_int, write_i64);

    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_1,
        Wi128<i128>, 1, 1, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_2,
        Wi128<i128>, 2, 2, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_3,
        Wi128<i128>, 3, 3, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_4,
        Wi128<i128>, 4, 4, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_5,
        Wi128<i128>, 5, 5, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_6,
        Wi128<i128>, 6, 6, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_7,
        Wi128<i128>, 7, 7, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_8,
        Wi128<i128>, 8, 8, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_9,
        Wi128<i128>, 9, 9, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_10,
        Wi128<i128>, 10, 10, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_11,
        Wi128<i128>, 11, 11, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_12,
        Wi128<i128>, 12, 12, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_13,
        Wi128<i128>, 13, 13, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_14,
        Wi128<i128>, 14, 14, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_15,
        Wi128<i128>, 15, 15, read_int128, write_i128);
    #[cfg(feature = "i128")]
    qc_bytes_ext!(prop_ext_int128_16,
        Wi128<i128>, 16, 16, read_int128, write_i128);
}
