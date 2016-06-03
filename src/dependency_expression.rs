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
        let this_passed = results[target_id][execution_index];

        let mut other_passing_tests = Vec::<usize>::new();

        for other_id in 0..test_count {
            if other_id == target_id {
                continue;
            }

            if results[other_id][execution_index] {
                other_passing_tests.push(other_id);
            }
        }

        let mut execution_operands = Vec::<Operand>::new(); // Don't count target test
        for other_id in 0..test_count {
            if other_id == target_id {
                continue;
            }

            let other_passed = results[other_id][execution_index];
            if !this_passed {
                if !other_passed { // If this one failed, only use others as operands if they failed, too
                    execution_operands.push(Operand::Test(other_id as i32));
                }
                continue;
            }

            if other_passed {
                continue; // Will use other passed tests when coupling with other failed tests (see other_passing_tests)
            }

            if other_passing_tests.is_empty() {
                execution_operands.push(Operand::Test(other_id as i32));
                continue;
            }

            let mut operands: Vec<Operand> = other_passing_tests.iter()
                .cloned()
                .map(|other_passing_id| Operand::InverseTest(other_passing_id as i32))
                .collect();
            operands.push(Operand::Test(other_id as i32));

            execution_operands.push(Operand::Expression(Expression {
                operator: Operator::And,
                operands: operands
            }));
        }

        if execution_operands.is_empty() {
            continue;
        }

        if this_passed {
            pass_operands.extend(execution_operands);
        } else {
            let new_operand = if execution_operands.len() == 1 {
                execution_operands.remove(0)
            } else {
                Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: execution_operands,
                })
            };

            fail_operands.push(new_operand);
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
        dependency_with_id(0, expression)
    }

    fn dependency_with_id(test_id: i32, expression: Expression) -> TestDependency {
        TestDependency {
            test_id: test_id,
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
                    operands: vec!(
                        Operand::Test(2),
                        Operand::Test(1)
                    )
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
                    operands: vec!(Operand::Test(1), Operand::Test(2))
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
                        operands: vec!(Operand::InverseTest(2), Operand::Test(1))
                    }))
                }),
            )
        }));
    }

    #[test]
    fn should_explode_for_pass() {
        let slice: &[&[bool]] = &[&[true], &[false], &[true], &[false]];

        assert_eq!(dependency_expression(slice, 0), dependency(Expression {
            operator: Operator::And,
            operands: vec!(Operand::Expression(Expression {
                operator: Operator::Or,
                operands: vec!(),
            }), Operand::InverseExpression(Expression {
                operator: Operator::Or,
                operands: vec!(Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(
                        Operand::InverseTest(2),
                        Operand::Test(1),
                    )
                }), Operand::Expression(Expression {
                    operator: Operator::And,
                    operands: vec!(
                        Operand::InverseTest(2),
                        Operand::Test(3),
                    )
                }))
            }))
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
                    }),Operand::Test(3),Operand::Expression(Expression {
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
                            Operand::InverseTest(3),
                            Operand::Test(2),
                        )
                    }))
                }),
            )
        }));

        assert_eq!(dependency_expression(slice, 2), dependency_with_id(2, Expression {
            operator: Operator::And,
            operands: vec!(
                Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: vec!(Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(Operand::Test(0), Operand::Test(1))
                    }),Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(Operand::Test(0), Operand::Test(3))
                    }),Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(Operand::Test(0), Operand::Test(1), Operand::Test(3))
                    }),)
                }),
                Operand::InverseExpression(Expression {
                    operator: Operator::Or,
                    operands: vec!(Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(
                            Operand::InverseTest(1),
                            Operand::Test(0),
                        )
                    }), Operand::Expression(Expression {
                        operator: Operator::And,
                        operands: vec!(
                            Operand::InverseTest(1),
                            Operand::Test(3),
                        )
                    }))
                }),
            )
        }));
    }
}