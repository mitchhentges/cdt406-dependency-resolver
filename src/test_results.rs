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

pub struct CsvTestSource {
    filename: &'static str
}

impl CsvTestSource {
    pub fn new(filename: &'static str) -> CsvTestSource {
        CsvTestSource { filename: filename }
    }
}

impl TestSource for CsvTestSource {
    fn read_tests(&self) -> TestResults {
        TestResults::new(Box::new([Box::new([true]), Box::new([false])]))
    }
}