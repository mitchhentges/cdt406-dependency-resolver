mod test_results;
mod args_parse;
mod expression;
mod dependency_expression;
mod quine_mccluskey;
use test_results::*;
use args_parse::*;
use dependency_expression::*;
use quine_mccluskey::*;
use expression::*;
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
    let test_dependencies: Vec<(i32, Option<Expression>)> = (0..tests.count)
        //.take(1)
        .map(|i| dependency_expression(&tests_slices, i))
        .map(|test_dependency| (test_dependency.test_id, reduce(&test_dependency.dependency)))
        .collect();
    println!("{:?}", test_dependencies[0]);
    println!("{:?}", test_dependencies[1]);
    println!("{:?}", test_dependencies[2]);
    println!("{:?}", test_dependencies[3]);
    //bork();
    println!("Done!");
}