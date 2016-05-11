extern crate rustc_serialize;
use std::collections::HashSet;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Or,
    And,
}

impl ToJson for Operator {
    fn to_json(&self) -> Json {
        match *self {
            Operator::Or => Json::String("Or".to_owned()),
            Operator::And => Json::String("And".to_owned()),
        }
    }
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

pub fn expression_json(expression: &Expression, lookup: &Vec<String>) -> Json {
    let mut map = BTreeMap::new();
    map.insert("operator".to_owned(), expression.operator.to_json());
    let operands: Vec<Json> = expression.operands.iter()
        .map(|operand| match operand {
            &Operand::Test(id) => Json::String(lookup[id as usize].clone()),
            &Operand::Expression(ref e) => expression_json(e, lookup),
            _ => Json::Null,
        })
        .collect();
    map.insert("inputs".to_owned(), operands.to_json());
    Json::Object(map)
}

impl Expression {
    pub fn evaluate(&self, results: &Vec<bool>) -> bool {
        match self.operator {
            Operator::Or => self.operands.iter().any(|operand| evaluate_operand(operand, results)),
            Operator::And => !(self.operands.iter().any(|operand| !evaluate_operand(operand, results))),
        }
    }

    pub fn variables(&self) -> HashSet<i32> {
        self.operands.iter()
            .flat_map(|operand| match *operand {
                Operand::Test(id) | Operand::InverseTest(id) => {
                    let mut set = HashSet::new();
                    set.insert(id);
                    set
                },
                Operand::Expression(ref e) | Operand::InverseExpression(ref e) => e.variables(),
            }).collect()
    }
}

fn evaluate_operand(operand: &Operand, results: &Vec<bool>) -> bool {
    match *operand {
        Operand::Test(id) => results[id as usize],
        Operand::InverseTest(id) => !results[id as usize],
        Operand::Expression(ref expression) => expression.evaluate(results),
        Operand::InverseExpression(ref expression) => !expression.evaluate(results),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn set(vec: Vec<i32>) -> HashSet<i32> {
        let mut set = HashSet::<i32>::new();
        set.extend(vec);
        set
    }

    #[test]
    fn should_evaluate_and() {
        let expression = Expression {
            operator: Operator::And,
            operands: vec!(Operand::Test(0), Operand::Test(1))
        };
        assert_eq!(expression.evaluate(&vec!(false, false)), false);
        assert_eq!(expression.evaluate(&vec!(false, true)), false);
        assert_eq!(expression.evaluate(&vec!(true, false)), false);
        assert_eq!(expression.evaluate(&vec!(true, true)), true);
    }

    #[test]
    fn should_evaluate_or() {
        let expression = Expression {
            operator: Operator::Or,
            operands: vec!(Operand::Test(0), Operand::Test(1))
        };
        assert_eq!(expression.evaluate(&vec!(false, false)), false);
        assert_eq!(expression.evaluate(&vec!(false, true)), true);
        assert_eq!(expression.evaluate(&vec!(true, false)), true);
        assert_eq!(expression.evaluate(&vec!(true, true)), true);
    }

    #[test]
    fn should_evaluate_inverse_test() {
        let expression = Expression {
            operator: Operator::Or,
            operands: vec!(Operand::Test(0), Operand::InverseTest(1))
        };
        assert_eq!(expression.evaluate(&vec!(false, false)), true);
        assert_eq!(expression.evaluate(&vec!(false, true)), false);
        assert_eq!(expression.evaluate(&vec!(true, false)), true);
        assert_eq!(expression.evaluate(&vec!(true, true)), true);
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
        assert_eq!(expression.evaluate(&vec!(false, false, false)), false);
        assert_eq!(expression.evaluate(&vec!(false, false, true)), false);
        assert_eq!(expression.evaluate(&vec!(false, true, false)), false);
        assert_eq!(expression.evaluate(&vec!(false, true, true)), true);
        assert_eq!(expression.evaluate(&vec!(true, false, false)), false);
        assert_eq!(expression.evaluate(&vec!(true, false, true)), false);
        assert_eq!(expression.evaluate(&vec!(true, true, false)), true);
        assert_eq!(expression.evaluate(&vec!(true, true, true)), true);
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
        assert_eq!(expression.evaluate(&vec!(false, false, false)), true);
        assert_eq!(expression.evaluate(&vec!(false, false, true)), true);
        assert_eq!(expression.evaluate(&vec!(false, true, false)), true);
        assert_eq!(expression.evaluate(&vec!(false, true, true)), false);
        assert_eq!(expression.evaluate(&vec!(true, false, false)), true);
        assert_eq!(expression.evaluate(&vec!(true, false, true)), true);
        assert_eq!(expression.evaluate(&vec!(true, true, false)), true);
        assert_eq!(expression.evaluate(&vec!(true, true, true)), true);
    }

    #[test]
    fn should_calculate_unique_variable_count_when_one_operator() {
        let expression = Expression {
            operator: Operator::And,
            operands: vec!(
                Operand::Test(0),
                Operand::Test(1),
                Operand::Test(5),
                Operand::Test(0),
                Operand::Test(6),
                Operand::Test(6),
            ),
        };
        assert_eq!(set(vec!(0, 1, 5, 6)), expression.variables());
    }

    #[test]
    fn should_calculate_unique_variable_count_with_child_expressions() {
        let expression = Expression {
            operator: Operator::And,
            operands: vec!(Operand::Expression(
                Expression {
                    operator: Operator::Or,
                    operands: vec!(
                        Operand::Test(0),
                        Operand::Test(1),
                    )
                }), Operand::Expression(Expression {
                    operator: Operator::Or,
                    operands: vec!(
                        Operand::Test(1),
                        Operand::Test(2),
                    )
                },
            )),
        };
        assert_eq!(set(vec!(0, 1, 2)), expression.variables());
    }
}