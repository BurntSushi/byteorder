#![crate_name = "byteorder"]
#![doc(html_root_url = "http://burntsushi.net/rustdoc/byteorder")]

#![feature(io)]
#![allow(dead_code, unused_variables)]

use std::old_io::IoResult;

// A trivial logging macro. No reason to pull in `log`, which has become
// difficult to use in tests.
macro_rules! lg {
    ($($arg:tt)*) => ({
        let _ = ::std::old_io::stderr().write_str(&*format!($($arg)*));
        let _ = ::std::old_io::stderr().write_str("\n");
    });
}

pub trait ByteOrder {
    fn read_u16(bs: &[u8]) -> u16;
    fn read_u32(bs: &[u8]) -> u32;
    fn read_u64(bs: &[u8]) -> u64;
    fn write_u16(bs: &mut [u8], n: u16);
    fn write_u32(bs: &mut [u8], n: u32);
    fn write_u64(bs: &mut [u8], n: u64);

    fn read_i16(bs: &[u8]) -> i16 {
        <Self as ByteOrder>::read_u16(bs) as i16
    }

    fn write_i16(bs: &mut [u8], n: i16) {
        <Self as ByteOrder>::write_u16(bs, n as u16)
    }

    fn read_i32(bs: &[u8]) -> i32 {
        <Self as ByteOrder>::read_u32(bs) as i32
    }

    fn write_i32(bs: &mut [u8], n: i32) {
        <Self as ByteOrder>::write_u32(bs, n as u32)
    }

    fn read_i64(bs: &[u8]) -> i64 {
        <Self as ByteOrder>::read_u64(bs) as i64
    }

    fn write_i64(bs: &mut [u8], n: i64) {
        <Self as ByteOrder>::write_u64(bs, n as u64)
    }
}

pub trait ReaderBytesExt: Reader + Sized {
    fn read_u8(&mut self) -> IoResult<u8> {
        let mut bs = &mut [0; 1];
        try!(read_full(self, bs));
        Ok(bs[0])
    }

    fn read_i8(&mut self) -> IoResult<i8> {
        let mut bs = &mut [0; 1];
        try!(read_full(self, bs));
        Ok(bs[0] as i8)
    }

    fn read_u16<T: ByteOrder>(&mut self) -> IoResult<u16> {
        let mut bs = &mut [0; 2];
        try!(read_full(self, bs));
        Ok(<T as ByteOrder>::read_u16(bs))
    }

    fn read_i16<T: ByteOrder>(&mut self) -> IoResult<i16> {
        let mut bs = &mut [0; 2];
        try!(read_full(self, bs));
        Ok(<T as ByteOrder>::read_i16(bs))
    }

    fn read_u32<T: ByteOrder>(&mut self) -> IoResult<u32> {
        let mut bs = &mut [0; 4];
        try!(read_full(self, bs));
        Ok(<T as ByteOrder>::read_u32(bs))
    }

    fn read_i32<T: ByteOrder>(&mut self) -> IoResult<i32> {
        let mut bs = &mut [0; 4];
        try!(read_full(self, bs));
        Ok(<T as ByteOrder>::read_i32(bs))
    }

    fn read_u64<T: ByteOrder>(&mut self) -> IoResult<u64> {
        let mut bs = &mut [0; 8];
        try!(read_full(self, bs));
        Ok(<T as ByteOrder>::read_u64(bs))
    }

    fn read_i64<T: ByteOrder>(&mut self) -> IoResult<i64> {
        let mut bs = &mut [0; 8];
        try!(read_full(self, bs));
        Ok(<T as ByteOrder>::read_i64(bs))
    }
}

impl<R: Reader> ReaderBytesExt for R {}

fn read_full<R: Reader>(rdr: &mut R, buf: &mut [u8]) -> IoResult<()> {
    let mut n = 0us;
    while n < buf.len() {
        n += try!(rdr.read(&mut buf[n..]));
    }
    Ok(())
}

pub trait WriterBytesExt: Writer + Sized {
    fn write_u8(&mut self, n: u8) -> IoResult<()> {
        self.write_all(&[n])
    }

    fn write_i8(&mut self, n: i8) -> IoResult<()> {
        self.write_all(&[n as u8])
    }

    fn write_u16<T: ByteOrder>(&mut self, n: u16) -> IoResult<()> {
        let mut bs = &mut [0; 2];
        <T as ByteOrder>::write_u16(bs, n);
        self.write_all(bs)
    }

    fn write_i16<T: ByteOrder>(&mut self, n: i16) -> IoResult<()> {
        let mut bs = &mut [0; 2];
        <T as ByteOrder>::write_i16(bs, n);
        self.write_all(bs)
    }

    fn write_u32<T: ByteOrder>(&mut self, n: u32) -> IoResult<()> {
        let mut bs = &mut [0; 4];
        <T as ByteOrder>::write_u32(bs, n);
        self.write_all(bs)
    }

    fn write_i32<T: ByteOrder>(&mut self, n: i32) -> IoResult<()> {
        let mut bs = &mut [0; 4];
        <T as ByteOrder>::write_i32(bs, n);
        self.write_all(bs)
    }

