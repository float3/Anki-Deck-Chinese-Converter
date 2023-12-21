use character_converter::{is_simplified, simplified_to_traditional};
use std::env;
use std::io::{BufReader, Read};
use std::{fs::File, io::Write};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file = File::open(args[1].clone()).unwrap();
    let mut buf_reader = BufReader::new(file.try_clone().unwrap());
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    contents = contents
        .split('\t')
        .map(|x| {
            if is_simplified(x) {
                simplified_to_traditional(x).to_string()
            } else {
                x.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\t");
    let mut file = File::create("output.txt").unwrap();
    let _ = file.write(contents.as_bytes());
}
