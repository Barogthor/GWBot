use std::fs::File;
use std::io::{BufReader, BufRead, Lines};

pub trait FileParser{
    fn parse(path: &str) -> Self;
}

pub type CSVRecord = Vec<String>;
#[derive(Default, Debug)]
pub struct CSVFile {
    headers: Vec<String>,
    records: Vec<CSVRecord>,
}

impl CSVFile {
    fn read_header(&mut self, iter: &mut Lines<BufReader<File>>){
        let header = iter.next().unwrap().unwrap();
        let header: Vec<&str> = header.split(';').collect();
        for x in header {
            self.headers.push(x.trim().to_string());
        }
    }
    fn read_record(&mut self, raw_record: String){
        let split_record: Vec<&str> = raw_record.split(';').collect();
        let mut record = vec![];
        for x in split_record {
            record.push(x.trim().to_string());
        }
        self.records.push(record);
    }
}

impl FileParser for CSVFile {
    fn parse(path: &str) -> Self {
        let mut csv = CSVFile::default();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut iter = reader.lines();
        csv.read_header(&mut iter);
        iter.for_each(|s| csv.read_record(s.unwrap()));
        csv
    }
}



#[derive(Debug)]
pub struct ZaishenQuestData {
    pub name: String
}
#[derive(Debug)]
pub struct ZaishenQuestStore(Vec<ZaishenQuestData>);

impl ZaishenQuestStore {
    pub fn from_csv(path: &str) -> Self{
        let csv = CSVFile::parse(path);
        let mut store = Self{ 0: vec![] };
        for x in csv.records {
            let name = x.get(1).unwrap().to_string();
            store.0.push(ZaishenQuestData{name});
        }
        store
    }

    pub fn get_from_id(&self, id: i64) -> Option<&ZaishenQuestData> {
        self.0.get(id as usize)
    }
}



