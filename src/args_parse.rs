#[derive(Debug, PartialEq, Eq)]
pub struct Arguments {
    pub input_filename: String,
    pub output_filename: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArgsParseException {
    InvalidNumberOfArguments(usize)
}

pub fn parse_cli_args(args: Vec<String>) -> Result<Arguments, ArgsParseException> {
    if args.len() != 3 {
        Err(ArgsParseException::InvalidNumberOfArguments(args.len()))
    } else {
        Ok(Arguments { input_filename: args[1].to_owned(), output_filename: args[2].to_owned() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_err_on_wrong_count_of_params() {
        let result = parse_cli_args(vec!["test-dependencies".to_owned()]);
        assert_eq!(result, Err(ArgsParseException::InvalidNumberOfArguments(1)));
    }

    #[test]
    fn should_put_args_in_correct_order() {
        let result = parse_cli_args(vec!["test-dependencies".to_owned(), "input.csv".to_owned(), "output.json".to_owned()]);
        assert_eq!(result, Ok(Arguments{
            input_filename: "input.csv".to_owned(),
            output_filename: "output.json".to_owned()
        }));
    }
}