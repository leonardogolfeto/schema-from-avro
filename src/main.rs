use rsgen_avro::{Source, Generator};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

fn main() {

    if let Some(arg_input_file) = env::args().nth(1) {

        let file = File::open(arg_input_file).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut raw_schema = String::new();
        buf_reader.read_to_string(&mut raw_schema).unwrap();

        let source = Source::SchemaStr(&raw_schema);

        let mut out = std::io::stdout();

        let g = Generator::new().unwrap();
        g.gen(&source, &mut out).unwrap();
    }

}

