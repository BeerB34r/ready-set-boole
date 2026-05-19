/// bitwise addition
pub fn adder(a: u32, b: u32) -> u32 {
    let mut sum = a ^ b;
    let mut carry = (a & b) << 1;

    while carry != 0 {
        let tmp = (sum & carry) << 1;
        sum ^= carry;
        carry = tmp;
    }
    return sum;
}

/// bitwise multiplication
pub fn multiplier(a: u32, b: u32) -> u32 {
    let (mut a, mut b): (u32, u32) = if a > b { (a, b) } else { (b, a) };
    let mut c: u32 = 0;

    while b != 0 {
        if b & 1 != 0 {
            c = adder(c, a);
        }
        a <<= 1;
        b >>= 1;
    }
    return c;
}

/// convert from binary to gray code
pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

// TODO make it read something other than _just_ RPN
/// evaluate propositional formula
pub fn eval_formula(formula: &str) -> Result<bool, &'static str> {
    let mut stack: Vec<bool> = vec![];
    for c in formula.chars() {
        match c {
            '1' => stack.push(true),
            '0' => stack.push(false),
            '!' => {
                let neg: Option<bool> = stack.pop();
                if neg.is_none() {
                    return Err("not enough values for unary operation");
                }
                stack.push(!neg.unwrap());
            }
            '&' => {
                let lhs: Option<bool> = stack.pop();
                let rhs: Option<bool> = stack.pop();
                if lhs.is_none() || rhs.is_none() {
                    return Err("not enough values for binary operation");
                };
                stack.push(lhs.unwrap() & rhs.unwrap());
            }
            '|' => {
                let lhs: Option<bool> = stack.pop();
                let rhs: Option<bool> = stack.pop();
                if lhs.is_none() || rhs.is_none() {
                    return Err("not enough values for binary operation");
                };
                stack.push(lhs.unwrap() | rhs.unwrap());
            }
            '^' => {
                let lhs: Option<bool> = stack.pop();
                let rhs: Option<bool> = stack.pop();
                if lhs.is_none() || rhs.is_none() {
                    return Err("not enough values for binary operation");
                };
                stack.push(lhs.unwrap() ^ rhs.unwrap());
            }
            '>' => {
                let lhs: Option<bool> = stack.pop();
                let rhs: Option<bool> = stack.pop();
                if lhs.is_none() || rhs.is_none() {
                    return Err("not enough values for binary operation");
                };
                stack.push(!lhs.unwrap() | rhs.unwrap());
            }
            '=' => {
                let lhs: Option<bool> = stack.pop();
                let rhs: Option<bool> = stack.pop();
                if lhs.is_none() || rhs.is_none() {
                    return Err("not enough values for binary operation");
                };
                stack.push(!(lhs.unwrap() ^ rhs.unwrap()));
            }
            _ => return Err("unrecognized symbol in proposition"),
        }
    }
    match stack.pop() {
        Some(b) => return Ok(b),
        None => return Err("no value arises from given proposition"),
    }
}

pub fn truth_table(formula: &str) -> () {
    for c in ('A'..='Z').filter(|&x| formula.contains(x)) {
        print!("| {} ", c);
    }
    if !('A'..='Z').any(|x| formula.contains(x)) {
        return;
    }
    println!("| = |");
    println!(
        "{}|---|",
        "|---".repeat(
            ('A'..='Z')
                .filter(|&x| formula.contains(x))
                .collect::<Vec<char>>()
                .len(),
        )
    );
    fn replace(formula: &str, set: String, prefix: String) {
        for c in set.chars().filter(|&x| formula.contains(x)) {
            replace(
                &formula.replace(c, "0"),
                set.replace(c, ""),
                prefix.clone() + "| 0 ",
            );
            replace(
                &formula.replace(c, "1"),
                set.replace(c, ""),
                prefix.clone() + "| 1 ",
            );
        }
        if !set.chars().any(|c| formula.contains(c)) {
            match eval_formula(formula) {
                Ok(b) => println!("{}| {} |", prefix, if b { 1 } else { 0 }),
                Err(s) => println!("{}", s),
            }
        }
    }
    replace(formula, ('A'..='Z').collect(), "".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adder_tests() {
        for i in 0..10 {
            for j in 0..10 {
                assert_eq!(i + j, adder(i, j));
            }
        }
    }
    #[test]
    fn multiplier_tests() {
        for i in 0..=10 {
            for j in 0..=10 {
                assert_eq!(i * j, multiplier(i, j));
            }
        }
    }
    #[test]
    fn gray_code_tests() {
        assert_eq!(
            vec![0u32, 1, 3, 2, 6, 7, 5, 4, 12, 13, 15],
            (0..=10).map(gray_code).collect::<Vec<u32>>()
        );
    }
    #[test]
    fn formula_tests() {
        assert!(eval_formula("1").unwrap());
        assert!(!eval_formula("0").unwrap());
        assert!(!eval_formula("10&").unwrap());
        assert!(eval_formula("10|").unwrap());
        assert!(eval_formula("10|1&").unwrap());
        assert!(eval_formula("101|&").unwrap());
        assert!(eval_formula("11>").unwrap());
        assert!(!eval_formula("10=").unwrap());
        assert!(eval_formula("1011||=").unwrap());
    }
}
