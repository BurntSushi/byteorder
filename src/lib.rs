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

let mut reader = Cursor::new(vec![2, 5, 3, 0]);
// Note that we use type parameters to indicate which kind of byte order
// we want!
assert_eq!(517u16, reader.read_as::<BigEndian>().unwrap());
assert_eq!(768u16, reader.read_as::<BigEndian>().unwrap());
```

Write unsigned 16 bit little-endian integers to a `Write` type:

```rust
use byteorder::{LittleEndian, WriteBytesExt};

let mut writer = vec![];
writer.write_as::<LittleEndian>(517u16).unwrap();
writer.write_as::<LittleEndian>(768u16).unwrap();
assert_eq!(writer, vec![5, 2, 0, 3]);
```
*/

#![crate_name = "byteorder"]
#![doc(html_root_url = "http://burntsushi.net/rustdoc/byteorder")]

#![deny(missing_docs)]

use std::error;
use std::fmt;
use std::io;
use std::result;
use std::mem::transmute;

/// A short-hand for `result::Result<T, byteorder::Error>`.
pub type Result<T> = result::Result<T, Error>;

/// An error type for reading bytes.
///
/// This is a thin wrapper over the standard `io::Error` type. Namely, it
/// adds one additional error case: an unexpected EOF.
///
/// Note that this error is also used for the `write` methods to keep things
/// consistent.
#[derive(Debug)]
pub enum Error {
    /// An unexpected EOF.
    ///
    /// This occurs when a call to the underlying reader returns `0` bytes,
    /// but more bytes are required to decode a meaningful value.
    UnexpectedEOF,
    /// Any underlying IO error that occurs while reading bytes.
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::Io(err) }
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        match err {
            Error::Io(err) => err,
            Error::UnexpectedEOF => io::Error::new(io::ErrorKind::Other,
                                                   "unexpected EOF")
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnexpectedEOF => write!(f, "Unexpected end of file."),
            Error::Io(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnexpectedEOF => "Unexpected end of file.",
            Error::Io(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::UnexpectedEOF => None,
            Error::Io(ref err) => err.cause(),
        }
    }
}

fn read_full<R: io::Read + ?Sized>(reader: &mut R, buf: &mut [u8]) -> Result<()> {
    let mut nread = 0usize;
    while nread < buf.len() {
        match reader.read(&mut buf[nread..]) {
            Ok(0) => return Err(Error::UnexpectedEOF),
            Ok(n) => nread += n,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {},
            Err(e) => return Err(From::from(e))
        }
    }
    Ok(())
}

fn write_all<W: io::Write + ?Sized>(writer: &mut W, buf: &[u8]) -> Result<()> {
    writer.write_all(buf).map_err(From::from)
}

/// ByteOrder describes types that can serialize integers as bytes.
///
/// Note that `Self` does not appear anywhere in this trait's definition!
/// Therefore, in order to use it, you'll need to use syntax like
/// `T::from_bytes([0, 1])` where `T` implements `ByteOrder`.
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
/// let mut buf: [u8; 4];
/// buf = LittleEndian::into_bytes(1_000_000u32);
/// assert_eq!(1_000_000, LittleEndian::from_bytes(buf));
/// ```
///
/// Write and read `i16` numbers in big endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, BigEndian};
///
/// let mut buf: [u8; 2];
/// buf = BigEndian::into_bytes(-50_000i16);
/// assert_eq!(-50_000i16, BigEndian::from_bytes(buf));
/// ```
pub trait ByteOrder<T> {
    /// Conversion buffer type.
    ///
    /// Should be big enough to hold a T.
    ///
    /// ## Note
    /// This is a workaround until associated constants are stable. As soon as
    /// this happens, `Self::Buffer` will be replaced by an associated constant
    /// `Self::Len` and every use of `Self::Buffer` will be replaced by the
    /// expression `[u8; Self::Len]`.
    type Buffer: AsRef<[u8]> + AsMut<[u8]>;
    /// Converts the byte array `buf` into a `T`.
    fn from_bytes(buf: Self::Buffer) -> T;
    /// Converts `n` into a byte array.
    fn into_bytes(n: T) -> Self::Buffer;
    /// Returns a sufficiently big conversion buffer buffer.
    ///
    /// ## Note
    /// This is a workaround until associated constants are stable.
    /// Thus this method will be removed as soon as this happens.
    fn buffer() -> Self::Buffer;
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


macro_rules! impl_byte_order {
    ($val:ident, $bytes:expr, $byte_order:ident, $convert:ident) => {
        impl ByteOrder<$val> for $byte_order {
            type Buffer = [u8; $bytes];

            #[inline]
            fn from_bytes(buf: Self::Buffer) -> $val {
                unsafe { transmute::<_, $val>(buf) }.$convert()

            }

            #[inline]
            fn into_bytes(n: $val) -> Self::Buffer {
                unsafe { transmute(n.$convert()) }
            }

            #[inline]
            fn buffer() -> Self::Buffer {
                [0; $bytes]
            }
        }
    };
    ($byte_order:ident, $convert:ident) => {
        impl_byte_order!(u8 , 1, $byte_order, $convert);
        impl_byte_order!(u16, 2, $byte_order, $convert);
        impl_byte_order!(u32, 4, $byte_order, $convert);
        impl_byte_order!(u64, 8, $byte_order, $convert);
        impl_byte_order!(i8 , 1, $byte_order, $convert);
        impl_byte_order!(i16, 2, $byte_order, $convert);
        impl_byte_order!(i32, 4, $byte_order, $convert);
        impl_byte_order!(i64, 8, $byte_order, $convert);

        impl ByteOrder<f32> for $byte_order {
            type Buffer = [u8; 4];

            #[inline]
            fn from_bytes(buf: Self::Buffer) -> f32 {
                unsafe {
                    transmute(transmute::<_, u32>(buf).$convert())
                }
            }

            #[inline]
            fn into_bytes(n: f32) -> Self::Buffer {
                unsafe {
                    transmute(transmute::<_, u32>(n).$convert())
                }
            }

            #[inline]
            fn buffer() -> Self::Buffer {
                [0; 4]
            }
        }

        impl ByteOrder<f64> for $byte_order {
            type Buffer = [u8; 8];

            #[inline]
            fn from_bytes(buf: Self::Buffer) -> f64 {
                unsafe {
                    transmute(transmute::<_, u64>(buf).$convert())
                }
            }

            #[inline]
            fn into_bytes(n: f64) -> Self::Buffer {
                unsafe {
                    transmute(transmute::<_, u64>(n).$convert())
                }
            }

            #[inline]
            fn buffer() -> Self::Buffer {
                [0; 8]
            }
        }
    }
}

// Implement `ByteOrder` for built-in primitives
impl_byte_order!(LittleEndian, to_le);
impl_byte_order!(BigEndian, to_be);

/// Extends `Read` with methods for reading numbers. (For `std::io`.)
///
/// Most of the methods defined here have an unconstrained type parameter that
/// must be explicitly instantiated. Typically, it is instantiated with either
/// the `BigEndian` or `LittleEndian` types defined in this crate.
///
/// # Examples
///
/// Read unsigned 16 bit big-endian integers from a `Read`:
///
/// ```rust
/// use std::io::Cursor;
/// use byteorder::{BigEndian, ReadBytesExt};
///
/// let mut reader = Cursor::new(vec![2, 5, 3, 0, 63, 192, 0, 0]);
/// assert_eq!(517u16, reader.read_as::<BigEndian>().unwrap());
/// assert_eq!(768u16, reader.read_as::<BigEndian>().unwrap());
/// assert_eq!(1.5f32, reader.read_as::<BigEndian>().unwrap());
/// ```
pub trait ReadBytesExt<T> {
    /// Reads a value of type `T` from the underlying reader.
    fn read_as<B: ByteOrder<T>>(&mut self) -> Result<T>;
}

/// Implement ReadBytesExt for every type that implements `io::Read`
impl<T, R: io::Read> ReadBytesExt<T> for R {
    #[inline]
    fn read_as<B: ByteOrder<T>>(&mut self) -> Result<T> {
        let mut buf = B::buffer();
        try!(read_full(self, buf.as_mut()));
        Ok(B::from_bytes(buf))
    }
}

/// Extends `Write` with methods for writing numbers. (For `std::io`.)
///
/// Most of the methods defined here have an unconstrained type parameter that
/// must be explicitly instantiated. Typically, it is instantiated with either
/// the `BigEndian` or `LittleEndian` types defined in this crate.
///
/// # Examples
///
/// Write unsigned 16 bit big-endian integers to a `Write`:
///
/// ```rust
/// use byteorder::{BigEndian, WriteBytesExt};
///
/// let mut writer = vec![];
/// writer.write_as::<BigEndian>(517u16).unwrap();
/// writer.write_as::<BigEndian>(768u16).unwrap();
/// writer.write_as::<BigEndian>(1.5f32).unwrap();
/// assert_eq!(writer, vec![2, 5, 3, 0, 63, 192, 0, 0]);
/// ```
pub trait WriteBytesExt<T> {
    /// Writes a value of type `T` to the underlying writer.
    fn write_as<B: ByteOrder<T>>(&mut self, n: T) -> Result<()>;
}

/// Implement WriteBytesExt for every type that implements `io::Write`
impl<T, W: io::Write> WriteBytesExt<T> for W {
    #[inline]
    fn write_as<B: ByteOrder<T>>(&mut self, n: T) -> Result<()> {
        let buf = B::into_bytes(n);
        write_all(self, buf.as_ref())
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
        ($name:ident, $ty_int:ident, $max:expr) => (
            mod $name {
                use {BigEndian, ByteOrder, NativeEndian, LittleEndian};
                use super::qc_sized;

                #[test]
                fn big_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let buf = BigEndian::into_bytes(n);
                        n == BigEndian::from_bytes(buf)
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max - 1);
                }

                #[test]
                fn little_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let buf = LittleEndian::into_bytes(n);
                        n == LittleEndian::from_bytes(buf)
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max - 1);
                }

