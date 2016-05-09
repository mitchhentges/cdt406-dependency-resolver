use expression::*;

pub fn reduce(expression: &Expression) -> Expression {
    let variables = expression.variables();
    let case_capacity = (2 as i32).pow((variables.len()) as u32);
    let case_length = variables.len() + 1; // Need extra slot for result

    let mut input_matrix = Vec::<Vec<bool>>::with_capacity(case_capacity as usize);
    for _ in 0..case_capacity {
        let mut case = Vec::<bool>::with_capacity(variables.len());
        for _ in 0..case_length {
            case.push(false);
        }
        input_matrix.push(case);
    }
    println!("reduce()!");
    println!("vars are{:?}", variables);
    println!("capacity is {}", case_capacity);
    println!("{:?}", input_matrix);
    return Expression {
        operator: Operator::And,
        operands: vec!(),
    };
}