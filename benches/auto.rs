#![feature(test)]

extern crate test;

use autobench::AutoMap;
use rand::Rng;
use rustc_hash::FxHashMap;
use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;
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
        }

        #[bench]
        fn $hashmap_name(b: &mut Bencher) {
            let mut auto = FxHashMap::default();
            for i in 0..$size {
                auto.insert(i, i);
            }
        }

        #[bench]
        fn $vec_name(b: &mut Bencher) {
            let mut auto = VecMap::new();
            for i in 0..$size {
                auto.insert(i, i);
            }
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

bench_size!(2, bench_2_auto, bench_2_hashmap, bench_2_vec);
bench_size!(4, bench_4_auto, bench_4_hashmap, bench_4_vec);
bench_size!(8, bench_8_auto, bench_8_hashmap, bench_8_vec);
bench_size!(16, bench_16_auto, bench_16_hashmap, bench_16_vec);
bench_size!(32, bench_32_auto, bench_32_hashmap, bench_32_vec);
bench_size!(64, bench_64_auto, bench_64_hashmap, bench_64_vec);
bench_size!(128, bench_128_auto, bench_128_hashmap, bench_128_vec);
bench_size!(256, bench_256_auto, bench_256_hashmap, bench_256_vec);
