#![feature(test)]

extern crate test;

use autobench::AutoMap;
use rand::Rng;
use rustc_hash::FxHashMap;
use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;
use std::hint::black_box;
use test::Bencher;
use vecmap::VecMap;

macro_rules! bench_size {
    ($size:expr, $auto_name:ident, $hashmap_name:ident, $vec_name:ident) => {
        #[bench]
        fn $auto_name(b: &mut Bencher) {
            let mut auto = AutoMap::<_, _, BuildHasherDefault<FxHasher>>::default();
            for i in 0..$size {
                auto.insert(i, random_string());
            }

            b.iter(|| {
                for i in 0..100000 {
                    black_box(auto.get(&i));
                }
            });
        }

        #[bench]
        fn $hashmap_name(b: &mut Bencher) {
            let mut auto = FxHashMap::default();
            for i in 0..$size {
                auto.insert(i, random_string());
            }

            b.iter(|| {
                for i in 0..100000 {
                    black_box(auto.get(&i));
                }
            });
        }

        #[bench]
        fn $vec_name(b: &mut Bencher) {
            let mut auto = VecMap::new();
            for i in 0..$size {
                auto.insert(i, random_string());
            }

            b.iter(|| {
                for i in 0..100000 {
                    black_box(auto.get(&i));
                }
            });
        }
    };
}

fn random_string() -> String {
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    for _ in 0..10 {
        s.push(rng.gen_range(0..10).to_string().chars().next().unwrap());
    }
    s
}

bench_size!(2, bench_002_auto, bench_002_hashmap, bench_002_vec);
bench_size!(4, bench_004_auto, bench_004_hashmap, bench_004_vec);
bench_size!(8, bench_008_auto, bench_008_hashmap, bench_008_vec);
bench_size!(16, bench_016_auto, bench_016_hashmap, bench_016_vec);
bench_size!(32, bench_032_auto, bench_032_hashmap, bench_032_vec);
bench_size!(64, bench_064_auto, bench_064_hashmap, bench_064_vec);
bench_size!(128, bench_128_auto, bench_128_hashmap, bench_128_vec);
bench_size!(256, bench_256_auto, bench_256_hashmap, bench_256_vec);
