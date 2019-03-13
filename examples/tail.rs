extern crate journald;

use {
    journald::reader::{
        JournalReader,
        JournalReaderConfig,
        JournalSeek,
    },
    std::{
        thread::sleep,
        time::Duration,
    },
};

fn main() {
    let mut journal =
        JournalReader::open(&JournalReaderConfig::default()).expect("journal open failed");

    journal.seek(JournalSeek::Tail).expect("journal seek to tail failed");

    loop {
        match journal.next_entry().expect("reading next entry failed") {
            Some(entry) => println!("Entry: {:#?}", entry),
            None => sleep(Duration::from_secs(1)),
        }
    }
}
