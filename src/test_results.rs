use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead, Read};

#[derive(Debug, PartialEq, Eq)]
pub struct Test {
    pub id: i32,
    pub name: String,
    pub executions: Vec<bool>,
}

impl Test {
    pub fn new(id: i32, name: String, executions: Vec<bool>) -> Test {
        Test { id: id, name: name, executions: executions }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AllTestResults {
    pub results: Vec<Test>,
}

impl AllTestResults {
    pub fn new(results: Vec<Test>) -> AllTestResults {
        AllTestResults{ results: results }
    }
}

pub trait TestSource {
    fn read_tests(&self) -> Result<AllTestResults, String>;
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
    fn read_tests(&self) -> Result<AllTestResults, String> {
        let file = try!(File::open(self.filename).map_err(|e| e.to_string()));
        let result = parse(BufReader::new(file));
        Ok(AllTestResults::new(result.unwrap()))
    }
}

pub fn parse<T: Read>(reader: BufReader<T>) -> Result<Vec<Test>, ParseError> {
    let mut all_results: Vec<Test> = Vec::new();
    let mut next_test_id = 0;
    for line in reader.lines() {
        let line = try!(line);

        let values: Vec<String> = line.split(',')
            .map(|value| value.to_owned())
            .collect();

        if values.len() == 1 {
            return Err(ParseError::NoTestExecutions);
        }

        if !values.iter()
            .skip(1)
            .map(|value| value.trim())
            .all(|value| value == "0" || value == "1") {
            return Err(ParseError::InvalidFormat);
        }

        let test_name = values[0].to_owned();
        let executions: Vec<bool> = values.iter()
            .skip(1)
            .map(|result| result.trim() == "1")
            .collect();

        all_results.push(Test::new(next_test_id, test_name, executions));
        next_test_id += 1;
    }
    Ok(all_results)
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    Io,
    NoTestExecutions,
    InvalidFormat
}

impl From<io::Error> for ParseError {
    fn from(_: io::Error) -> ParseError {
        ParseError::Io
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn parse_string(input: &str) -> Result<Vec<Test>, ParseError> {
        let formatted = format!("{}\n", input);
        let reader = BufReader::new(formatted.as_bytes());
        parse(reader)
    }

    fn result(test_name: &str, results: &[bool]) -> Result<Vec<Test>, ParseError> {
        Ok(vec!(test_history(test_name, 0, results)))
    }

    fn test_history(test_name: &str, id: i32, results: &[bool]) -> Test {
        Test::new(id, test_name.to_owned(), results.to_vec())
    }

    #[test]
    fn should_fail_no_name_if_blank_line() {
        assert_eq!(parse_string(""), Err(ParseError::NoTestExecutions));
    }

    #[test]
    fn should_fail_invalid_format_if_not_1_or_0() {
        assert_eq!(parse_string("Test name,A"), Err(ParseError::InvalidFormat));
    }

    #[test]
    fn should_parse_valid_data() {
        assert_eq!(parse_string("Test name,1,0,0,1"), result("Test name", &[true, false, false, true]));
    }

    #[test]
    fn should_parse_valid_data_with_spaces() {
        assert_eq!(parse_string("Test name,1  ,0, 0,1 "), result("Test name", &[true, false, false, true]));
    }

    #[test]
    fn should_assign_increasing_test_id() {
        assert_eq!(parse_string("A,1,0\nB,0,0"), Ok(vec!(
            test_history("A", 0, &[true, false]),
            test_history("B", 1, &[false, false])
        )));
    }
}