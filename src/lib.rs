pub fn eval(expression: &str) -> f64 {
    let expr = expression.replace('x', "*"); // treat x as *
    let mut numbers = vec![];
    let mut ops = vec![];

    let mut num = String::new();

    // Step 1: parse numbers and operators
    for c in expr.chars() {
        if c.is_digit(10) || c == '.' {
            num.push(c);
        } else if matches!(c, '+' | '-' | '*' | '/') {
            numbers.push(num.parse::<f64>().unwrap());
            num.clear();
            ops.push(c);
        }
    }
    // push the last number
    if !num.is_empty() {
        numbers.push(num.parse::<f64>().unwrap());
    }

    // Step 2: handle * and / first
    let mut i = 0;
    while i < ops.len() {
        match ops[i] {
            '*' => {
                let res = numbers[i] * numbers[i + 1];
                numbers[i] = res;
                numbers.remove(i + 1);
                ops.remove(i);
            }
            '/' => {
                let res = numbers[i] / numbers[i + 1];
                numbers[i] = res;
                numbers.remove(i + 1);
                ops.remove(i);
            }
            _ => i += 1,
        }
    }

    // Step 3: handle + and -
    let mut result = numbers[0];
    for (i, &op) in ops.iter().enumerate() {
        match op {
            '+' => result += numbers[i + 1],
            '-' => result -= numbers[i + 1],
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = eval("22+22-22x22");
        assert_eq!(result, -440.0); // now correct
    }
}
