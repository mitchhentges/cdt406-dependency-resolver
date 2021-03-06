extern crate rustc_serialize;
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
    Expression(Expression),
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
        })
        .collect();
    map.insert("inputs".to_owned(), operands.to_json());
    Json::Object(map)
}

impl Expression {
    pub fn evaluate(&self, results: &Vec<bool>, variable_to_index: &Vec<usize>) -> bool {
        match self.operator {
            Operator::Or => self.operands.iter().any(|operand| evaluate_operand(operand, results, variable_to_index)),
            Operator::And => self.operands.iter().all(|operand| evaluate_operand(operand, results, variable_to_index)),
        }
    }

    pub fn variables(&self) -> Vec<i32> {
        let mut vars = Vec::<i32>::new();
        for operand in &self.operands {
            match operand {
                &Operand::Test(id) => {
                    if !vars.contains(&id) {
                        vars.push(id);
                    }
                },
                &Operand::Expression(ref e) => {
                    for id in e.variables() {
                        if !vars.contains(&id) {
                            vars.push(id)
                        }
                    }
                }
            }
        }
        vars
    }
}

fn evaluate_operand(operand: &Operand, results: &Vec<bool>, variable_to_index: &Vec<usize>) -> bool {
    match *operand {
        Operand::Test(id) => results[variable_to_index[id as usize]],
        Operand::Expression(ref expression) => expression.evaluate(results, variable_to_index),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate(expression: &Expression, results: &Vec<bool>) -> bool {
        let mut mapping = Vec::<usize>::with_capacity(results.len());
        mapping.extend(0..results.len());
        expression.evaluate(results, &mapping)
    }

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
        assert_eq!(vec!(0, 1, 5, 6), expression.variables());
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
        assert_eq!(vec!(0, 1, 2), expression.variables());
    }
}