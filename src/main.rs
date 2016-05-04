mod test_results;
mod args_parse;
use test_results::*;
use std::env;
use std::process;

fn main() {
    let parse_result = args_parse::parse_cli_args(env::args().collect());
    if parse_result.is_err() {
        println!("Usage: ./test-dependencies input-filename output-filename");
        process::exit(-1);
    }
    let args = parse_result.unwrap();
    let source = CsvTestSource::new(&args.input_filename);
    println!("{:?}", source.read_tests());
    println!("Done!");
}