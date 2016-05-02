trait TestSource {
    fn read_tests<'a>() -> &'a[bool];
}

pub struct CsvTestSource {
    pub filename: &'static str
}

impl TestSource for CsvTestSource {
    fn read_tests<'a>() -> &'a[bool] {
        &[true, false, true]
    }
}