#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Or,
    And,
    Not,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    Expression(Expression),
    Test(i32),
    InverseTest(i32),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Expression {
    pub operator: Operator,
    pub operands: Vec<Operand>,
}