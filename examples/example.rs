use symmetric_interaction_calculus::term;

pub fn main() {
    let example = term::from_string(include_bytes!("example.sic"));

    println!("-- Input (with original names):\n\n{}\n", &example);
    println!("-- Input:\n\n{}\n", term::from_net(&term::to_net(&example)));
    println!("-- Output:\n\n{}\n", term::reduce(&example));
}
