// I'm using a different syntax here, because the parser is dumb.
// - [=<tag> a b c d] stands for [let <tag>(a,b) = c in d]
// - [&<tag> a b] stands for [<tag>(a,b)]
// - [@a b] stands for [(a b)]
// - [#a b] stands for [λa. b]
// - [*] stands for [()]
// - [-] stands for an erased (unused) lambda or let variable
// - [/a b c] inlines all occurrences of [a] by the closed term [b]

use symmetric_interaction_calculus::term;

pub fn main() {
    let example = term::from_string(include_bytes!("example.sic"));

    println!("-- Input (with original names):\n\n{}\n", &example);
    println!("-- Input:\n\n{}\n", term::from_net(&term::to_net(&example)));
    println!("-- Output:\n\n{}\n", term::reduce(&example));
}
