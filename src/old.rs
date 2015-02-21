use std::old_io::IoResult;

use ByteOrder;

/// Extends `Reader` with methods for reading numbers. (For `std::old_io`.)
///
/// Most of the methods defined here have an unconstrained type parameter that
/// must be explicitly instantiated. Typically, it is instantiated with either
/// the `BigEndian` or `LittleEndian` types defined in this crate.
///
/// # Examples
///
/// Read unsigned 16 bit big-endian integers from a `Reader`:
///
/// ```rust
/// use std::old_io::MemReader;
/// use byteorder::{BigEndian, ReaderBytesExt};
///
/// let mut rdr = MemReader::new(vec![2, 5, 3, 0]);
/// assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
/// assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
/// ```
pub trait ReaderBytesExt: Reader + Sized {
    /// Reads an unsigned 8 bit integer from the underlying reader.
    ///
    /// Note that since this reads a single byte, no byte order conversions
    /// are used. It is included for completeness.
    fn read_u8(&mut self) -> IoResult<u8> {
        let mut buf = [0; 1];
        try!(read_full(self, &mut buf));
        Ok(buf[0])
    }

    /// Reads a signed 8 bit integer from the underlying reader.
    ///
    /// Note that since this reads a single byte, no byte order conversions
    /// are used. It is included for completeness.
    fn read_i8(&mut self) -> IoResult<i8> {
        let mut buf = [0; 1];
        try!(read_full(self, &mut buf));
        Ok(buf[0] as i8)
    }

    /// Reads an unsigned 16 bit integer from the underlying reader.
    fn read_u16<T: ByteOrder>(&mut self) -> IoResult<u16> {
        let mut buf = [0; 2];
        try!(read_full(self, &mut buf));
        Ok(<T as ByteOrder>::read_u16(&buf))
    }

    /// Reads a signed 16 bit integer from the underlying reader.
    fn read_i16<T: ByteOrder>(&mut self) -> IoResult<i16> {
        let mut buf = [0; 2];
        try!(read_full(self, &mut buf));
        Ok(<T as ByteOrder>::read_i16(&buf))
    }

    /// Reads an unsigned 32 bit integer from the underlying reader.
    fn read_u32<T: ByteOrder>(&mut self) -> IoResult<u32> {
        let mut buf = [0; 4];
        try!(read_full(self, &mut buf));
        Ok(<T as ByteOrder>::read_u32(&buf))
    }

    /// Reads a signed 32 bit integer from the underlying reader.
    fn read_i32<T: ByteOrder>(&mut self) -> IoResult<i32> {
        let mut buf = [0; 4];
        try!(read_full(self, &mut buf));
        Ok(<T as ByteOrder>::read_i32(&buf))
    }

    /// Reads an unsigned 64 bit integer from the underlying reader.
    fn read_u64<T: ByteOrder>(&mut self) -> IoResult<u64> {
        let mut buf = [0; 8];
        try!(read_full(self, &mut buf));
        Ok(<T as ByteOrder>::read_u64(&buf))
    }

    /// Reads a signed 64 bit integer from the underlying reader.
    fn read_i64<T: ByteOrder>(&mut self) -> IoResult<i64> {
        let mut buf = [0; 8];
        try!(read_full(self, &mut buf));
        Ok(<T as ByteOrder>::read_i64(&buf))
    }

    /// Reads an unsigned n-bytes integer from the underlying reader.
    fn read_uint<T: ByteOrder>(&mut self, nbytes: usize) -> IoResult<u64> {
        let mut buf = [0; 8];
        try!(read_full(self, &mut buf[0..nbytes]));
        Ok(<T as ByteOrder>::read_uint(&buf, nbytes))
    }

    /// Reads a signed n-bytes integer from the underlying reader.
    fn read_int<T: ByteOrder>(&mut self, nbytes: usize) -> IoResult<i64> {
        let mut buf = [0; 8];
        try!(read_full(self, &mut buf[0..nbytes]));
        Ok(<T as ByteOrder>::read_int(&buf, nbytes))
    }

    /// Reads a IEEE754 single-precision (4 bytes) floating point number from
    /// the underlying reader.
    fn read_f32<T: ByteOrder>(&mut self) -> IoResult<f32> {
        let mut buf = [0; 4];
        try!(read_full(self, &mut buf));
        Ok(<T as ByteOrder>::read_f32(&buf))
    }

