#![feature(test)]

extern crate byteorder;
extern crate test;

macro_rules! bench_num {
    ($ty:ident, $size:expr, $data:expr) => (
        mod $ty {
            use std::$ty;
            use byteorder::{ByteOrder, BigEndian, NativeEndian, LittleEndian};
            use super::test::Bencher;
            use super::test::black_box as bb;

            const NITER: usize = 100_000;

            #[bench]
            fn read_big_endian(b: &mut Bencher) {
                let buf = $data;
                b.iter(|| {
                    for _ in 0..NITER {
                        let val: $ty = BigEndian::from_bytes(bb(buf));
                        bb(val);
                    }
                });
                b.bytes = NITER as u64 * $size as u64
            }

            #[bench]
            fn read_little_endian(b: &mut Bencher) {
                let buf = $data;
                b.iter(|| {
                    for _ in 0..NITER {
                        let val: $ty = LittleEndian::from_bytes(bb(buf));
                        bb(val);
                    }
                });
                b.bytes = NITER as u64 * $size as u64
            }

            #[bench]
            fn read_native_endian(b: &mut Bencher) {
                let buf = $data;
                b.iter(|| {
                    for _ in 0..NITER {
                        let val: $ty = NativeEndian::from_bytes(bb(buf));
                        bb(val);
                    }
                });
                b.bytes = NITER as u64 * $size as u64
            }

            #[bench]
            fn write_big_endian(b: &mut Bencher) {
                let n = $ty::MAX;
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(BigEndian::into_bytes(bb(n)));
                    }
                });
                b.bytes = NITER as u64 * $size as u64
            }

            #[bench]
            fn write_little_endian(b: &mut Bencher) {
                let n = $ty::MAX;
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(LittleEndian::into_bytes(bb(n)));
                    }
                });
                b.bytes = NITER as u64 * $size as u64
            }

            #[bench]
            fn write_native_endian(b: &mut Bencher) {
                let n = $ty::MAX;
                b.iter(|| {
                    for _ in 0..NITER {
                        bb(NativeEndian::into_bytes(bb(n)));
                    }
                });
                b.bytes = NITER as u64 * $size as u64
            }
        }
    );
}

bench_num!(u16, 2, [1, 2]);
bench_num!(i16, 2, [1, 2]);
bench_num!(u32, 4, [1, 2, 3, 4]);
bench_num!(i32, 4, [1, 2, 3, 4]);
bench_num!(u64, 8, [1, 2, 3, 4, 5, 6, 7, 8]);
bench_num!(i64, 8, [1, 2, 3, 4, 5, 6, 7, 8]);
bench_num!(f32, 4, [1, 2, 3, 4]);
bench_num!(f64, 8, [1, 2, 3, 4, 5, 6, 7, 8]);