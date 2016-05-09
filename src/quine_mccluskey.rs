use expression::*;

pub fn reduce(expression: &Expression) -> Expression {
    let variables = expression.variables();
    let case_capacity = (2 as i32).pow((variables.len()) as u32) as usize;
    let case_length = variables.len();
    let mut target_index = case_length;

    for i in 0..(case_length + 1) {
        if !variables.contains(&(i as i32)) {
            target_index = i;
            break;
        }
    };

    println!("{:?}", expression);

    let mut input_matrix = Vec::<Vec<bool>>::with_capacity(case_capacity);
    for case_index in 0..case_capacity {
        let mut case = Vec::<bool>::with_capacity(case_length + 1); // Need extra slot for result
        for _ in 0..(case_length + 1) {
            case.push(false);
        }
        for test_index in 0..case_length {
            let value = ((case_index >> test_index) & 1) == 1;
            if test_index == target_index { // If this index is the result column
                case[case_length] = value;
            } else {
                case[test_index] = value;
            }
        }

        case[target_index] = expression.evaluate(&case);
        input_matrix.push(case);
    }
    println!("vars are{:?}", variables);
    return Expression {
        operator: Operator::And,
        operands: vec!(),
    };
}