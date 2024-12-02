use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use std::io::Read;

use crate::hybrid_hash_map::HybridMapInterface;
use crate::hybrid_hash_map::HybridMapType;
// use crate::hybrid_hash_map::TableMap;

#[derive(Default)]
struct StringData {
    bytes: Vec<u8>,
    size: Vec<usize>,
}

// type FreqMap<K, V> = crate::hybrid_hash_map::HybridHash<K, V, fnv_rs::FnvBuildHasher>;
// type FreqMapTable<K> = TableMap<K, u32, fnv_rs::FnvBuildHasher>;
type FreqMapType<K> = <K as HybridMapType<u32, fnv_rs::FnvBuildHasher>>::Out;

// #[derive(Default)]
struct ColumnData(StringData, Vec<i16>, Vec<i16>, Vec<i16>, Vec<i64>, Vec<i64>, () /*Vec<f32>*/, Vec<i64>);

impl Default for ColumnData {
    fn default() -> ColumnData {
        ColumnData {
            0: StringData::default(),
            1: Vec::with_capacity(10000),
            2: Vec::with_capacity(10000),
            3: Vec::with_capacity(10000),
            4: Vec::with_capacity(10000),
            5: Vec::with_capacity(10000),
            6: (),
            7: Vec::with_capacity(10000),
        }
    }
}

#[derive(Default)]
struct Row(Vec<u8>, i16, i16, i16, i64, i64, () /*Vec<f32>*/, i64);

#[derive(Default)]
struct ColumnStats(
    (),
    FreqMapType<i16>,
    FreqMapType<i16>,
    FreqMapType<i16>,
    FreqMapType<i64>,
    FreqMapType<i64>,
    (),
    FreqMapType<i64>,
);

#[inline(never)]
fn parse(column: usize, data_slice: &[u8], current_row: &mut Row) -> Result<(), Box<dyn Error>> {
    match column {
        0 => {
            // current_row.0.clear();
            // current_row.0.extend_from_slice(data_slice)
        }
        1 => {
            current_row.1 = data_slice[0].into(); // !!! could be empty
        }
        2 => {
            current_row.2 = data_slice[0].into();
        }
        3 => match atoi_simd::parse::<u8>(data_slice) {
            Ok(val) => {
                current_row.3 = val as i16;
            }
            Err(e) => return Err(e.to_string().into()),
        },
        4 => match atoi_simd::parse::<u32>(data_slice) {
            Ok(val) => {
                current_row.4 = val as i64;
            }
            Err(e) => return Err(e.to_string().into()),
        },
        5 => match atoi_simd::parse::<u32>(data_slice) {
            Ok(val) => {
                current_row.5 = val as i64;
            }
            Err(e) => return Err(e.to_string().into()),
        },
        6 => {}

        7 => match atoi_simd::parse::<u32>(data_slice) {
            Ok(val) => {
                current_row.7 = val as i64;
            }
            Err(e) => return Err(e.to_string().into()),
        },
        _ => return Err(format!("Invalid Column= {column}").into()),
    };
    Ok(())
}

#[inline(never)]
fn process_row(
    previous_row: &mut Row,
    current_row: &mut Row,
    column_data: &mut ColumnData,
    column_stats: &mut ColumnStats,
) -> Result<(), Box<dyn Error>> {
    for column in 0..8 {
        match column {
            0 => {
                // column_data.0.bytes.extend_from_slice(&current_row.0);
                // column_data.0.size.push(current_row.0.len());
            }
            1 => {
                column_data.1.push(current_row.1 - previous_row.1);
                column_stats.1.bump(current_row.1 - previous_row.1);
            }
            2 => {
                column_data.2.push(current_row.2 - previous_row.2);
                column_stats.2.bump(current_row.2 - previous_row.2);
            }
            3 => {
                column_data.3.push(current_row.3 - previous_row.3);
                column_stats.3.bump(current_row.3 - previous_row.3);
            }
            4 => {
                column_data.4.push(current_row.4 - previous_row.4);
                column_stats.4.bump(current_row.4 - previous_row.4);
            }
            5 => {
                column_data.5.push(current_row.5 - previous_row.5);
                column_stats.5.bump(current_row.5 - previous_row.5);
            }
            6 => {}

            7 => {
                column_data.7.push(current_row.7 - previous_row.7);
                column_stats.7.bump(current_row.7 - previous_row.7);
            }
            _ => return Err(format!("Invalid Column= {column}").into()),
        };
    }
    Ok(())
}

pub fn process_file(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(path)?;
    // let mut buf_reader = BufReader::new(file);

    let num_columns = 8;

    let mut data = vec![];
    file.read_to_end(&mut data)?;
    let data = data;

    let mut column_data: ColumnData = Default::default();
    let mut column_stats: ColumnStats = Default::default();
    let mut previous_row: Row = Default::default();
    let mut current_row: Row = Default::default();

    let mut current_line = 0;

    let mut left_index = 0;

    loop {
        for column in 0..num_columns - 1 {
            let right_index = left_index
                + memchr::memchr(b',', &data[left_index..])
                    .ok_or_else(|| format!("Missing ',' on line {current_line}"))?;

            parse(column, &data[left_index..right_index], &mut current_row)?;
            left_index = right_index + 1;
        }

        let mut right_index = left_index
            + memchr::memchr(b'\n', &data[left_index..])
                .ok_or_else(|| format!("Missing '\n' on line {current_line}"))?;

        let done = right_index == data.len() - 1;

        if data[right_index - 1] == b'\r' {
            right_index -= 1;
        }

        parse(num_columns - 1, &data[left_index..right_index], &mut current_row)?;

        process_row(&mut previous_row, &mut current_row, &mut column_data, &mut column_stats)?;

        if done {
            break;
        }
        left_index = right_index + 1;

        current_line += 1;
        std::mem::swap(&mut current_row, &mut previous_row);
    }

    // println!("{:?}", column_stats.7.hash_map);
    // println!("{:?}", column_stats.4.hash_map.len());
    // println!("{:?}", column_stats.5.hash_map.len());
    // println!("{:?}", column_stats.7.hash_map.len());

    // println!("{:?}", column_stats.4.table);
    // println!("{:?}", column_stats.5.table);
    // println!("{:?}", column_stats.7.table);

    Ok(())
}
