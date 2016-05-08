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

    TestDependency {
        test_id: target_id as i32,
        dependency: Expression {
            operator: Operator::And,
            operands: vec!(
                Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: fail_operands
                }),
                Operand::InverseExpression(Expression {
                    operator: Operator::Or,
                    operands: pass_operands
                }),
            )
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expression::*;

    fn dependency(expression: Expression) -> TestDependency {
        TestDependency {
            test_id: 0,
            dependency: expression
        }
    }

    #[test]
    fn should_and_operator_simultaneously_failing() {
        let slice: &[&[bool]] = &[&[false], &[false], &[false]];
        assert_eq!(dependency_expression(slice, 0), dependency(Expression {
            operator: Operator::And,
            operands: vec!(
                Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: vec!(Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(Operand::Test(1), Operand::Test(2))
                    }))
                }),
                Operand::InverseExpression(Expression {
                    operator: Operator::Or,
                    operands: vec!()
                }),
            )
        }));
    }

    #[test]
    fn should_or_operator_separately_failing() {
        let slice: &[&[bool]] = &[&[false, false], &[true, false], &[false, true]];
        assert_eq!(dependency_expression(slice, 0), dependency(Expression {
            operator: Operator::And,
            operands: vec!(
                Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: vec!(Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(Operand::Test(2))
                    }), Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(Operand::Test(1))
                    }))
                }),
                Operand::InverseExpression(Expression {
                    operator: Operator::Or,
                    operands: vec!()
                }),
            )
        }));
    }

    #[test]
    fn should_not_operator_with_other_test_state_for_target_pass() {
        let slice: &[&[bool]] = &[&[true], &[false], &[false]];
        assert_eq!(dependency_expression(slice, 0), dependency(Expression {
            operator: Operator::And,
            operands: vec!(
                Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: vec!()
                }),
                Operand::InverseExpression(Expression {
                    operator: Operator::Or,
                    operands: vec!(Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(Operand::Test(1), Operand::Test(2))
                    }))
                }),
            )
        }));
    }

    #[test]
    fn should_invert_other_passing_tests_when_target_fails() {
        let slice: &[&[bool]] = &[&[true], &[false], &[true]];
        assert_eq!(dependency_expression(slice, 0), dependency(Expression {
            operator: Operator::And,
            operands: vec!(
                Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: vec!()
                }),
                Operand::InverseExpression(Expression {
                    operator: Operator::Or,
                    operands: vec!(Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(Operand::Test(1), Operand::InverseTest(2))
                    }))
                }),
            )
        }));
    }

    #[test]
    fn should_work_with_more_complex_example() {
        let slice: &[&[bool]] = &[
            &[false, false, true, false, false],
            &[false, true, true, true, false],
            &[false, false, false, true, false],
            &[true, false, true, false, false]
        ];

        assert_eq!(dependency_expression(slice, 0), dependency(Expression {
            operator: Operator::And,
            operands: vec!(
                Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: vec!(Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(Operand::Test(1), Operand::Test(2))
                    }),Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(Operand::Test(2), Operand::Test(3))
                    }),Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(Operand::Test(3))
                    }),Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(Operand::Test(1), Operand::Test(2), Operand::Test(3))
                    }),)
                }),
                Operand::InverseExpression(Expression {
                    operator: Operator::Or,
                    operands: vec!(Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(
                            Operand::InverseTest(1),
                            Operand::Test(2),
                            Operand::InverseTest(3),
                        )
                    }))
                }),
            )
        }));
    }
}