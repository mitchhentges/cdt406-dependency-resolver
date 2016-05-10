use std::collections::HashSet;
use expression::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum VariableState {
    False,
    True,
    Factored,
    TargetVariable,
}

#[derive(Debug, PartialEq, Eq)]
struct AllQMSteps {
    steps: Vec<Vec<QMStepRow>>
}

impl AllQMSteps {
    fn new(steps_len: usize) -> AllQMSteps {
        let mut steps = Vec::<Vec<QMStepRow>>::with_capacity(steps_len);

        for _ in 0..steps_len {
            steps.push(Vec::<QMStepRow>::new())
        }
        AllQMSteps { steps: steps }
    }

    fn is_empty(&self) -> bool {
        !self.steps.iter().any(|rows| !rows.is_empty())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct QMStepRow {
    row: Vec<VariableState>,
    one_count: usize,
    used: bool,
    covered_rows: HashSet<usize>, //index of rows covered in original truth table
}

impl QMStepRow {
    fn reduce(&self, other: &QMStepRow) -> Option<QMStepRow> {
        let mut diff_column: i32 = -1;

        for i in 0..self.row.len() {
            if self.row[i] != other.row[i] {
                if diff_column != -1 { // More than two columns different
                    return None;
                }

                diff_column = i as i32;
            }
        }

        if diff_column == -1 { // More than two columns different
            return None;
        }

        let mut row = self.row.clone();
        row[diff_column as usize] = VariableState::Factored;
        let mut covered_rows = self.covered_rows.clone();
        covered_rows.extend(other.covered_rows.clone());

        let mut one_count = 0;
        for i in &row {
            if i == &VariableState::True {
                one_count += 1;
            }
        }

        Some(QMStepRow {
            row: row,
            one_count: one_count,
            used: false,
            covered_rows: covered_rows,
        })
    }
}

// Things got messy here. Operation "Get this project done" kicked into overdrive, and maintaining
// a nice, testable structure became low-priority
pub fn reduce(expression: &Expression) -> Expression {
    let variables = expression.variables();
    let steps_len = variables.len() + 1;
    let mut target_index = variables.len();
    for i in 0..(variables.len() + 1) {
        if !variables.contains(&(i as i32)) {
            target_index = i;
            break;
        }
    };

    println!("current var: {} variables: {:?}", target_index, variables);

    let table = truth_table(expression, target_index, &variables);
    let mut qm_steps = AllQMSteps::new(steps_len);

    for i in 0..table.len() {
        if !table[i][target_index] {
            continue;
        }

        let mut true_count = 0;
        let mut row = Vec::<VariableState>::with_capacity(table[i].len());

        for j in 0..table[i].len() {
            if j == target_index {
                row.push(VariableState::TargetVariable);
            }

            row.push(match table[i][j] {
                true => VariableState::True,
                false => VariableState::False,
            });
            if table[i][j] {
                true_count += 1;
            }
        }

        let mut covered_rows = HashSet::new();
        covered_rows.insert(i);

        qm_steps.steps[true_count].push(QMStepRow {
            row: row,
            one_count: true_count,
            used: false,
            covered_rows: covered_rows,
        })
    }

    let mut prime_implicants = Vec::<QMStepRow>::new();
    let mut next_qm_step_rows = Vec::<QMStepRow>::new();

    while !qm_steps.is_empty() {
        for i in 0..qm_steps.steps.len() {
            for x in 0..qm_steps.steps[i].len() {
                if i < qm_steps.steps.len() - 1 {
                    for y in 0..qm_steps.steps[i + 1].len() {
                        let new_step = qm_steps.steps[i][x].reduce(&(qm_steps.steps[i + 1][y]));
                        if new_step.is_none() {
                            continue;
                        }

                        next_qm_step_rows.push(new_step.unwrap());
                        qm_steps.steps[i][x].used = true;
                        qm_steps.steps[i + 1][y].used = true;
                    }
                }
            }

            for x in (0..qm_steps.steps[i].len()).rev() {
                if !qm_steps.steps[i][x].used {
                    prime_implicants.push(qm_steps.steps[i].remove(x));
                }
            }
            qm_steps.steps[i].clear();
        }

        for step in next_qm_step_rows.iter_mut() {
            step.used = false;
        }
        next_qm_step_rows.drain(0..)
            .fold((), |_, step_row| {
                qm_steps.steps[step_row.one_count].push(step_row);
            }
        );
    }

    prime_implicants.dedup();
    let minterms: HashSet<usize> =  prime_implicants.iter()
        .flat_map(|qm_step_row| &qm_step_row.covered_rows)
        .cloned()
        .collect();

    let mut min_implicants = Vec::<QMStepRow>::new();
    let mut remaining_minterms = minterms.clone();

    for minterm in &minterms {
        let mut coverage = 0;
        let mut index: usize = 0; // Guaranteed to be set in next for loop, if used
        for i in 0..prime_implicants.len() {
            if prime_implicants[i].covered_rows.contains(&minterm) {
                println!("+ minterm {} contained in prime_implicant {:?}", minterm, prime_implicants[i]);
                coverage += 1;
                index = i;
            } else {
                println!("- minterm {} !contained in prime_implicant {:?}", minterm, prime_implicants[i]);
            }
        }

        if coverage == 1 {
            println!("Only covered once: (mt) (index) -> ({}) ({})", minterm, index);
            let implicant = prime_implicants.remove(index);
            implicant.covered_rows.iter()
                .fold((), |_, term| {
                    remaining_minterms.remove(term);
                });
            min_implicants.push(implicant);
        }
    }

    //println!("{} {:?}", remaining_minterms.len(), remaining_minterms);
    //println!("{} {:?}", minterms.len(), minterms);
    //println!("{:?}", prime_implicants);
    println!("{:?}", min_implicants);

    if remaining_minterms.len() != 0 {
        unimplemented!();
    }

    let mut root_expression = Expression {
        operator: Operator::Or,
        operands: vec!()
    };

    for implicant in min_implicants {
        let mut operands: Vec<Operand> = implicant.row.iter()
            .enumerate()
            .map(|(i, state)| match *state {
                VariableState::True => Some(Operand::Test(i as i32)),
                VariableState::False => Some(Operand::InverseTest(i as i32)),
                VariableState::Factored => None,
                VariableState::TargetVariable => None,
            })
            .filter(|option| option.is_some())
            .map(|option| option.unwrap())
            .collect();

        if operands.len() == 1 {
            root_expression.operands.push(operands.remove(0));
        } else if operands.len() > 1 {
            root_expression.operands.push(Operand::Expression(Expression {
                operator: Operator::And,
                operands: operands
            }));
        }
    }

    return root_expression;
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

/*pub fn bork() {
    let expression = Expression {
        operator: Operator::And,
        operands: vec!(
                Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: vec!(
                        Operand::Test(1),
                        Operand::Test(2),
                    )
                }),
                Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: vec!(
                        Operand::Test(1),
                        Operand::Test(3),
                    )
                }),
                Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: vec!(
                        Operand::Test(2),
                        Operand::Test(4),
                    )
                }),
                Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: vec!(
                        Operand::Test(3),
                        Operand::Test(5),
                    )
                }),
                Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: vec!(
                        Operand::Test(4),
                        Operand::Test(6),
                    )
                }),
                Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: vec!(
                        Operand::Test(5),
                        Operand::Test(6),
                    )
                }),
            )
    };

    reduce(&expression);
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_derp() {
        bork();
    }
}