    fn write_u64<T: ByteOrder>(&mut self, n: u64) -> IoResult<()> {
        let mut bs = &mut [0; 8];
        <T as ByteOrder>::write_u64(bs, n);
        self.write_all(bs)
    }

    fn write_i64<T: ByteOrder>(&mut self, n: i64) -> IoResult<()> {
        let mut bs = &mut [0; 8];
        <T as ByteOrder>::write_i64(bs, n);
        self.write_all(bs)
    }
}

impl<W: Writer> WriterBytesExt for W {}

#[allow(missing_copy_implementations)] pub enum BigEndian {}
#[allow(missing_copy_implementations)] pub enum LittleEndian {}

impl ByteOrder for BigEndian {
    fn read_u16(bs: &[u8]) -> u16 {
        ((bs[0] as u16) << 8) | (bs[1] as u16)
    }

    fn read_u32(bs: &[u8]) -> u32 {
        (bs[0] as u32) << 24
        | (bs[1] as u32) << 16
        | (bs[2] as u32) << 8
        | (bs[3] as u32)
    }

    fn read_u64(bs: &[u8]) -> u64 {
        (bs[0] as u64) << 56
        | (bs[1] as u64) << 48
        | (bs[2] as u64) << 40
        | (bs[3] as u64) << 32
        | (bs[4] as u64) << 24
        | (bs[5] as u64) << 16
        | (bs[6] as u64) << 8
        | (bs[7] as u64)
    }

    fn write_u16(bs: &mut [u8], n: u16) {
        bs[0] = (n >> 8) as u8;
        bs[1] = n as u8;
    }

    fn write_u32(bs: &mut [u8], n: u32) {
        bs[0] = (n >> 24) as u8;
        bs[1] = (n >> 16) as u8;
        bs[2] = (n >> 8) as u8;
        bs[3] = n as u8;
    }

    fn write_u64(bs: &mut [u8], n: u64) {
        bs[0] = (n >> 56) as u8;
        bs[1] = (n >> 48) as u8;
        bs[2] = (n >> 40) as u8;
        bs[3] = (n >> 32) as u8;
        bs[4] = (n >> 24) as u8;
        bs[5] = (n >> 16) as u8;
        bs[6] = (n >> 8) as u8;
        bs[7] = n as u8;
    }
}

impl ByteOrder for LittleEndian {
    fn read_u16(bs: &[u8]) -> u16 {
        bs[0] as u16 | (bs[1] as u16) << 8
    }

    fn read_u32(bs: &[u8]) -> u32 {
        (bs[0] as u32)
        | (bs[1] as u32) << 8
        | (bs[2] as u32) << 16
        | (bs[3] as u32) << 24
    }

    fn read_u64(bs: &[u8]) -> u64 {
        (bs[0] as u64)
        | (bs[1] as u64) << 8
        | (bs[2] as u64) << 16
        | (bs[3] as u64) << 24
        | (bs[4] as u64) << 32
        | (bs[5] as u64) << 40
        | (bs[6] as u64) << 48
        | (bs[7] as u64) << 56
    }

    fn write_u16(bs: &mut [u8], n: u16) {
        bs[0] = n as u8;
        bs[1] = (n >> 8) as u8;
    }

    fn write_u32(bs: &mut [u8], n: u32) {
        bs[0] = n as u8;
        bs[1] = (n >> 8) as u8;
        bs[2] = (n >> 16) as u8;
        bs[3] = (n >> 24) as u8;
    }

    fn write_u64(bs: &mut [u8], n: u64) {
        bs[0] = n as u8;
        bs[1] = (n >> 8) as u8;
        bs[2] = (n >> 16) as u8;
        bs[3] = (n >> 24) as u8;
        bs[4] = (n >> 32) as u8;
        bs[5] = (n >> 40) as u8;
        bs[6] = (n >> 48) as u8;
        bs[7] = (n >> 56) as u8;
    }
}

#[cfg(test)]
mod test {
    extern crate quickcheck;

    macro_rules! qc_byte_order {
        ($name:ident, $ty_int:ty, $read:ident, $write:ident) => (
            mod $name {
                use test::quickcheck::quickcheck;
                use {BigEndian, ByteOrder, LittleEndian};

                #[test]
                fn big_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let bs = &mut [0; 8];
                        <BigEndian as ByteOrder>::$write(bs, n);
                        n == <BigEndian as ByteOrder>::$read(bs)
                    }
                    quickcheck(prop as fn($ty_int) -> bool);
                }

                #[test]
                fn little_endian() {
                    fn prop(n: $ty_int) -> bool {
                        let bs = &mut [0; 8];
                        <LittleEndian as ByteOrder>::$write(bs, n);
                        n == <LittleEndian as ByteOrder>::$read(bs)
                    }
                    quickcheck(prop as fn($ty_int) -> bool);
                }
            }
        );
    }

    qc_byte_order!(prop_u16, u16, read_u16, write_u16);
    qc_byte_order!(prop_i16, i16, read_i16, write_i16);
    qc_byte_order!(prop_u32, u32, read_u32, write_u32);
    qc_byte_order!(prop_i32, i32, read_i32, write_i32);
    qc_byte_order!(prop_u64, u64, read_u64, write_u64);
    qc_byte_order!(prop_i64, i64, read_i64, write_i64);
}
