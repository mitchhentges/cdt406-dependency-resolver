mod test_results;

pub fn return_string() -> &'static str {
    return "bork";
}

fn use_array(tests: &[&[bool]]) {
    println!("Bork: {:?} [3][4]{}", tests, tests[3][2]);
    println!("Length: {}", tests.len());
    /*for bork in tests {
        println!("borking");
        for bap in bork {
            //println!("tests[bork][bap]: {}", bap);
        }
    }*/
}

fn main() {
    let source = test_results::CsvTestSource { filename: "name" };
    let s: &[&[bool]] = &[&[true, false, true],
            &[false, false, false],
            &[false, false, false],
            &[false, true, true]];
    use_array(s);
    println!("* End of main() {:?}", s);
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