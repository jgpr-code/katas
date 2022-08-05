fn digit2word(digit: u8) -> &'static str {
    match digit {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        _ => "nine",
    }
}

fn tensdigit2word(digit: u8) -> &'static str {
    match digit {
        0 => "zero",
        1 => "ten",
        2 => "twenty",
        3 => "thirty",
        4 => "fourty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        _ => "ninety",
    }
}

fn teens(digit: u8) -> &'static str {
    match digit {
        0 => "ten",
        1 => "eleven",
        2 => "twelve",
        3 => "thirteen",
        4 => "fourteen",
        5 => "fifteen",
        6 => "sixteen",
        7 => "seventeen",
        8 => "eighteen",
        _ => "nineteen",
    }
}

struct Digits {
    hundreds: u8,
    tens: u8,
    ones: u8,
}

fn number2digits(mut number: u16) -> Digits {
    let hundreds: u8 = (number / 100).try_into().unwrap();
    number = number % 100;
    let tens: u8 = (number / 10).try_into().unwrap();
    number = number % 10;
    let ones: u8 = number.try_into().unwrap();

    Digits {
        hundreds,
        tens,
        ones,
    }
}

fn digits2words(digits: Digits) -> String {
    let mut answer = String::new();
    let hundreds = digits.hundreds;
    let tens = digits.tens;
    let ones = digits.ones;

    if hundreds == 0 && tens == 0 && ones == 0 {
        return "zero".to_string();
    }

    if hundreds != 0 {
        answer = answer + &digit2word(hundreds) + " hundred";
        if tens != 0 || ones != 0 {
            answer = answer + " and";
        }
    }

    if tens != 0 {
        answer = answer + " ";
        if tens == 1 {
            answer = answer + &teens(ones);
            return answer.trim().to_string();
        } else {
            answer = answer + &tensdigit2word(tens);
        }
    }

    if ones != 0 {
        answer = answer + " ";
        answer = answer + &digit2word(ones);
    }

    answer.trim().to_string()
}

fn number2words(number: u16) -> String {
    let digits = number2digits(number);

    digits2words(digits)
}

fn main() {
    println!("{} = {}", 3, number2words(3));
    println!("{} = {}", 502, number2words(502));
    println!("{} = {}", 745, number2words(745));
    println!("{} = {}", 0, number2words(0));
    println!("{} = {}", 11, number2words(11));
    println!("{} = {}", 412, number2words(412));
    println!("{} = {}", 125, number2words(125));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number2words_just_ones() {
        assert!(number2words(0) == "zero");
        assert!(number2words(1) == "one");
        assert!(number2words(2) == "two");
        assert!(number2words(3) == "three");
        assert!(number2words(4) == "four");
        assert!(number2words(5) == "five");
        assert!(number2words(6) == "six");
        assert!(number2words(7) == "seven");
        assert!(number2words(8) == "eight");
        assert!(number2words(9) == "nine");
    }

    #[test]
    fn number2words_just_tens() {
        assert!(number2words(10) == "ten");
        assert!(number2words(20) == "twenty");
        assert!(number2words(30) == "thirty");
        assert!(number2words(40) == "fourty");
        assert!(number2words(50) == "fifty");
        assert!(number2words(60) == "sixty");
        assert!(number2words(70) == "seventy");
        assert!(number2words(80) == "eighty");
        assert!(number2words(90) == "ninety");
    }

    #[test]
    fn number2words_just_hundreds() {
        assert!(number2words(100) == "one hundred");
        assert!(number2words(200) == "two hundred");
        assert!(number2words(300) == "three hundred");
        assert!(number2words(400) == "four hundred");
        assert!(number2words(500) == "five hundred");
        assert!(number2words(600) == "six hundred");
        assert!(number2words(700) == "seven hundred");
        assert!(number2words(800) == "eight hundred");
        assert!(number2words(900) == "nine hundred");
    }

    #[test]
    fn number2words_special_two_digit() {
        assert!(number2words(11) == "eleven");
        assert!(number2words(12) == "twelve");
        assert!(number2words(13) == "thirteen");
        assert!(number2words(14) == "fourteen");
        assert!(number2words(15) == "fifteen");
        assert!(number2words(16) == "sixteen");
        assert!(number2words(17) == "seventeen");
        assert!(number2words(18) == "eighteen");
        assert!(number2words(19) == "nineteen");
    }

    #[test]
    fn number2words_normal_two_digit() {
        assert!(number2words(21) == "twenty one");
        assert!(number2words(32) == "thirty two");
        assert!(number2words(43) == "fourty three");
        assert!(number2words(54) == "fifty four");
        assert!(number2words(65) == "sixty five");
        assert!(number2words(76) == "seventy six");
        assert!(number2words(87) == "eighty seven");
        assert!(number2words(98) == "ninety eight");
    }

    #[test]
    fn number2words_no_tens() {
        assert!(number2words(102) == "one hundred and two");
        assert!(number2words(304) == "three hundred and four");
        assert!(number2words(506) == "five hundred and six");
        assert!(number2words(708) == "seven hundred and eight");
    }

    #[test]
    fn number2words_no_ones() {
        assert!(number2words(120) == "one hundred and twenty");
        assert!(number2words(340) == "three hundred and fourty");
        assert!(number2words(560) == "five hundred and sixty");
        assert!(number2words(780) == "seven hundred and eighty");
    }

    #[test]
    fn number2words_normal_three_digit() {
        assert!(number2words(121) == "one hundred and twenty one");
        assert!(number2words(232) == "two hundred and thirty two");
        assert!(number2words(343) == "three hundred and fourty three");
        assert!(number2words(454) == "four hundred and fifty four");
        assert!(number2words(565) == "five hundred and sixty five");
        assert!(number2words(676) == "six hundred and seventy six");
    }
}
