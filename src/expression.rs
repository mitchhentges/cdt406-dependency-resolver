use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Or,
    And,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operands {
    Expressions(Vec<Expression>),
    TestIds(Vec<i32>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Expression {
    operator: Operator,
    operands: Operands,
}

pub fn unique_variables(expression: &Expression) -> HashSet<i32> {
    let mut found_variables = HashSet::<i32>::new();
    let next_variables = match expression.operands {
        Operands::Expressions (ref expressions) => {
            expressions
                    .iter()
                    .flat_map(|expression| unique_variables(expression))
                    .collect()
        },
        Operands::TestIds (ref ids) => ids.clone(),
    };
    found_variables.extend(next_variables);
    found_variables
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
    fn should_calculate_unique_variable_count_when_one_operator() {
        let expression = Expression {
            operator: Operator::And,
            operands: Operands::TestIds(vec!(0, 1, 5, 0, 6, 6)),
        };
        assert_eq!(set(vec!(0, 1, 5, 6)), unique_variables(&expression));
    }

    #[test]
    fn should_calculate_unique_variable_count_with_child_expressions() {
        let expression = Expression {
            operator: Operator::And,
            operands: Operands::Expressions(vec!(
                Expression {
                    operator: Operator::Or,
                    operands: Operands::TestIds(vec!(0, 1))
                },
                Expression {
                    operator: Operator::Or,
                    operands: Operands::TestIds(vec!(1, 2))
                },
            )),
        };
        assert_eq!(set(vec!(0, 1, 2)), unique_variables(&expression));
    }
}