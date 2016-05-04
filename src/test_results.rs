pub struct TestResults {
    pub results: Box<[Box<[bool]>]>
}

impl TestResults {
    pub fn new(results: Box<[Box<[bool]>]>) -> TestResults {
        TestResults{ results: results }
    }
}

pub trait TestSource {
    fn read_tests(&self) -> TestResults;
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
    fn read_tests(&self) -> TestResults {
        TestResults::new(Box::new([Box::new([true]), Box::new([false])]))
    }
}