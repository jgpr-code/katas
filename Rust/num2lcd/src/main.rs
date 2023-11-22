#[rustfmt::skip]
const NUMBERS: [&'static str; 10] = [
r#"
 _ 
| |
|_|
"#,
r#"
   
  |
  |
"#,
r#"
 _ 
 _|
|_ 
"#,
r#"
 _ 
 _|
 _|
"#,
r#"
   
|_|
  |
"#,
r#"
 _ 
|_ 
 _|
"#,
r#"
 _ 
|_ 
|_|
"#,
r#"
 _ 
  |
  |
"#,
r#"
 _ 
|_|
|_|
"#,
r#"
 _ 
|_|
 _|
"#,
];

fn expand_horizontal_bars(n: usize, line: &str) -> String {
    line.replace("_", &"_".repeat(n))
}

fn expand_vertical_bars(n: usize, line: &str) -> Vec<String> {
    let mut full = vec![String::from(line)];
    let fill = line.replace("_", " ");
    for _i in 1..n {
        full.push(String::from(&fill));
    }
    full.into_iter().rev().collect()
}

fn expand_digits(horizontal_size: usize, vertical_size: usize) -> Vec<Vec<String>> {
    let mut expanded_digits: Vec<Vec<String>> = Vec::new();
    for i in NUMBERS.iter() {
        let lines: Vec<&str> = i.lines().skip(1).collect();
        let mut expanded_lines: Vec<String> = Vec::new();
        let mut min_size = 0;
        for line in lines.into_iter() {
            let mut base = expand_horizontal_bars(horizontal_size, line);
            min_size = std::cmp::max(min_size, base.len());
            // if there was a length mismatch just replace the first ' ' with as many ' ' that there is no difference anymore
            // the length mismatch happens if a line does not contain any '_' but above or below lines do
            if base.len() < min_size {
                base = base.replacen(" ", &" ".repeat(min_size - base.len() + 1), 1);
            }
            let mut full = expand_vertical_bars(vertical_size, &base);
            expanded_lines.append(&mut full);
        }
        expanded_digits.push(expanded_lines);
    }
    expanded_digits
}

fn add_digits(a: &[String], b: &[String]) -> Vec<String> {
    assert_eq!(a.len(), b.len());
    let mut result = Vec::new();
    for (a, b) in a.iter().zip(b) {
        result.push(format!("{}{}", a, b));
    }
    result
}

fn num2lcd(number: u32, horizontal_size: usize, vertical_size: usize) -> String {
    let digits: Vec<usize> = number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let expanded_digits = expand_digits(horizontal_size, vertical_size);
    let mut diter = digits.into_iter();
    let mut multiline_result = expanded_digits[diter.next().unwrap()].clone();
    for digit in diter {
        multiline_result = add_digits(&multiline_result, &expanded_digits[digit]);
    }
    let mut result = String::new();
    for line in multiline_result.into_iter() {
        result.push_str(&format!("{}\n", &line));
    }
    result
}

fn print_digit(digit: &[String]) {
    for line in digit.iter() {
        println!("{}", line);
    }
}

fn main() {
    for expanded_digit in expand_digits(2, 2) {
        print_digit(&expanded_digit);
    }
    println!("{} is\n{}\n", 123, num2lcd(123, 4, 1));
    println!("{} is\n{}\n", 1132312, num2lcd(1132312, 3, 3));
}
