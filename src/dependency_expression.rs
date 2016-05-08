use expression::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TestDependency {
    pub test_id: i32,
    pub dependency: Expression,
}

pub fn dependency_expression(results: &[&[bool]], target_id: usize) -> TestDependency {
    let test_count = results.len();
    let executions = results[0].len();

    let mut pass_operands = Vec::<Operand>::with_capacity(executions); // maximum of all passes
    let mut fail_operands = Vec::<Operand>::with_capacity(executions); // or all fails
    for execution_index in 0..executions {
        let passed = results[target_id][execution_index];

        let mut execution_operands = Vec::<Operand>::with_capacity(test_count - 1); // Don't count target test
        for other_id in 0..test_count {
            if other_id == target_id {
                continue;
            }

            let operand = match results[other_id][execution_index] {
                false => Some(Operand::Test(other_id as i32)),
                true => {
                    if !passed {
                        // Don't care about other passed tests if the target failed
                        None
                    } else {
                        Some(Operand::InverseTest(other_id as i32))
                    }
                }
            };

            if operand.is_some() {
                execution_operands.push(operand.unwrap());
            }
        }

        let execution_expression = Expression {
            operator: Operator::And,
            operands: execution_operands,
        };

        if passed {
            pass_operands.push(Operand::Expression(execution_expression));
        } else {
            fail_operands.push(Operand::Expression(execution_expression));
        }
    }

    let pass_expression = Expression {
        operator: Operator::Or,
        operands: pass_operands
    };

    let fail_expression = Expression {
        operator: Operator::Or,
        operands: fail_operands
    };

    TestDependency {
        test_id: target_id as i32,
        dependency: Expression {
            operator: Operator::And,
            operands: vec!(
                Operand::Expression(fail_expression),
                Operand::Expression(Expression {
                    operator: Operator::Not,
                    operands: vec!(Operand::Expression(pass_expression)),
                }),
            )
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_derp() {
        let bork = vec!(vec!(true, false, true), vec!(false, false, true));
        let a: Vec<&[bool]> = bork.iter().map(|vec| &vec[..]).collect();
        let test_dependencies: Vec<TestDependency> = (0..a.len())
            .map(|i| dependency_expression(&a, i))
            .collect();
        println!("{:?}", test_dependencies);
    }
}