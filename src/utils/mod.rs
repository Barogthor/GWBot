use std::fs::File;
use std::io::{Cursor, BufReader, BufRead, Lines};

pub trait FileParser{
    fn parse(filename: &str);
}

pub type CSVRecord = Vec<String>;
#[derive(Default)]
pub struct CSVParser {
    headers: Vec<String>,
    records: Vec<CSVRecord>,
}

impl CSVParser {
    fn read_header(&mut self, iter: &mut Lines<BufReader<File>>){
        let header = iter.next().unwrap().unwrap();
        let header: Vec<&str> = header.split(';').collect();
        for x in header {
            self.headers.push(x.to_string());
        }
    }
    fn read_record(&mut self, raw_record: String){
        let split_record: Vec<&str> = raw_record.split(';').collect();
        let mut record = vec![];
        for x in split_record {
            record.push(x.to_string());
        }
        self.records.push(record);
    }
}


impl FileParser for CSVParser{
    fn parse(filename: &str) {
        let mut parser = CSVParser::default();
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut iter = reader.lines();
        parser.read_header(&mut iter);
        iter.for_each(|s| parser.read_record(s.unwrap()));
    }
}

pub enum Internationalization {
    French,
    English,
}