pub trait TestSource {
    fn read_tests(&self) -> Box<[Box<[bool]>]>;
}

pub struct CsvTestSource {
    pub filename: &'static str
}

impl TestSource for CsvTestSource {
    fn read_tests(&self) -> Box<[Box<[bool]>]> {
        Box::new([Box::new([true]), Box::new([false])])
    }
}