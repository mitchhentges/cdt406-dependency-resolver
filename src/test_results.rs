use std::fs::File;
use std::io::{BufReader, BufRead, Read};

#[derive(Debug, PartialEq, Eq)]
pub struct Test {
    pub name: String,
    pub executions: Vec<bool>,
}

impl Test {
    pub fn new(name: String, executions: Vec<bool>) -> Test {
        Test { name: name, executions: executions }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TestResults {
    pub results: Vec<Test>,
}

impl TestResults {
    pub fn new(results: Vec<Test>) -> TestResults {
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
        let result = parse(BufReader::new(file));
        Ok(TestResults::new(result.unwrap()))
    }
}

pub fn parse<T: Read>(reader: BufReader<T>) -> Result<Vec<Test>, String> {
    let mut all_results: Vec<Test> = Vec::new();
    for line in reader.lines() {
        if line.is_err() {
            println!("Error occurred while reading file, using input up to here");
            println!("\t{}", line.unwrap_err().to_string());
            break;
        }

        let line = line.unwrap();
        let test_executions = line.matches(',').count(); // First chunk is the test name

        if test_executions < 1 { // There's not even a single test
            println!("No test executions for test {}", line);
            continue;
        }

        let test_name = line[0..line.find(',').unwrap()].to_owned();
        let results: Vec<bool> = line.split(',')
        .skip(1)
        .map(|result| result == "0")
        .collect();
        let test = Test::new(test_name, results);
        all_results.push(test);
    }
    Ok(all_results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, BufRead, Read};

    #[test]
    fn bork() {
        let reader = BufReader::new("A,0,1".as_bytes());
        let result = parse(reader);
        println!("{:?}", result);
    }
}