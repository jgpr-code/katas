// In this demo I'll program a small fizzbuzz example in Rust (of course)
// Recap what fizzbuzz is:
// For the numbers from 1 to 20 if
// - number divisible by 3 -> print fizz
// - number divisible by 5 -> print buzz
// - number divisible by both -> print fizzbuzz
// - else -> print number

fn fizzbuzz(number: i32) -> String {
    match (number % 3 == 0, number % 5 == 0) {
        (true, true) => String::from("fizzbuzz"),
        (true, _) => String::from("fizz"),
        (_, true) => String::from("buzz"),
        _ => number.to_string(),
    }
}

fn main() {
    for i in 1..=20 {
        println!("{}", fizzbuzz(i));
    }
}
