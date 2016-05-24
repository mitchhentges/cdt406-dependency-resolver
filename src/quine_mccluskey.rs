use std::collections::HashSet;
use expression::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum VariableState {
    False,
    True,
    Factored,
}

#[derive(Debug, PartialEq, Eq)]
struct AllQMSteps {
    steps: Vec<Vec<QMStepRow>>
}

impl AllQMSteps {
    fn new(variable_count: usize) -> AllQMSteps {
        // Each "step" is a permutation on the number of "true"s
        let mut steps = Vec::<Vec<QMStepRow>>::with_capacity(variable_count + 1);

        for _ in 0..(variable_count + 1) {
            steps.push(Vec::<QMStepRow>::new())
        }
        AllQMSteps { steps: steps }
    }

    fn is_empty(&self) -> bool {
        !self.steps.iter().any(|rows| !rows.is_empty())
    }
}

#[derive(Debug, Eq)]
struct QMStepRow {
    row: Vec<VariableState>,
    true_count: usize,
    used: bool,
    covered_rows: Vec<usize>, //index of rows covered in original truth table
}

impl PartialEq for QMStepRow {
    fn eq(&self, other: &QMStepRow) -> bool {
        return self.row.eq(&other.row);
    }

    fn ne(&self, other: &QMStepRow) -> bool {
        return self.row.ne(&other.row);
    }
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
        covered_rows.dedup();

        let mut true_count = 0;
        for i in &row {
            if i == &VariableState::True {
                true_count += 1;
            }
        }

        Some(QMStepRow {
            row: row,
            true_count: true_count,
            used: false,
            covered_rows: covered_rows,
        })
    }
}

// Things got messy here. Operation "Get this project done" kicked into overdrive, and maintaining
// a nice, testable structure became low-priority
pub fn reduce(expression: &Expression) -> Option<Expression> {
    let variables = expression.variables();
    let mut max_variable: usize = 0;

    for variable in &variables {
        let converted_variable = variable.clone() as usize;
        if converted_variable > max_variable {
            max_variable = converted_variable;
        }
    }
    max_variable += 1;

    let mut index_to_variable = Vec::<i32>::with_capacity(variables.len());
    let mut variable_to_index: Vec<usize> = vec!(0; max_variable);

    for (i, variable) in variables.iter().enumerate() {
        index_to_variable.push(variable.clone());
        variable_to_index[variable.clone() as usize] = i;
    }

    let table = truth_table(expression, variables.len() as u32, &variable_to_index);
    let mut qm_steps = AllQMSteps::new(variables.len());

    for i in 0..table.len() {
        if !table[i][variables.len()] { // If the result (last cell) is "false", ignore row
            continue;
        }

        let mut true_count = 0;
        let mut row = Vec::<VariableState>::with_capacity(variables.len());

        for j in 0..variables.len() {
            row.push(match table[i][j] {
                true => VariableState::True,
                false => VariableState::False,
            });
            if table[i][j] {
                true_count += 1;
            }
        }

        let mut covered_rows = Vec::new();
        covered_rows.push(i);

        qm_steps.steps[true_count].push(QMStepRow {
            row: row,
            true_count: true_count,
            used: false,
            covered_rows: covered_rows,
        })
    }

    let mut prime_implicants = Vec::<QMStepRow>::new();
    let mut next_qm_step_rows = Vec::<QMStepRow>::new();

    while !qm_steps.is_empty() {
        for i in 0..qm_steps.steps.len() {
            for x in 0..qm_steps.steps[i].len() {
                if i == qm_steps.steps.len() - 1 { // Don't check for all-true row + all-true-plus-one row
                    continue;
                }

                for y in 0..qm_steps.steps[i + 1].len() {
                    let new_step = qm_steps.steps[i][x].reduce(&(qm_steps.steps[i + 1][y]));
                    if new_step.is_none() {
                        continue;
                    }

                    let unwrapped = new_step.unwrap();
                    if !next_qm_step_rows.contains(&unwrapped) {
                        next_qm_step_rows.push(unwrapped);
                    }
                    qm_steps.steps[i][x].used = true;
                    qm_steps.steps[i + 1][y].used = true;
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
                qm_steps.steps[step_row.true_count].push(step_row);
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
                coverage += 1;
                index = i;
            }
        }

        if coverage == 1 {
            let implicant = prime_implicants.remove(index);
            implicant.covered_rows.iter()
                .fold((), |_, term| {
                    remaining_minterms.remove(term);
                });
            min_implicants.push(implicant);
        }
    }

    if remaining_minterms.len() != 0 {
        // Usually "Petrick's Method" would be used here, but I don't think we'll ever reach
        // the situation where it's necessary
        unimplemented!();
    }

    let mut root_expression = Expression {
        operator: Operator::Or,
        operands: vec!()
    };

    let min_implicants_len = min_implicants.len();
    for implicant in min_implicants {
        if implicant.row.len() == 0 {
            continue;
        }

        let mut operands: Vec<Operand> = implicant.row.iter()
            .enumerate()
            .map(|(i, state)| match *state {
                VariableState::True => Some(Operand::Test(index_to_variable[i])),
                VariableState::False => None,
                VariableState::Factored => None,
            })
            .filter(|option| option.is_some())
            .map(|option| option.unwrap())
            .collect();

        operands.dedup();

        if min_implicants_len == 1 {
            return Some(Expression {
                operator: Operator::Or,
                operands: operands,
            });
        }

        if operands.len() == 1 {
            root_expression.operands.push(operands.remove(0));
        } else if operands.len() > 1 {
            root_expression.operands.push(Operand::Expression(Expression {
                operator: Operator::And,
                operands: operands
            }));
        }
    }

    if root_expression.operands.is_empty() {
        return None;
    }

    root_expression.operands.dedup();
    Some(root_expression)
}

pub fn truth_table(expression: &Expression, variable_count: u32, variable_to_index: &Vec<usize>) -> Vec<Vec<bool>> {
    let case_length = (variable_count + 1) as usize; // One cell per variable, plus the result on the end
    let case_count = (2 as i32).pow(variable_count as u32) as usize;
    let mut table = Vec::<Vec<bool>>::with_capacity(case_count);

    for case_index in 0..case_count {
        let mut case = Vec::<bool>::with_capacity(case_length);

        for test_index in 0..(case_length - 1) {
            case.push(((case_index >> test_index) & 1) == 1);
        }

        let result = expression.evaluate(&case, variable_to_index);
        case.push(result);
        table.push(case);
    }
    table
}