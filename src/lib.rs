/*!
This crate provides convenience methods for encoding and decoding numbers
in either big-endian or little-endian order.

The organization of the crate is pretty simple. A trait, `ByteOrder`, specifies
byte conversion methods for each type of number in Rust (sans numbers that have
a platform dependent size like `usize` and `isize`). Two types, `BigEndian`
and `LittleEndian` implement these methods. Finally, `ReadBytesExt` and
`WriteBytesExt` provide convenience methods available to all types that
implement `Read` and `Write`.

**Future plans:** Currently, this crate works with `std::old_io` via the
`ReaderBytesExt` and `WriterBytesExt` traits; however, you should prefer the
`ReadBytesExt` and `WriteBytesExt` traits, which work with the new `std::io`
module. I will keep the `std::old_io` traits here for as long as reasonable.

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

#![crate_name = "byteorder"]
#![doc(html_root_url = "http://burntsushi.net/rustdoc/byteorder")]

#![deny(missing_docs)]

#![allow(unused_features)] // for `rand` while testing
#![feature(core, io, test, old_io)]

use std::mem::transmute;

pub use new::{ReadBytesExt, WriteBytesExt, Error, Result};
pub use old::{ReaderBytesExt, WriterBytesExt};

mod new;
mod old;

fn extend_sign(val: u64, nbytes: usize) -> i64 {
    let shift  = (8 - nbytes) * 8;
    (val << shift) as i64 >> shift
}

/// ByteOrder describes types that can serialize integers as bytes.
///
/// Note that `Self` does not appear anywhere in this trait's definition!
/// Therefore, in order to use it, you'll need to use syntax like
/// `<T as ByteOrder>::read_u16(&[0, 1])` where `T` implements `ByteOrder`.
///
/// This crate provides two types that implement `ByteOrder`: `BigEndian`
/// and `LittleEndian`.
///
/// # Examples
///
/// Write and read `u32` numbers in little endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, LittleEndian};
///
/// let mut buf = [0; 4];
/// <LittleEndian as ByteOrder>::write_u32(&mut buf, 1_000_000);
/// assert_eq!(1_000_000, <LittleEndian as ByteOrder>::read_u32(&buf));
/// ```
///
/// Write and read `i16` numbers in big endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, BigEndian};
///
/// let mut buf = [0; 2];
/// <BigEndian as ByteOrder>::write_i16(&mut buf, -50_000);
/// assert_eq!(-50_000, <BigEndian as ByteOrder>::read_i16(&buf));
/// ```
pub trait ByteOrder : std::marker::MarkerTrait {
    /// Reads an unsigned 16 bit integer from `buf`.
    ///
    /// Task failure occurs when `buf.len() < 2`.
    fn read_u16(buf: &[u8]) -> u16;

    /// Reads an unsigned 32 bit integer from `buf`.
    ///
    /// Task failure occurs when `buf.len() < 4`.
    fn read_u32(buf: &[u8]) -> u32;

    /// Reads an unsigned 64 bit integer from `buf`.
    ///
    /// Task failure occurs when `buf.len() < 8`.
    fn read_u64(buf: &[u8]) -> u64;

    /// Reads an unsigned n-bytes integer from `buf`.
    ///
    /// Task failure occurs when `nbytes < 1` or `nbytes > 8` or `buf.len() < nbytes`
    fn read_uint(buf: &[u8], nbytes: usize) -> u64;

    /// Writes an unsigned 16 bit integer `n` to `buf`.
    ///
    /// Task failure occurs when `buf.len() < 2`.
    fn write_u16(buf: &mut [u8], n: u16);

    /// Writes an unsigned 32 bit integer `n` to `buf`.
    ///
    /// Task failure occurs when `buf.len() < 4`.
    fn write_u32(buf: &mut [u8], n: u32);

    /// Writes an unsigned 64 bit integer `n` to `buf`.
    ///
    /// Task failure occurs when `buf.len() < 8`.
    fn write_u64(buf: &mut [u8], n: u64);

    /// Reads a signed 16 bit integer from `buf`.
    ///
    /// Task failure occurs when `buf.len() < 2`.
    fn read_i16(buf: &[u8]) -> i16 {
        <Self as ByteOrder>::read_u16(buf) as i16
    }

    /// Reads a signed 32 bit integer from `buf`.
    ///
    /// Task failure occurs when `buf.len() < 4`.
    fn read_i32(buf: &[u8]) -> i32 {
        <Self as ByteOrder>::read_u32(buf) as i32
    }

    /// Reads a signed 64 bit integer from `buf`.
    ///
    /// Task failure occurs when `buf.len() < 8`.
    fn read_i64(buf: &[u8]) -> i64 {
        <Self as ByteOrder>::read_u64(buf) as i64
    }

    /// Reads a signed n-bytes integer from `buf`.
    ///
    /// Task failure occurs when `nbytes < 1` or `nbytes > 8` or `buf.len() < nbytes`
    fn read_int(buf: &[u8], nbytes: usize) -> i64 {
        extend_sign(<Self as ByteOrder>::read_uint(buf, nbytes), nbytes)
    }

    /// Reads a IEEE754 single-precision (4 bytes) floating point number.
    ///
    /// Task failure occurs when `buf.len() < 4`.
    fn read_f32(buf: &[u8]) -> f32 {
        unsafe { transmute(<Self as ByteOrder>::read_u32(buf)) }
    }

    /// Reads a IEEE754 double-precision (8 bytes) floating point number.
    ///
    /// Task failure occurs when `buf.len() < 8`.
    fn read_f64(buf: &[u8]) -> f64 {
        unsafe { transmute(<Self as ByteOrder>::read_u64(buf)) }
    }

    /// Writes a signed 16 bit integer `n` to `buf`.
    ///
    /// Task failure occurs when `buf.len() < 2`.
    fn write_i16(buf: &mut [u8], n: i16) {
        <Self as ByteOrder>::write_u16(buf, n as u16)
    }

    /// Writes a signed 32 bit integer `n` to `buf`.
    ///
    /// Task failure occurs when `buf.len() < 4`.
    fn write_i32(buf: &mut [u8], n: i32) {
        <Self as ByteOrder>::write_u32(buf, n as u32)
    }

    /// Writes a signed 64 bit integer `n` to `buf`.
    ///
    /// Task failure occurs when `buf.len() < 8`.
    fn write_i64(buf: &mut [u8], n: i64) {
        <Self as ByteOrder>::write_u64(buf, n as u64)
    }

    /// Writes a IEEE754 single-precision (4 bytes) floating point number.
    ///
    /// Task failure occurs when `buf.len() < 4`.
    fn write_f32(buf: &mut [u8], n: f32) {
        <Self as ByteOrder>::write_u32(buf, unsafe { transmute(n) })
    }

    /// Writes a IEEE754 double-precision (8 bytes) floating point number.
    ///
    /// Task failure occurs when `buf.len() < 8`.
    fn write_f64(buf: &mut [u8], n: f64) {
        <Self as ByteOrder>::write_u64(buf, unsafe { transmute(n) })
    }
}

/// Defines big-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
#[allow(missing_copy_implementations)] pub enum BigEndian {}

/// Defines little-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
#[allow(missing_copy_implementations)] pub enum LittleEndian {}

macro_rules! read_num_bytes {
    ($ty:ty, $size:expr, $src:expr, $which:ident) => ({
        use std::num::Int;
        use std::ptr::copy_nonoverlapping_memory;

        assert!($src.len() >= $size); // critical for memory safety!
        let mut out = [0u8; $size];
        let ptr_out = out.as_mut_ptr();
        unsafe {
            copy_nonoverlapping_memory(ptr_out, $src.as_ptr(), $size);
            (*(ptr_out as *const $ty)).$which()
        }
    });
    ($ty:ty, $size:expr, le $bytes:expr, $src:expr, $which:ident) => ({
        use std::num::Int;
        use std::ptr::copy_nonoverlapping_memory;

        assert!($bytes > 0 && $bytes < 9 && $bytes <= $src.len());
        let mut out = [0u8; $size];
        let ptr_out = out.as_mut_ptr();
        unsafe {
            copy_nonoverlapping_memory(ptr_out, $src.as_ptr(), $bytes);
            (*(ptr_out as *const $ty)).$which()
        }
    });
    ($ty:ty, $size:expr, be $bytes:expr, $src:expr, $which:ident) => ({
        use std::num::Int;
        use std::ptr::copy_nonoverlapping_memory;

        assert!($bytes > 0 && $bytes < 9 && $bytes <= $src.len());
        let mut out = [0u8; $size];
        let ptr_out = out.as_mut_ptr();
        unsafe {
            copy_nonoverlapping_memory(ptr_out.offset((8 - $bytes) as isize),
                                       $src.as_ptr(), $bytes);
            (*(ptr_out as *const $ty)).$which()
        }
    });
}

macro_rules! write_num_bytes {
    ($ty:ty, $size:expr, $n:expr, $dst:expr, $which:ident) => ({
        use std::num::Int;
        use std::ptr::copy_nonoverlapping_memory;

        assert!($dst.len() >= $size); // critical for memory safety!
        unsafe {
            // n.b. https://github.com/rust-lang/rust/issues/22776
            let bytes = transmute::<_, [u8; $size]>($n.$which());
            copy_nonoverlapping_memory($dst.as_mut_ptr(), (&bytes).as_ptr(), $size);
        }
    });
}

impl ByteOrder for BigEndian {
    fn read_u16(buf: &[u8]) -> u16 {
        read_num_bytes!(u16, 2, buf, to_be)
    }

    fn read_u32(buf: &[u8]) -> u32 {
        read_num_bytes!(u32, 4, buf, to_be)
    }

    fn read_u64(buf: &[u8]) -> u64 {
        read_num_bytes!(u64, 8, buf, to_be)
    }

    fn read_uint(buf: &[u8], nbytes: usize) -> u64 {
        read_num_bytes!(u64, 8, be nbytes, buf, to_be)
    }

    fn write_u16(buf: &mut [u8], n: u16) {
        write_num_bytes!(u16, 2, n, buf, to_be);
    }

    fn write_u32(buf: &mut [u8], n: u32) {
        write_num_bytes!(u32, 4, n, buf, to_be);
    }

    fn write_u64(buf: &mut [u8], n: u64) {
        write_num_bytes!(u64, 8, n, buf, to_be);
    }
}

impl ByteOrder for LittleEndian {
    fn read_u16(buf: &[u8]) -> u16 {
        read_num_bytes!(u16, 2, buf, to_le)
    }

    fn read_u32(buf: &[u8]) -> u32 {
        read_num_bytes!(u32, 4, buf, to_le)
    }

    fn read_u64(buf: &[u8]) -> u64 {
        read_num_bytes!(u64, 8, buf, to_le)
    }

    fn read_uint(buf: &[u8], nbytes: usize) -> u64 {
        read_num_bytes!(u64, 8, le nbytes, buf, to_le)
    }

    fn write_u16(buf: &mut [u8], n: u16) {
        write_num_bytes!(u16, 2, n, buf, to_le);
    }

    fn write_u32(buf: &mut [u8], n: u32) {
        write_num_bytes!(u32, 4, n, buf, to_le);
    }

    fn write_u64(buf: &mut [u8], n: u64) {
        write_num_bytes!(u64, 8, n, buf, to_le);
    }
}

#[cfg(test)]
mod test {
    extern crate quickcheck;
    extern crate rand;

    use test::rand::thread_rng;
    use test::quickcheck::{QuickCheck, StdGen, Testable};

    fn qc_sized<A: Testable>(f: A, size: u64) {
        QuickCheck::new()
            .gen(StdGen::new(thread_rng(), size as usize))
            .tests(1_00)
            .max_tests(10_000)
            .quickcheck(f);
    }

    macro_rules! qc_byte_order {
        ($name:ident, $ty_int:ident, $max:ident,
         $bytes:expr, $read:ident, $write:ident) => (
            mod $name {
                use std::$ty_int;
                use {BigEndian, ByteOrder, LittleEndian};
                use super::qc_sized;

                #[test]
                fn big_endian() {
                    let max = $ty_int::$max as u64 - 1 >> (8 * (8 - $bytes));
                    fn prop(n: $ty_int) -> bool {
                        let mut buf = [0; 8];
                        <BigEndian as ByteOrder>::$write(&mut buf, n);
                        n == <BigEndian as ByteOrder>::$read(&mut buf[8 - $bytes..], $bytes)
                    }
                    qc_sized(prop as fn($ty_int) -> bool, max as u64 - 1);
                }

                #[test]
                fn little_endian() {
                    let max = $ty_int::$max as u64 - 1 >> (8 * (8 - $bytes));
                    fn prop(n: $ty_int) -> bool {
                        let mut buf = [0; 8];
                        <LittleEndian as ByteOrder>::$write(&mut buf, n);
                        n == <LittleEndian as ByteOrder>::$read(&mut buf, $bytes)
                    }
                    qc_sized(prop as fn($ty_int) -> bool, max as u64 - 1);
                }
            }
        );
        ($name:ident, $ty_int:ident, $max:ident,
         $read:ident, $write:ident) => (
            mod $name {
                use std::$ty_int;
                use {BigEndian, ByteOrder, LittleEndian};
                use super::qc_sized;

                #[test]
                fn big_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut buf = [0; 8];
                        <BigEndian as ByteOrder>::$write(&mut buf, n);
                        n == <BigEndian as ByteOrder>::$read(&mut buf)
                    }
                    qc_sized(prop as fn($ty_int) -> bool,
                             $ty_int::$max as u64 - 1);
                }

                #[test]
                fn little_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut buf = [0; 8];
                        <LittleEndian as ByteOrder>::$write(&mut buf, n);
                        n == <LittleEndian as ByteOrder>::$read(&mut buf)
                    }
                    qc_sized(prop as fn($ty_int) -> bool,
                             $ty_int::$max as u64 - 1);
                }
            }
        );
    }

    qc_byte_order!(prop_u16, u16, MAX, read_u16, write_u16);
    qc_byte_order!(prop_i16, i16, MAX, read_i16, write_i16);
    qc_byte_order!(prop_u32, u32, MAX, read_u32, write_u32);
    qc_byte_order!(prop_i32, i32, MAX, read_i32, write_i32);
    qc_byte_order!(prop_u64, u64, MAX, read_u64, write_u64);
    qc_byte_order!(prop_i64, i64, MAX, read_i64, write_i64);
    qc_byte_order!(prop_f32, f32, MAX, read_f32, write_f32);
    qc_byte_order!(prop_f64, f64, MAX, read_f64, write_f64);

    qc_byte_order!(prop_uint_1, u64, MAX, 1, read_uint, write_u64);
    qc_byte_order!(prop_uint_2, u64, MAX, 2, read_uint, write_u64);
    qc_byte_order!(prop_uint_3, u64, MAX, 3, read_uint, write_u64);
    qc_byte_order!(prop_uint_4, u64, MAX, 4, read_uint, write_u64);
    qc_byte_order!(prop_uint_5, u64, MAX, 5, read_uint, write_u64);
    qc_byte_order!(prop_uint_6, u64, MAX, 6, read_uint, write_u64);
    qc_byte_order!(prop_uint_7, u64, MAX, 7, read_uint, write_u64);
    qc_byte_order!(prop_uint_8, u64, MAX, 8, read_uint, write_u64);

    qc_byte_order!(prop_int_1, i64, MAX, 1, read_int, write_i64);
    qc_byte_order!(prop_int_2, i64, MAX, 2, read_int, write_i64);
    qc_byte_order!(prop_int_3, i64, MAX, 3, read_int, write_i64);
    qc_byte_order!(prop_int_4, i64, MAX, 4, read_int, write_i64);
    qc_byte_order!(prop_int_5, i64, MAX, 5, read_int, write_i64);
    qc_byte_order!(prop_int_6, i64, MAX, 6, read_int, write_i64);
    qc_byte_order!(prop_int_7, i64, MAX, 7, read_int, write_i64);
    qc_byte_order!(prop_int_8, i64, MAX, 8, read_int, write_i64);

    macro_rules! qc_bytes_ext {
        ($name:ident, $ty_int:ident, $max:ident,
         $bytes:expr, $read:ident, $write:ident) => (
            mod $name {
                use std::io::Cursor;
                use std::$ty_int;
                use {ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
                use super::qc_sized;

                #[test]
                fn big_endian() {
                    let max = $ty_int::$max as u64 - 1 >> (8 * (8 - $bytes));
                    fn prop(n: $ty_int) -> bool {
                        let mut wtr = vec![];
                        wtr.$write::<BigEndian>(n).unwrap();
                        let mut rdr = Vec::new();
                        rdr.push_all(&wtr[8-$bytes..]);
                        let mut rdr = Cursor::new(rdr);
                        n == rdr.$read::<BigEndian>($bytes).unwrap()
                    }
                    qc_sized(prop as fn($ty_int) -> bool, max);
                }

                #[test]
                fn little_endian() {
                    let max = $ty_int::$max as u64 - 1 >> (8 * (8 - $bytes));
                    fn prop(n: $ty_int) -> bool {
                        let mut wtr = vec![];
                        wtr.$write::<LittleEndian>(n).unwrap();
                        let mut rdr = Cursor::new(wtr);
                        n == rdr.$read::<LittleEndian>($bytes).unwrap()
                    }
                    qc_sized(prop as fn($ty_int) -> bool, max);
                }
            }
        );
        ($name:ident, $ty_int:ident,
         $max:ident, $read:ident, $write:ident) => (
            mod $name {
                use std::io::Cursor;
                use std::$ty_int;
                use {ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
                use super::qc_sized;

                #[test]
                fn big_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut wtr = vec![];
                        wtr.$write::<BigEndian>(n).unwrap();
                        let mut rdr = Cursor::new(wtr);
                        n == rdr.$read::<BigEndian>().unwrap()
                    }
                    qc_sized(prop as fn($ty_int) -> bool,
                             $ty_int::$max as u64 - 1);
                }

                #[test]
                fn little_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut wtr = vec![];
                        wtr.$write::<LittleEndian>(n).unwrap();
                        let mut rdr = Cursor::new(wtr);
                        n == rdr.$read::<LittleEndian>().unwrap()
                    }
                    qc_sized(prop as fn($ty_int) -> bool,
                             $ty_int::$max as u64 - 1);
                }
            }
        );
    }

    qc_bytes_ext!(prop_ext_u16, u16, MAX, read_u16, write_u16);
    qc_bytes_ext!(prop_ext_i16, i16, MAX, read_i16, write_i16);
    qc_bytes_ext!(prop_ext_u32, u32, MAX, read_u32, write_u32);
    qc_bytes_ext!(prop_ext_i32, i32, MAX, read_i32, write_i32);
    qc_bytes_ext!(prop_ext_u64, u64, MAX, read_u64, write_u64);
    qc_bytes_ext!(prop_ext_i64, i64, MAX, read_i64, write_i64);
    qc_bytes_ext!(prop_ext_f32, f32, MAX, read_f32, write_f32);
    qc_bytes_ext!(prop_ext_f64, f64, MAX, read_f64, write_f64);

    qc_bytes_ext!(prop_ext_uint_1, u64, MAX, 1, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_2, u64, MAX, 2, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_3, u64, MAX, 3, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_4, u64, MAX, 4, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_5, u64, MAX, 5, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_6, u64, MAX, 6, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_7, u64, MAX, 7, read_uint, write_u64);
    qc_bytes_ext!(prop_ext_uint_8, u64, MAX, 8, read_uint, write_u64);

    qc_bytes_ext!(prop_ext_int_1, i64, MAX, 1, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_2, i64, MAX, 2, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_3, i64, MAX, 3, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_4, i64, MAX, 4, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_5, i64, MAX, 5, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_6, i64, MAX, 6, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_7, i64, MAX, 7, read_int, write_i64);
    qc_bytes_ext!(prop_ext_int_8, i64, MAX, 8, read_int, write_i64);

    // Test that all of the byte conversion functions panic when given a
    // buffer that is too small.
    //
    // These tests are critical to ensure safety, otherwise we might end up
    // with a buffer overflow.
    macro_rules! too_small {
        ($name:ident, $maximally_small:expr, $zero:expr,
         $read:ident, $write:ident) => (
            mod $name {
                use {BigEndian, ByteOrder, LittleEndian};

                #[test]
                #[should_fail]
                fn read_big_endian() {
                    let buf = [0; $maximally_small];
                    <BigEndian as ByteOrder>::$read(&buf);
                }

                #[test]
                #[should_fail]
                fn read_little_endian() {
                    let buf = [0; $maximally_small];
                    <LittleEndian as ByteOrder>::$read(&buf);
                }

                #[test]
                #[should_fail]
                fn write_big_endian() {
                    let mut buf = [0; $maximally_small];
                    <BigEndian as ByteOrder>::$write(&mut buf, $zero);
                }

                #[test]
                #[should_fail]
                fn write_little_endian() {
                    let mut buf = [0; $maximally_small];
                    <LittleEndian as ByteOrder>::$write(&mut buf, $zero);
                }
            }
        );
        ($name:ident, $maximally_small:expr, $read:ident) => (
            mod $name {
                use {BigEndian, ByteOrder, LittleEndian};

                #[test]
                #[should_fail]
                fn read_big_endian() {
                    let buf = [0; $maximally_small];
                    <BigEndian as ByteOrder>::$read(&buf, $maximally_small + 1);
                }

                #[test]
                #[should_fail]
                fn read_little_endian() {
                    let buf = [0; $maximally_small];
                    <LittleEndian as ByteOrder>::$read(&buf, $maximally_small + 1);
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

    too_small!(small_uint_1, 1, read_uint);
    too_small!(small_uint_2, 2, read_uint);
    too_small!(small_uint_3, 3, read_uint);
    too_small!(small_uint_4, 4, read_uint);
    too_small!(small_uint_5, 5, read_uint);
    too_small!(small_uint_6, 6, read_uint);
    too_small!(small_uint_7, 7, read_uint);

    too_small!(small_int_1, 1, read_int);
    too_small!(small_int_2, 2, read_int);
    too_small!(small_int_3, 3, read_int);
    too_small!(small_int_4, 4, read_int);
    too_small!(small_int_5, 5, read_int);
    too_small!(small_int_6, 6, read_int);
    too_small!(small_int_7, 7, read_int);
}

#[cfg(test)]
mod bench {
    extern crate test;

    macro_rules! bench_num {
        ($name:ident, $read:ident, $bytes:expr, $data:expr) => (
            mod $name {
                use {ByteOrder, BigEndian, LittleEndian};
                use super::test::Bencher;
                use super::test::black_box as bb;

                const NITER: usize = 100_000;

                #[bench]
                fn read_big_endian(b: &mut Bencher) {
                    let buf = $data;
                    b.iter(|| {
                        for _ in 0..NITER {
                            bb(<BigEndian as ByteOrder>::$read(&buf, $bytes));
                        }
                    });
                }

                #[bench]
                fn read_little_endian(b: &mut Bencher) {
                    let buf = $data;
                    b.iter(|| {
                        for _ in 0..NITER {
                            bb(<LittleEndian as ByteOrder>::$read(&buf, $bytes));
                        }
                    });
                }
            }
        );
        ($ty:ident, $max:ident,
         $read:ident, $write:ident, $size:expr, $data:expr) => (
            mod $ty {
                use std::$ty;
                use {ByteOrder, BigEndian, LittleEndian};
                use super::test::Bencher;
                use super::test::black_box as bb;

                const NITER: usize = 100_000;

                #[bench]
                fn read_big_endian(b: &mut Bencher) {
                    let buf = $data;
                    b.iter(|| {
                        for _ in 0..NITER {
                            bb(<BigEndian as ByteOrder>::$read(&buf));
                        }
                    });
                }

                #[bench]
                fn read_little_endian(b: &mut Bencher) {
                    let buf = $data;
                    b.iter(|| {
                        for _ in 0..NITER {
                            bb(<LittleEndian as ByteOrder>::$read(&buf));
                        }
                    });
                }

                #[bench]
                fn write_big_endian(b: &mut Bencher) {
                    let mut buf = $data;
                    let n = $ty::$max;
                    b.iter(|| {
                        for _ in 0..NITER {
                            bb(<BigEndian as ByteOrder>::$write(&mut buf, n));
                        }
                    });
                }

                #[bench]
                fn write_little_endian(b: &mut Bencher) {
                    let mut buf = $data;
                    let n = $ty::$max;
                    b.iter(|| {
                        for _ in 0..NITER {
                            bb(<LittleEndian as ByteOrder>::$write(&mut buf,
                                                                   n));
                        }
                    });
                }
            }
        );
    }

    bench_num!(u16, MAX, read_u16, write_u16, 2, [1, 2]);
    bench_num!(i16, MAX, read_i16, write_i16, 2, [1, 2]);
    bench_num!(u32, MAX, read_u32, write_u32, 4, [1, 2, 3, 4]);
    bench_num!(i32, MAX, read_i32, write_i32, 4, [1, 2, 3, 4]);
    bench_num!(u64, MAX, read_u64, write_u64, 8, [1, 2, 3, 4, 5, 6, 7, 8]);
    bench_num!(i64, MAX, read_i64, write_i64, 8, [1, 2, 3, 4, 5, 6, 7, 8]);
    bench_num!(f32, MAX, read_f32, write_f32, 4, [1, 2, 3, 4]);
    bench_num!(f64, MAX, read_f64, write_f64, 8,
               [1, 2, 3, 4, 5, 6, 7, 8]);

    bench_num!(uint_1, read_uint, 1, [1]);
    bench_num!(uint_2, read_uint, 2, [1, 2]);
    bench_num!(uint_3, read_uint, 3, [1, 2, 3]);
    bench_num!(uint_4, read_uint, 4, [1, 2, 3, 4]);
    bench_num!(uint_5, read_uint, 5, [1, 2, 3, 4, 5]);
    bench_num!(uint_6, read_uint, 6, [1, 2, 3, 4, 5, 6]);
    bench_num!(uint_7, read_uint, 7, [1, 2, 3, 4, 5, 6, 7]);
    bench_num!(uint_8, read_uint, 8, [1, 2, 3, 4, 5, 6, 7, 8]);

    bench_num!(int_1, read_int, 1, [1]);
    bench_num!(int_2, read_int, 2, [1, 2]);
    bench_num!(int_3, read_int, 3, [1, 2, 3]);
    bench_num!(int_4, read_int, 4, [1, 2, 3, 4]);
    bench_num!(int_5, read_int, 5, [1, 2, 3, 4, 5]);
    bench_num!(int_6, read_int, 6, [1, 2, 3, 4, 5, 6]);
    bench_num!(int_7, read_int, 7, [1, 2, 3, 4, 5, 6, 7]);
    bench_num!(int_8, read_int, 8, [1, 2, 3, 4, 5, 6, 7, 8]);
}
