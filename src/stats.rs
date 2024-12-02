use fnv_rs::FnvHashMap;
// use rustc_hash::FxHashMap;
use std::collections::HashMap;
// use std::fs::File;
// use std::io::{BufReader, Read};
// use std::path::PathBuf;
// use identity_hash::IdentityHasher;
// use std::{collections::HashMap, hash::BuildHasherDefault};

// use ahash::AHashMap;
// use ahash::{AHasher, RandomState};
// use std::collections::HashMap;

#[derive(Debug)]
pub struct Stats {
    pub frequency_map: Vec<u32>,
    pub frequency_map_hash: FreqMap<u8, u32>,
    pub bytes: u32,
}

// type FreqMap<K, V> = FxHashMap<K, V>;
type FreqMap<K, V> = FnvHashMap<K, V>;
// type FreqMap<K, V> = AHashMap<K, V>;
// type FreqMap<K, V> = HashMap<K, V, RandomState>;
// type FreqMap<K, V> = HashMap<K, V, BuildHasherDefault<IdentityHasher<K>>>;

impl Stats {
    pub fn from_chunk(chunk: &Vec<u8>) -> Self {
        // let mut stats = Stats { frequency_map: FreqMap::default(), bytes: 0 };
        let mut stats = Stats {
            frequency_map_hash: HashMap::with_capacity_and_hasher(100, Default::default()),
            // frequency_map_hash: HashMap::with_capacity_and_hasher(100, Default::default()),
            frequency_map: vec![0; 65535],
            bytes: 0,
        };

        for &byte in chunk {
            if (byte as u16) < 65535 {
                stats.frequency_map[byte as usize] += 1;
            } else {
                let stat_entry = stats.frequency_map_hash.entry(byte).or_insert(0u32);
                *stat_entry += 1u32;
            }
            stats.bytes += 1;
        }

        stats
    }

    // pub fn entropy(&self) -> f64 {
    //     let mut sum = 0.0;
    //     for (_byte, count) in &self.frequency_map {
    //         let freq = *count as f64 / self.bytes as f64;
    //         sum += freq.log2() * freq;
    //     }
    //     -sum
    // }
}
