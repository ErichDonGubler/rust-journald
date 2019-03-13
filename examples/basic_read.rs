extern crate journald;

use journald::reader::{JournalReader, JournalReaderConfig};

fn main() {
    let mut journal =
        JournalReader::open(&JournalReaderConfig::default()).expect("journal open failed");

    println!("Trying to read first 10 entries of all journal files...");
    for _ in 0..10 {
        match journal.next_entry().expect("reading next entry failed") {
            Some(entry) => println!("Entry: {:#?}", entry),
            None => println!("End of journal reached."),
        }
    }
}
