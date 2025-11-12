//! # eval - Mathematical Expression Evaluator -samsit-san
//!
//! A lightweight library for evaluating mathematical expressions with basic arithmetic operations.
//!
//! Supports left-to-right evaluation of expressions containing:
//! - Addition (`+`)
//! - Subtraction (`-`)
//! - Multiplication (`*` or `x`)
//! - Division (`/`)

/// Evaluates a mathematical expression and returns the result as a floating-point number
///
/// Parses and evaluates the input expression left-to-right, supporting basic arithmetic operations.
/// Returns the computed result as an `f64`.
///
/// # Arguments
///
/// * `expression` - A `String` containing the mathematical expression to evaluate
///
/// # Returns
///
/// An `f64` containing the result of the evaluation
///
/// # Examples
///
/// ```
/// let result = eval("2+2".to_string());
/// assert_eq!(result, 4.0);
///
/// let result = eval("10*5-20".to_string());
/// assert_eq!(result, 30.0);
/// ```
pub fn eval(expression: String) -> f64 {
    let mut result = 0.0;
    let mut current_op = '+'; // default operation
    let mut num = String::new();

    for c in expression.chars() {
        if c.is_digit(10) || c == '.' {
            num.push(c); // build the number
        } else if matches!(c, '+' | '-' | '*' | '/') {
            let n = num.parse::<f64>().unwrap();
            result = apply_op(result, n, current_op);
            current_op = c;
            num.clear();
        }
    }

    // Apply last number
    if !num.is_empty() {
        let n = num.parse::<f64>().unwrap();
        result = apply_op(result, n, current_op);
    }

    result
}

// helper function to apply operation
/// Applies a single arithmetic operation between two numbers
///
/// # Arguments
///
/// * `a` - The left operand
/// * `b` - The right operand
/// * `op` - The operation character: `+`, `-`, `*`, `x`, or `/`
///
/// # Returns
///
/// The result of applying the operation
fn apply_op(a: f64, b: f64, op: char) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' | 'x' => a * b,
        '/' => a / b,
        _ => a,
    }
}

// example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = eval("2+2-4+2*6".to_string());
        assert_eq!(result, 12.0); // left-to-right evaluation
    }
}
