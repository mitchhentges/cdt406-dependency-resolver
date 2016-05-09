use std::collections::HashSet;
use expression::*;

#[derive(Debug, PartialEq, Eq)]
enum State {
    False,
    True,
    Used,
}

#[derive(Debug, PartialEq, Eq)]
struct QM_Item {
    row: Vec<State>,
    one_count: usize,
    used: bool,
    covered_rows: Vec<usize>, //index of rows covered in original truth table
}

pub fn reduce(expression: &Expression) -> Expression {
    let variables = expression.variables();
    let mut target_index = variables.len();
    for i in 0..(variables.len() + 1) {
        if !variables.contains(&(i as i32)) {
            target_index = i;
            break;
        }
    };

    let table = truth_table(expression, target_index, &variables);

    // There can be a group for all 0s, some 0s and 1s, and all 1s
    let mut qm_groups = Vec::<Vec<QM_Item>>::with_capacity(variables.len() + 1);
    for _ in 0..(variables.len() + 1) {
        qm_groups.push(Vec::<QM_Item>::new());
    }

    for i in 0..table.len() {
        if !table[i][target_index] {
            continue;
        }

        let mut true_count = 0;
        let mut row = Vec::<State>::with_capacity(table[i].len());

        for j in 0..table[i].len() {
            if j == target_index {
                continue;
            }

            row.push(match table[i][j] {
                true => State::True,
                false => State::False,
            });
            if table[i][j] {
                true_count += 1;
            }
        }

        qm_groups[true_count].push(QM_Item {
            row: row,
            one_count: true_count,
            used: false,
            covered_rows: vec!(i),
        })
    }



    return Expression {
        operator: Operator::And,
        operands: vec!(),
    };
}

pub fn truth_table(expression: &Expression, target_index: usize, variables: &HashSet<i32>) -> Vec<Vec<bool>> {
    let case_capacity = (2 as i32).pow((variables.len()) as u32) as usize;
    let case_length = variables.len();

    let mut table = Vec::<Vec<bool>>::with_capacity(case_capacity);
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
        table.push(case);
    }
    table
}