    /// Reads a IEEE754 double-precision (8 bytes) floating point number from
    /// the underlying reader.
    fn read_f64<T: ByteOrder>(&mut self) -> IoResult<f64> {
        let mut buf = [0; 8];
        try!(read_full(self, &mut buf));
        Ok(<T as ByteOrder>::read_f64(&buf))
    }
}

/// All types that implement `Reader` get methods defined in `ReaderBytesExt`
/// for free.
impl<R: Reader> ReaderBytesExt for R {}

fn read_full<R: Reader>(rdr: &mut R, buf: &mut [u8]) -> IoResult<()> {
    let mut n = 0usize;
    while n < buf.len() {
        n += try!(rdr.read(&mut buf[n..]));
    }
    Ok(())
}

/// Extends `Writer` with methods for writing numbers. (For `std::old_io`.)
///
/// Most of the methods defined here have an unconstrained type parameter that
/// must be explicitly instantiated. Typically, it is instantiated with either
/// the `BigEndian` or `LittleEndian` types defined in this crate.
///
/// # Examples
///
/// Write unsigned 16 bit big-endian integers to a `Writer`:
///
/// ```rust
/// use byteorder::{BigEndian, WriterBytesExt};
///
/// let mut wtr = vec![];
/// wtr.write_u16::<BigEndian>(517).unwrap();
/// wtr.write_u16::<BigEndian>(768).unwrap();
/// assert_eq!(wtr, vec![2, 5, 3, 0]);
/// ```
pub trait WriterBytesExt: Writer + Sized {
    /// Writes an unsigned 8 bit integer to the underlying writer.
    ///
    /// Note that since this writes a single byte, no byte order conversions
    /// are used. It is included for completeness.
    fn write_u8(&mut self, n: u8) -> IoResult<()> {
        self.write_all(&[n])
    }

    /// Writes a signed 8 bit integer to the underlying writer.
    ///
    /// Note that since this writes a single byte, no byte order conversions
    /// are used. It is included for completeness.
    fn write_i8(&mut self, n: i8) -> IoResult<()> {
        self.write_all(&[n as u8])
    }

    /// Writes an unsigned 16 bit integer to the underlying writer.
    fn write_u16<T: ByteOrder>(&mut self, n: u16) -> IoResult<()> {
        let mut buf = [0; 2];
        <T as ByteOrder>::write_u16(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes a signed 16 bit integer to the underlying writer.
    fn write_i16<T: ByteOrder>(&mut self, n: i16) -> IoResult<()> {
        let mut buf = [0; 2];
        <T as ByteOrder>::write_i16(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes an unsigned 32 bit integer to the underlying writer.
    fn write_u32<T: ByteOrder>(&mut self, n: u32) -> IoResult<()> {
        let mut buf = [0; 4];
        <T as ByteOrder>::write_u32(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes a signed 32 bit integer to the underlying writer.
    fn write_i32<T: ByteOrder>(&mut self, n: i32) -> IoResult<()> {
        let mut buf = [0; 4];
        <T as ByteOrder>::write_i32(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes an unsigned 64 bit integer to the underlying writer.
    fn write_u64<T: ByteOrder>(&mut self, n: u64) -> IoResult<()> {
        let mut buf = [0; 8];
        <T as ByteOrder>::write_u64(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes a signed 64 bit integer to the underlying writer.
    fn write_i64<T: ByteOrder>(&mut self, n: i64) -> IoResult<()> {
        let mut buf = [0; 8];
        <T as ByteOrder>::write_i64(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes a IEEE754 single-precision (4 bytes) floating point number to
    /// the underlying writer.
    fn write_f32<T: ByteOrder>(&mut self, n: f32) -> IoResult<()> {
        let mut buf = [0; 4];
        <T as ByteOrder>::write_f32(&mut buf, n);
        self.write_all(&buf)
    }

    /// Writes a IEEE754 double-precision (8 bytes) floating point number to
    /// the underlying writer.
    fn write_f64<T: ByteOrder>(&mut self, n: f64) -> IoResult<()> {
        let mut buf = [0; 8];
        <T as ByteOrder>::write_f64(&mut buf, n);
        self.write_all(&buf)
    }
}

/// All types that implement `Writer` get methods defined in `WriterBytesExt`
/// for free.
impl<W: Writer> WriterBytesExt for W {}
