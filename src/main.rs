mod test_results;
mod args_parse;
mod expression;
mod dependency_expression;
use test_results::*;
use args_parse::*;
use dependency_expression::*;
use std::env;
use std::process;

fn main() {
    let parse_result = parse_cli_args(env::args().collect());
    if parse_result.is_err() {
        println!("Usage: ./test-dependencies input-filename output-filename");
        process::exit(-1);
    }
    let args = parse_result.unwrap();
    let source = CsvTestSource::new(&args.input_filename);
    let tests = source.read_tests().unwrap();
    let tests_slices: Vec<&[bool]> = tests.results
        .iter()
        .map(|vec| &vec.executions[..])
        .collect();
    let test_dependencies: Vec<TestDependency> = (0..tests.count)
        .map(|i| dependency_expression(&tests_slices, i))
        .collect();
    println!("{:?}", test_dependencies[0]);
    println!("Done!");
}