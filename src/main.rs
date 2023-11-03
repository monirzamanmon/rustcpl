extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::RuleType;

#[derive(Parser)]
#[grammar = "cpl.pest"]
struct CPLParser;

#[derive(Debug)]
enum CPLNode {
    Number(f64),
    BinaryOp(char, Box<CPLNode>, Box<CPLNode>),
}

fn eval_expression(node: &CPLNode) -> f64 {
    match node {
        CPLNode::Number(num) => *num,
        CPLNode::BinaryOp(op, left, right) => {
            let left_value = eval_expression(left);
            let right_value = eval_expression(right);
            match op {
                '+' => left_value + right_value,
                '-' => left_value - right_value,
                '*' => left_value * right_value,
                '/' => left_value / right_value,
                _ => panic!("Unsupported operator: {}", op),
            }
        }
    }
}

fn main() {
    let input = "2 + 3 * 4 - 1";
    let parsed = CPLParser::parse(RuleType::expression, input).unwrap_or_else(|e| panic!("{}", e));

    let mut expression = CPLNode::Number(0.0);

    for pair in parsed {
        match pair.as_rule() {
            RuleType::expression => {
                expression = CPLParser::eval_expression(pair.into_inner());
            }
            _ => unreachable!(),
        }
    }

    let result = eval_expression(&expression);
    println!("Result: {}", result);
}
