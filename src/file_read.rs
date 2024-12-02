use crate::huffman_encode::make_symbol_table;
use crate::stats::Stats;
use crossbeam_channel;
use crossbeam_channel::bounded;
use crossbeam_utils::thread;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

const DEFAULT_BUF_SIZE: usize = 1028 * 512;

struct FileChunks {
    file: std::fs::File,
    buf_size: usize,
}
impl FileChunks {
    fn from_path(path: &PathBuf) -> std::io::Result<Self> {
        Ok(FileChunks { file: File::open(path)?, buf_size: DEFAULT_BUF_SIZE })
    }

    fn get_chunk(&mut self) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0; self.buf_size];
        let bytes_read = self.file.read(&mut buf)?;
        buf.truncate(bytes_read);

        Ok(buf)
    }
}

pub fn process_file(path: &PathBuf) -> Result<(), std::io::Error> {
    let (source, sink) = bounded(5);

    thread::scope(|s| {
        s.spawn(|_| {
            let mut chunks = FileChunks::from_path(&path).unwrap();
            loop {
                match chunks.get_chunk() {
                    Ok(r) => {
                        if r.is_empty() {
                            drop(source);
                            break;
                        }
                        source.send(r).unwrap();
                    }
                    Err(_e) => {}
                }
            }
        });

        thread::scope(|s| {
            let num_threads = std::thread::available_parallelism().unwrap().get();
            for _ in 0..num_threads {
                s.spawn(|_| loop {
                    match sink.recv() {
                        Ok(chunk) => {
                            let stats = Stats::from_chunk(&chunk);
                            let _table = make_symbol_table(&stats);
                            // for (byte, &bin_rep) in table.iter().enumerate() {
                            //     if bin_rep == 0 {
                            //         continue;
                            //     }
                            //     // println!("byte:{:?}, bin_rep:{:#018b}", byte, bin_rep);
                            // }
                            // println!();
                        }
                        Err(_e) => {
                            break;
                        }
                    }
                });
            }
        })
        .unwrap();
    })
    .unwrap();

    Ok(())
}
