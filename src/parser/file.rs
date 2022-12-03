use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process<F, G, Event>(
    file_name: &str,
    mut process_line: F,
    mut process_event: G,
) -> Result<(), Box<dyn Error>>
where
    F: FnMut(String) -> Result<Event, Box<dyn Error>>,
    G: FnMut(Event) -> Result<(), Box<dyn Error>>,
{
    let file = File::open(file_name).expect("input file should exist");
    let file_buffer = BufReader::new(file);
    for line in file_buffer.lines() {
        match line {
            Ok(l) => process_event(process_line(l)?)?,
            Err(e) => panic!("cannot read line from filepath={}: {}", file_name, e),
        }
    }
    Ok(())
}
