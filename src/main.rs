use std::{error::Error, f32::consts::E, path};
use std::time::Instant;

use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for searchresult in reader.records() {
        let record = searchresult?;
        println!("{:?}", record)
    }
    Ok(())
}
fn main() {
    if let Err(e) = read_from_file("./unfriendtech.csv") {
        eprintln!("{}", e)
    }
    let start = Instant::now();
    println!("Elapsed: {:?}", start.elapsed());
}
