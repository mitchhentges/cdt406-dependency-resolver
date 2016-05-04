mod test_results;
use test_results::*;

pub fn return_string() -> &'static str {
    return "bork";
}

fn use_array(tests: TestResults) {
    println!("Bork: {:?} [1][0]{}", tests.results, tests.results[1][0]);
    println!("Length: {}", tests.results.len());
    for bork in tests.results.iter() {
        println!("borking");
        for bap in bork.iter() {
            println!("tests[bork][bap]: {}", bap);
        }
    }
}

fn main() {
    let source = CsvTestSource::new("name");
    use_array(source.read_tests());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_is_bork() {
        let s = return_string();
        assert_eq!("bork", s);
    }
}