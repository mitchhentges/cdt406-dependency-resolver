mod test_results;
mod args_parse;
use test_results::*;
use std::env;
use std::process;

fn main() {
    let parseResult = args_parse::parse_cli_args(env::args().collect());
    if parseResult.is_err() {
        println!("Usage: ./test-dependencies input-filename output-filename");
        process::exit(-1);
    }
    let args = parseResult.unwrap();
    let source = CsvTestSource::new(&args.input_filename);
}