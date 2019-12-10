use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn post_processor<'a>() -> Box<dyn Fn(String) + 'a> {
    Box::new(move |data: String| {
        io::stdout()
            .write_all(data.as_bytes())
            .expect("Failed to write to stdout");
    })
}

pub fn write_on_disk<'a>(input: &'a str) -> Box<dyn Fn(String) + 'a> {
    File::create(input).expect("Fail to create output file");

    Box::new(move |data: String| {
        let file_location = Path::new(input.clone());
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_location)
            .expect("Fail to open file");

        file.write_all(data.as_bytes())
            .expect("Failed to write to file");
    })
}
