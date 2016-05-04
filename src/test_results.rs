use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq)]
pub struct TestResults {
    pub results: Box<[Box<[bool]>]>
}

impl TestResults {
    pub fn new(results: Box<[Box<[bool]>]>) -> TestResults {
        TestResults{ results: results }
    }
}

pub trait TestSource {
    fn read_tests(&self) -> Result<TestResults, String>;
}

pub struct CsvTestSource<'a> {
    filename: &'a str
}

impl<'a> CsvTestSource<'a> {
    pub fn new(filename: &'a str) -> CsvTestSource<'a> {
        CsvTestSource { filename: filename }
    }
}

impl<'a> TestSource for CsvTestSource<'a> {
    fn read_tests(&self) -> Result<TestResults, String> {
        let file = try!(File::open(self.filename).map_err(|e| e.to_string()));
        let reader = BufReader::new(file);
        for line in reader.lines() {
            println!("{}", line.unwrap())
        }
        Ok(TestResults::new(Box::new([Box::new([true]), Box::new([false])])))
    }
}