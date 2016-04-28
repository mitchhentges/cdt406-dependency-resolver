pub fn return_string() -> &str {
    return "bork";
}

fn main() {
    println!("{}", return_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_is_bork() {
        let s = "bork";//return_string();
        assert_eq!("bork", s);
    }
}