                #[test]
                fn native_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let buf = NativeEndian::into_bytes(n);
                        n == NativeEndian::from_bytes(buf)
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max - 1);
                }
            }
        );
    }

    qc_byte_order!(prop_u16, u16, ::std::u16::MAX as u64);
    qc_byte_order!(prop_i16, i16, ::std::i16::MAX as u64);
    qc_byte_order!(prop_u32, u32, ::std::u32::MAX as u64);
    qc_byte_order!(prop_i32, i32, ::std::i32::MAX as u64);
    qc_byte_order!(prop_u64, u64, ::std::u64::MAX as u64);
    qc_byte_order!(prop_i64, i64, ::std::i64::MAX as u64);
    qc_byte_order!(prop_f32, f32, ::std::u64::MAX as u64);
    qc_byte_order!(prop_f64, f64, ::std::i64::MAX as u64);

    macro_rules! qc_bytes_ext {
        ($name:ident, $ty_int:ident, $max:expr) => (
            mod $name {
                use std::io::Cursor;
                use {
                    ReadBytesExt, WriteBytesExt,
                    BigEndian, NativeEndian, LittleEndian,
                };
                use super::qc_sized;

                #[test]
                fn big_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut wtr = vec![];
                        wtr.write_as::<BigEndian>(n).unwrap();
                        let mut rdr = Cursor::new(wtr);
                        n == rdr.read_as::<BigEndian>().unwrap()
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max - 1);
                }

                #[test]
                fn little_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut wtr = vec![];
                        wtr.write_as::<LittleEndian>(n).unwrap();
                        let mut rdr = Cursor::new(wtr);
                        n == rdr.read_as::<LittleEndian>().unwrap()
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max - 1);
                }

                #[test]
                fn native_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let mut wtr = vec![];
                        wtr.write_as::<NativeEndian>(n).unwrap();
                        let mut rdr = Cursor::new(wtr);
                        n == rdr.read_as::<NativeEndian>().unwrap()
                    }
                    qc_sized(prop as fn($ty_int) -> bool, $max - 1);
                }
            }
        );
    }

    qc_bytes_ext!(prop_ext_u16, u16, ::std::u16::MAX as u64);
    qc_bytes_ext!(prop_ext_i16, i16, ::std::i16::MAX as u64);
    qc_bytes_ext!(prop_ext_u32, u32, ::std::u32::MAX as u64);
    qc_bytes_ext!(prop_ext_i32, i32, ::std::i32::MAX as u64);
    qc_bytes_ext!(prop_ext_u64, u64, ::std::u64::MAX as u64);
    qc_bytes_ext!(prop_ext_i64, i64, ::std::i64::MAX as u64);
    qc_bytes_ext!(prop_ext_f32, f32, ::std::u64::MAX as u64);
    qc_bytes_ext!(prop_ext_f64, f64, ::std::i64::MAX as u64);
}