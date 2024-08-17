use fnv_rs::FnvHashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Stats {
    pub frequency_map: FreqMap<u8, u32>,
    pub bytes: u32,
}

type FreqMap<K, V> = FnvHashMap<K, V>;

pub fn read_file(path: &PathBuf) -> Stats {
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);

    let mut stats = Stats { frequency_map: FreqMap::default(), bytes: 0 };

    for b in buf_reader.bytes() {
        match b {
            Ok(b) => {
                let stat_entry = stats.frequency_map.entry(b).or_insert(0u32);
                *stat_entry += 1u32;
                stats.bytes += 1;
            }
            Err(_) => panic!("I/O Error"),
        }
    }

    stats
}

impl Stats {
    pub fn entropy(&self) -> f64 {
        let mut sum = 0.0;
        for (_byte, count) in &self.frequency_map {
            let freq = *count as f64 / self.bytes as f64;
            sum += freq.log2() * freq;
        }
        -sum
    }
}
