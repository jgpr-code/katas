fn modulus(num: i8, modul: i8) -> i8 {
    let non_negative = if num < 0 { num + modul } else { num };

    non_negative % modul
}

fn shift_letter(c: char, shift: i8) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    }

    let ascii_a: i8 = if c.is_ascii_lowercase() {
        'a' as i8
    } else {
        'A' as i8
    };

    ((modulus(c as i8 - ascii_a + shift, 26)) + ascii_a) as u8 as char
}

fn caesar_encrypt(text: &str, shift: i8) -> String {
    let mut answer = String::new();

    for c in text.chars() {
        answer.push(shift_letter(c, shift));
    }

    answer
}

fn caesar_decrypt(text: &str, shift: i8) -> String {
    caesar_encrypt(text, -shift)
}

fn main() {
    println!("Hello, world!");
    let a = caesar_encrypt("Method ,.Park!?", 1);
    println!("{}", a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num: i8 = -2;
        assert_eq!(modulus(num, 26), 24);
    }

    #[test]
    fn caesar_identity() {
        let text = "Method Park";

        let encrypted = caesar_encrypt(text, 13);

        let decrypted = caesar_decrypt(&encrypted, 13);

        assert_eq!(decrypted, text);
    }
}
