#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Or,
    And,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    Test(i32),
    InverseTest(i32),
    Expression(Expression),
    InverseExpression(Expression),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Expression {
    pub operator: Operator,
    pub operands: Vec<Operand>,
}

fn evaluate_operand(operand: &Operand, results: &Vec<bool>) -> bool {
    match *operand {
        Operand::Test(id) => results[id as usize],
        Operand::InverseTest(id) => !results[id as usize],
        Operand::Expression(ref expression) => evaluate(expression, results),
        Operand::InverseExpression(ref expression) => !evaluate(expression, results),
    }
}

pub fn evaluate(expression: &Expression, results: &Vec<bool>) -> bool {
    match expression.operator {
        Operator::Or => expression.operands.iter().any(|operand| evaluate_operand(operand, results)),
        Operator::And => !(expression.operands.iter().any(|operand| !evaluate_operand(operand, results))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_evaluate_and() {
        let expression = Expression {
            operator: Operator::And,
            operands: vec!(Operand::Test(0), Operand::Test(1))
        };
        assert_eq!(evaluate(&expression, &vec!(false, false)), false);
        assert_eq!(evaluate(&expression, &vec!(false, true)), false);
        assert_eq!(evaluate(&expression, &vec!(true, false)), false);
        assert_eq!(evaluate(&expression, &vec!(true, true)), true);
    }

    #[test]
    fn should_evaluate_or() {
        let expression = Expression {
            operator: Operator::Or,
            operands: vec!(Operand::Test(0), Operand::Test(1))
        };
        assert_eq!(evaluate(&expression, &vec!(false, false)), false);
        assert_eq!(evaluate(&expression, &vec!(false, true)), true);
        assert_eq!(evaluate(&expression, &vec!(true, false)), true);
        assert_eq!(evaluate(&expression, &vec!(true, true)), true);
    }

    #[test]
    fn should_evaluate_inverse_test() {
        let expression = Expression {
            operator: Operator::Or,
            operands: vec!(Operand::Test(0), Operand::InverseTest(1))
        };
        assert_eq!(evaluate(&expression, &vec!(false, false)), true);
        assert_eq!(evaluate(&expression, &vec!(false, true)), false);
        assert_eq!(evaluate(&expression, &vec!(true, false)), true);
        assert_eq!(evaluate(&expression, &vec!(true, true)), true);
    }

    #[test]
    fn should_evaluate_sub_expression() {
        let expression = Expression {
            operator: Operator::Or,
            operands: vec!(Operand::Expression(Expression {
                operator: Operator::And,
                operands: vec!(Operand::Test(0), Operand::Test(1))
            }), Operand::Expression(Expression {
                operator: Operator::And,
                operands: vec!(Operand::Test(1), Operand::Test(2))
            }))
        };
        assert_eq!(evaluate(&expression, &vec!(false, false, false)), false);
        assert_eq!(evaluate(&expression, &vec!(false, false, true)), false);
        assert_eq!(evaluate(&expression, &vec!(false, true, false)), false);
        assert_eq!(evaluate(&expression, &vec!(false, true, true)), true);
        assert_eq!(evaluate(&expression, &vec!(true, false, false)), false);
        assert_eq!(evaluate(&expression, &vec!(true, false, true)), false);
        assert_eq!(evaluate(&expression, &vec!(true, true, false)), true);
        assert_eq!(evaluate(&expression, &vec!(true, true, true)), true);
    }

    #[test]
    fn should_evaluate_inverse_expression() {
        let expression = Expression {
            operator: Operator::Or,
            operands: vec!(Operand::Expression(Expression {
                operator: Operator::And,
                operands: vec!(Operand::Test(0), Operand::Test(1))
            }), Operand::InverseExpression(Expression {
                operator: Operator::And,
                operands: vec!(Operand::Test(1), Operand::Test(2))
            }))
        };
        assert_eq!(evaluate(&expression, &vec!(false, false, false)), true);
        assert_eq!(evaluate(&expression, &vec!(false, false, true)), true);
        assert_eq!(evaluate(&expression, &vec!(false, true, false)), true);
        assert_eq!(evaluate(&expression, &vec!(false, true, true)), false);
        assert_eq!(evaluate(&expression, &vec!(true, false, false)), true);
        assert_eq!(evaluate(&expression, &vec!(true, false, true)), true);
        assert_eq!(evaluate(&expression, &vec!(true, true, false)), true);
        assert_eq!(evaluate(&expression, &vec!(true, true, true)), true);
    }
}