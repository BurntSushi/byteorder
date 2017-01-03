use std::io::{self, Result};

use ByteOrder;

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
/// let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
/// assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
/// assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
/// ```
pub trait ReadBytesExt: io::Read {
    /// Reads an unsigned 8 bit integer from the underlying reader.
    ///
    /// Note that since this reads a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read unsigned 8 bit integers from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    ///
    /// let mut rdr = Cursor::new(vec![2, 5]);
    /// assert_eq!(2, rdr.read_u8().unwrap());
    /// assert_eq!(5, rdr.read_u8().unwrap());
    /// ```
    #[inline]
    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        try!(self.read_exact(&mut buf));
        Ok(buf[0])
    }

    /// Reads a signed 8 bit integer from the underlying reader.
    ///
    /// Note that since this reads a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read unsigned 8 bit integers from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    ///
    /// let mut rdr = Cursor::new(vec![0x02, 0xfb]);
    /// assert_eq!(2, rdr.read_i8().unwrap());
    /// assert_eq!(-5, rdr.read_i8().unwrap());
    /// ```
    #[inline]
    fn read_i8(&mut self) -> Result<i8> {
        let mut buf = [0; 1];
        try!(self.read_exact(&mut buf));
        Ok(buf[0] as i8)
    }

    /// Reads an unsigned 16 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read unsigned 16 bit big-endian integers from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    ///
    /// let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
    /// assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
    /// assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
    /// ```
    #[inline]
    fn read_u16<T: ByteOrder>(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        try!(self.read_exact(&mut buf));
        Ok(T::read_u16(&buf))
    }

    /// Reads a signed 16 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read signed 16 bit big-endian integers from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    ///
    /// let mut rdr = Cursor::new(vec![0x00, 0xc1, 0xff, 0x7c]);
    /// assert_eq!(193, rdr.read_i16::<BigEndian>().unwrap());
    /// assert_eq!(-132, rdr.read_i16::<BigEndian>().unwrap());
    /// ```
    #[inline]
    fn read_i16<T: ByteOrder>(&mut self) -> Result<i16> {
        let mut buf = [0; 2];
        try!(self.read_exact(&mut buf));
        Ok(T::read_i16(&buf))
    }

    /// Reads an unsigned 32 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read unsigned 32 bit big-endian integers from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    ///
    /// let mut rdr = Cursor::new(vec![0x00, 0x00, 0x01, 0x0b]);
    /// assert_eq!(267, rdr.read_u32::<BigEndian>().unwrap());
    /// ```
    #[inline]
    fn read_u32<T: ByteOrder>(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        try!(self.read_exact(&mut buf));
        Ok(T::read_u32(&buf))
    }

    /// Reads a signed 32 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read signed 32 bit big-endian integers from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    ///
    /// let mut rdr = Cursor::new(vec![0xff, 0xff, 0x7a, 0x33]);
    /// assert_eq!(-34253, rdr.read_i32::<BigEndian>().unwrap());
    /// ```
    #[inline]
    fn read_i32<T: ByteOrder>(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        try!(self.read_exact(&mut buf));
        Ok(T::read_i32(&buf))
    }

    /// Reads an unsigned 64 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read an unsigned 64 bit big-endian integer from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    ///
    /// let mut rdr = Cursor::new(vec![0x00, 0x03, 0x43, 0x95, 0x4d, 0x60, 0x86, 0x83]);
    /// assert_eq!(918733457491587, rdr.read_u64::<BigEndian>().unwrap());
    /// ```
    #[inline]
    fn read_u64<T: ByteOrder>(&mut self) -> Result<u64> {
        let mut buf = [0; 8];
        try!(self.read_exact(&mut buf));
        Ok(T::read_u64(&buf))
    }

    /// Reads a signed 64 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read a signed 64 bit big-endian integer from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    ///
    /// let mut rdr = Cursor::new(vec![0x80, 0, 0, 0, 0, 0, 0, 0]);
    /// assert_eq!(i64::min_value(), rdr.read_i64::<BigEndian>().unwrap());
    /// ```
    #[inline]
    fn read_i64<T: ByteOrder>(&mut self) -> Result<i64> {
        let mut buf = [0; 8];
        try!(self.read_exact(&mut buf));
        Ok(T::read_i64(&buf))
    }

    /// Reads an unsigned 128 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read an unsigned 128 bit big-endian integer from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    ///
    /// let mut rdr = Cursor::new(vec![
    ///     0x00, 0x03, 0x43, 0x95, 0x4d, 0x60, 0x86, 0x83,
    ///     0x00, 0x03, 0x43, 0x95, 0x4d, 0x60, 0x86, 0x83
    /// ]);
    /// assert_eq!(16947640962301618749969007319746179, rdr.read_u128::<BigEndian>().unwrap());
    /// ```
    #[cfg(feature = "i128")]
    #[inline]
    fn read_u128<T: ByteOrder>(&mut self) -> Result<u128> {
        let mut buf = [0; 16];
        try!(self.read_exact(&mut buf));
        Ok(T::read_u128(&buf))
    }

    /// Reads a signed 128 bit integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read a signed 128 bit big-endian integer from a `Read`:
    ///
    /// ```rust
    /// #![feature(i128_type)]
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    ///
    /// let mut rdr = Cursor::new(vec![0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    /// assert_eq!(i128::min_value(), rdr.read_i128::<BigEndian>().unwrap());
    /// ```
    #[cfg(feature = "i128")]
    #[inline]
    fn read_i128<T: ByteOrder>(&mut self) -> Result<i128> {
        let mut buf = [0; 16];
        try!(self.read_exact(&mut buf));
        Ok(T::read_i128(&buf))
    }

    /// Reads an unsigned n-bytes integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read an unsigned n-byte big-endian integer from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    ///
    /// let mut rdr = Cursor::new(vec![0x80, 0x74, 0xfa]);
    /// assert_eq!(8418554, rdr.read_uint::<BigEndian>(3).unwrap());
    #[inline]
    fn read_uint<T: ByteOrder>(&mut self, nbytes: usize) -> Result<u64> {
        let mut buf = [0; 8];
        try!(self.read_exact(&mut buf[..nbytes]));
        Ok(T::read_uint(&buf[..nbytes], nbytes))
    }

    /// Reads a signed n-bytes integer from the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read an unsigned n-byte big-endian integer from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    ///
    /// let mut rdr = Cursor::new(vec![0xc1, 0xff, 0x7c]);
    /// assert_eq!(-4063364, rdr.read_int::<BigEndian>(3).unwrap());
    #[inline]
    fn read_int<T: ByteOrder>(&mut self, nbytes: usize) -> Result<i64> {
        let mut buf = [0; 8];
        try!(self.read_exact(&mut buf[..nbytes]));
        Ok(T::read_int(&buf[..nbytes], nbytes))
    }

    /// Reads an unsigned n-bytes integer from the underlying reader.
    #[cfg(feature = "i128")]
    #[inline]
    fn read_uint128<T: ByteOrder>(&mut self, nbytes: usize) -> Result<u128> {
        let mut buf = [0; 16];
        try!(self.read_exact(&mut buf[..nbytes]));
        Ok(T::read_uint128(&buf[..nbytes], nbytes))
    }

    /// Reads a signed n-bytes integer from the underlying reader.
    #[cfg(feature = "i128")]
    #[inline]
    fn read_int128<T: ByteOrder>(&mut self, nbytes: usize) -> Result<i128> {
        let mut buf = [0; 16];
        try!(self.read_exact(&mut buf[..nbytes]));
        Ok(T::read_int128(&buf[..nbytes], nbytes))
    }

    /// Reads a IEEE754 single-precision (4 bytes) floating point number from
    /// the underlying reader.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read a big-endian single-precision floating point number from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    /// use std::f32::consts;
    ///
    /// let mut rdr = Cursor::new(vec![0x40, 0x49, 0x0f, 0xdb]);
    /// assert_eq!(consts::PI, rdr.read_f32::<BigEndian>().unwrap());
    #[inline]
    fn read_f32<T: ByteOrder>(&mut self) -> Result<f32> {
        let mut buf = [0; 4];
        try!(self.read_exact(&mut buf));
        Ok(T::read_f32(&buf))
    }

    /// Reads a IEEE754 double-precision (8 bytes) floating point number from
    /// the underlying reader.
    #[inline]
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Read::read_exact`].
    ///
    /// [`Read::read_exact`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_exact
    ///
    /// # Examples
    ///
    /// Read a big-endian double-precision floating point number from a `Read`:
    ///
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::{BigEndian, ReadBytesExt};
    /// use std::f64::consts;
    ///
    /// let mut rdr = Cursor::new(vec![0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18]);
    /// assert_eq!(consts::PI, rdr.read_f64::<BigEndian>().unwrap());
    fn read_f64<T: ByteOrder>(&mut self) -> Result<f64> {
        let mut buf = [0; 8];
        try!(self.read_exact(&mut buf));
        Ok(T::read_f64(&buf))
    }
}

/// All types that implement `Read` get methods defined in `ReadBytesExt`
/// for free.
impl<R: io::Read + ?Sized> ReadBytesExt for R {}

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
/// let mut wtr = vec![];
/// wtr.write_u16::<BigEndian>(517).unwrap();
/// wtr.write_u16::<BigEndian>(768).unwrap();
/// assert_eq!(wtr, vec![2, 5, 3, 0]);
/// ```
pub trait WriteBytesExt: io::Write {
    /// Writes an unsigned 8 bit integer to the underlying writer.
    ///
    /// Note that since this writes a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    fn write_u8(&mut self, n: u8) -> Result<()> {
        self.write_all(&[n])
    }

    /// Writes a signed 8 bit integer to the underlying writer.
    ///
    /// Note that since this writes a single byte, no byte order conversions
    /// are used. It is included for completeness.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    fn write_i8(&mut self, n: i8) -> Result<()> {
        self.write_all(&[n as u8])
    }

    /// Writes an unsigned 16 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    fn write_u16<T: ByteOrder>(&mut self, n: u16) -> Result<()> {
        let mut buf = [0; 2];
        T::write_u16(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes a signed 16 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    fn write_i16<T: ByteOrder>(&mut self, n: i16) -> Result<()> {
        let mut buf = [0; 2];
        T::write_i16(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes an unsigned 32 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    fn write_u32<T: ByteOrder>(&mut self, n: u32) -> Result<()> {
        let mut buf = [0; 4];
        T::write_u32(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes a signed 32 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    fn write_i32<T: ByteOrder>(&mut self, n: i32) -> Result<()> {
        let mut buf = [0; 4];
        T::write_i32(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes an unsigned 64 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    fn write_u64<T: ByteOrder>(&mut self, n: u64) -> Result<()> {
        let mut buf = [0; 8];
        T::write_u64(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes a signed 64 bit integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    fn write_i64<T: ByteOrder>(&mut self, n: i64) -> Result<()> {
        let mut buf = [0; 8];
        T::write_i64(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes an unsigned 128 bit integer to the underlying writer.
    #[cfg(feature = "i128")]
    #[inline]
    fn write_u128<T: ByteOrder>(&mut self, n: u128) -> Result<()> {
        let mut buf = [0; 16];
        T::write_u128(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes a signed 128 bit integer to the underlying writer.
    #[cfg(feature = "i128")]
    #[inline]
    fn write_i128<T: ByteOrder>(&mut self, n: i128) -> Result<()> {
        let mut buf = [0; 16];
        T::write_i128(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes an unsigned n-bytes integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    ///
    /// # Panics
    ///
    /// If the given integer is not representable in the given number of bytes,
    /// this method panics. If `nbytes > 8`, this method panics.
    #[inline]
    fn write_uint<T: ByteOrder>(
        &mut self,
        n: u64,
        nbytes: usize,
    ) -> Result<()> {
        let mut buf = [0; 8];
        T::write_uint(&mut buf, n, nbytes);
        self.write_all(&buf[0..nbytes])
    }

    /// Writes a signed n-bytes integer to the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    ///
    /// # Panics
    ///
    /// If the given integer is not representable in the given number of bytes,
    /// this method panics. If `nbytes > 8`, this method panics.
    #[inline]
    fn write_int<T: ByteOrder>(
        &mut self,
        n: i64,
        nbytes: usize,
    ) -> Result<()> {
        let mut buf = [0; 8];
        T::write_int(&mut buf, n, nbytes);
        self.write_all(&buf[0..nbytes])
    }

    /// Writes an unsigned n-bytes integer to the underlying writer.
    ///
    /// If the given integer is not representable in the given number of bytes,
    /// this method panics. If `nbytes > 16`, this method panics.
    #[cfg(feature = "i128")]
    #[inline]
    fn write_uint128<T: ByteOrder>(
        &mut self,
        n: u128,
        nbytes: usize,
    ) -> Result<()> {
        let mut buf = [0; 16];
        T::write_uint128(&mut buf, n, nbytes);
        self.write_all(&buf[0..nbytes])
    }

    /// Writes a signed n-bytes integer to the underlying writer.
    ///
    /// If the given integer is not representable in the given number of bytes,
    /// this method panics. If `nbytes > 16`, this method panics.
    #[cfg(feature = "i128")]
    #[inline]
    fn write_int128<T: ByteOrder>(
        &mut self,
        n: i128,
        nbytes: usize,
    ) -> Result<()> {
        let mut buf = [0; 16];
        T::write_int128(&mut buf, n, nbytes);
        self.write_all(&buf[0..nbytes])
    }

    /// Writes a IEEE754 single-precision (4 bytes) floating point number to
    /// the underlying writer.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Write::write_all`].
    ///
    /// [`Write::write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
    #[inline]
    fn write_f32<T: ByteOrder>(&mut self, n: f32) -> Result<()> {
        let mut buf = [0; 4];
        T::write_f32(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes a IEEE754 double-precision (8 bytes) floating point number to
    /// the underlying writer.
    #[inline]
    fn write_f64<T: ByteOrder>(&mut self, n: f64) -> Result<()> {
        let mut buf = [0; 8];
        T::write_f64(&mut buf, n);
        self.write_all(&buf)
    }
}

/// All types that implement `Write` get methods defined in `WriteBytesExt`
/// for free.
impl<W: io::Write + ?Sized> WriteBytesExt for W {}
