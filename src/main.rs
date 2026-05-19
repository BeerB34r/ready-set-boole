fn boole_op() -> () {
    let operations: &[char] = &['+', '*', 'g'];
    print!(
        "Please provide a simple two-variable arithmetic expression ({:?}): ",
        operations
    );
    let _ = std::io::Write::flush(&mut std::io::stdout());
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    input = input.trim().to_string();
    println!("user input: \"{}\"", input);
    let f = |(a, b): (&str, &str)| {
        (
            a.trim().to_string(),
            b.replace(operations, " ").trim().to_string(),
        )
    };
    let (a, b): (String, String) = f(input.split_at(
        input
            .find(operations)
            .expect("no valid operation found in user string"),
    ));
    match input {
        _ if input.contains('+') => {
            println!(
                "{} + {} => {}",
                a,
                b,
                boole::adder(
                    a.parse().expect("lhs is not a number"),
                    b.parse().expect("rhs is not a number")
                )
            )
        }
        _ if input.contains('*') => {
            println!(
                "{} * {} => {}",
                a,
                b,
                boole::multiplier(
                    a.parse().expect("lhs is not a number"),
                    b.parse().expect("rhs is not a number")
                )
            )
        }
        _ if input.contains('g') => {
            println!(
                "gray({}) => {}",
                a,
                boole::gray_code(a.parse().expect("can't convert non-number to gray code"))
            )
        }
        _ => unreachable!("user input doesnt contain valid operation"),
    }
    print!("Please provide a propositional formula: ");
    let _ = std::io::Write::flush(&mut std::io::stdout());
    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    println!("user input: \"{}\"", input);
    boole::truth_table(&input.trim())
}

fn main() {
    println!("Hello, world!");

    boole_op();
}
