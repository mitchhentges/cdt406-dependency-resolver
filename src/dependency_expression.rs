use expression::*;

pub fn dependency_expression(results: &[&[bool]], target_id: usize) -> Option<Expression> {
    let test_count = results.len();
    let executions = results[0].len();

    // maximum capacity of all sub-expressions == number of executions
    let mut subexpressions = Vec::<Operand>::new();
    for execution_index in 0..executions {
        if !results[target_id][execution_index] {
            // We don't care about cases where the target test fails. It could've been a
            // dependency, or it could've been the test itself. We don't know
            continue;
        }

        let mut other_passing = Vec::<Operand>::new();
        for other_id in 0..test_count {
            if other_id == target_id || !results[other_id][execution_index] {
                // If this is the target test, or if it didn't pass, it can't be a dependency
                continue;
            }

            other_passing.push(Operand::Test(other_id.clone() as i32));
        }

        if other_passing.len() == 0 {
            // If a test can pass while all other fails, then it has no external dependencies
            return None;
        }

        subexpressions.push(Operand::Expression(Expression {
            operator: Operator::And,
            operands: other_passing,
        }));
    }

    Some(Expression {
        operator: Operator::Or,
        operands: subexpressions
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use expression::*;

    #[test]
    fn should_and_operator_simultaneously_passing() {
        let slice: &[&[bool]] = &[&[true], &[true], &[true]];
        assert_eq!(dependency_expression(slice, 0), Some(Expression {
            operator: Operator::Or,
            operands: vec!(
                Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(Operand::Test(1), Operand::Test(2)),
                }),
            ),
        }));
    }

    #[test]
    fn should_or_operator_separately_passing() {
        let slice: &[&[bool]] = &[&[true, true], &[true, false], &[false, true]];
        assert_eq!(dependency_expression(slice, 0), Some(Expression {
            operator: Operator::Or,
            operands: vec!(
                Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(Operand::Test(1)),
                }),
                Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(Operand::Test(2)),
                }),
            ),
        }));
    }

    #[test]
    fn should_work_with_more_complex_example() {
        let slice: &[&[bool]] = &[
            &[true, true, false, true, true],
            &[true, false, false, false, true],
            &[true, true, true, false, true],
            &[false, true, false, true, true]
        ];

        assert_eq!(dependency_expression(slice, 0), Some(Expression {
            operator: Operator::Or,
            operands: vec!(
                Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(Operand::Test(1), Operand::Test(2)),
                }),
                Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(Operand::Test(2), Operand::Test(3)),
                }),
                Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(Operand::Test(3)),
                }),
                Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(Operand::Test(1), Operand::Test(2), Operand::Test(3)),
                }),
            ),
        }));

        assert_eq!(dependency_expression(slice, 1), Some(Expression {
            operator: Operator::Or,
            operands: vec!(
                Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(Operand::Test(0), Operand::Test(2)),
                }),
                Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(Operand::Test(0), Operand::Test(2), Operand::Test(3)),
                }),
            ),
        }));

        assert_eq!(dependency_expression(slice, 2), None);

        assert_eq!(dependency_expression(slice, 3), Some(Expression {
            operator: Operator::Or,
            operands: vec!(
                Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(Operand::Test(0), Operand::Test(2)),
                }),
                Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(Operand::Test(0)),
                }),
                Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(Operand::Test(0), Operand::Test(1), Operand::Test(2)),
                }),
            ),
        }));
    }
}