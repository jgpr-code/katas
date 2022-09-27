trait Encryption {
    fn encrypt(&self, msg: &str) -> String;
    fn decrypt(&self, msg: &str) -> String;
}

// TODO write tests macro

struct AsciiCaesarCipher {
    offset_lower_a: i8,
    offset_upper_a: i8,
    default_shift: i8,
    alphabet_size: i8,
}

// TODO can this be moved to the impl block?
// Right now it is only used by shift_default and shift_default_string
enum ShiftDirection {
    FORWARD,
    BACKWARD,
}

impl AsciiCaesarCipher {
    // TODO make less verbose if possible e.g. remove repetition of offsets
    fn new(default_shift: i8, alphabet_size: i8) -> AsciiCaesarCipher {
        AsciiCaesarCipher {
            offset_lower_a: 'a' as i8,
            offset_upper_a: 'A' as i8,
            default_shift,
            alphabet_size,
        }
    }
    fn rot13() -> AsciiCaesarCipher {
        AsciiCaesarCipher {
            offset_lower_a: 'a' as i8,
            offset_upper_a: 'A' as i8,
            default_shift: 13,
            alphabet_size: 26,
        }
    }
    fn rot_x(x: i8) -> AsciiCaesarCipher {
        AsciiCaesarCipher {
            offset_lower_a: 'a' as i8,
            offset_upper_a: 'A' as i8,
            default_shift: x,
            alphabet_size: 26,
        }
    }

    fn to_offset_char(i: i8, offset: i8) -> char {
        (i + offset) as u8 as char
    }

    fn to_offset_i8(c: char, offset: i8) -> i8 {
        c as i8 - offset
    }

    fn shift(&self, c: char, shift: i8) -> char {
        if !c.is_ascii_alphabetic() {
            return c;
        }

        let ascii_offset = if c.is_ascii_lowercase() {
            self.offset_lower_a
        } else {
            self.offset_upper_a
        };

        let c_i8 = Self::to_offset_i8(c, ascii_offset);
        let shifted = (c_i8 + shift).rem_euclid(self.alphabet_size);

        Self::to_offset_char(shifted, ascii_offset)
    }

    fn shift_string(&self, text: &str, shift: i8) -> String {
        let mut shifted = String::new();
        for c in text.chars() {
            shifted.push(self.shift(c, shift));
        }
        shifted
    }

    fn shift_default(&self, c: char, direction: ShiftDirection) -> char {
        match direction {
            ShiftDirection::FORWARD => self.shift(c, self.default_shift),
            ShiftDirection::BACKWARD => self.shift(c, -self.default_shift),
        }
    }

    fn shift_default_string(&self, text: &str, direction: ShiftDirection) -> String {
        match direction {
            ShiftDirection::FORWARD => self.shift_string(text, self.default_shift),
            ShiftDirection::BACKWARD => self.shift_string(text, -self.default_shift),
        }
    }
}

impl Encryption for AsciiCaesarCipher {
    fn encrypt(&self, msg: &str) -> String {
        self.shift_default_string(msg, ShiftDirection::BACKWARD)
    }

    fn decrypt(&self, msg: &str) -> String {
        self.shift_default_string(msg, ShiftDirection::FORWARD)
    }
}

fn main() {
    println!("Hello, world!");
    let rot13 = AsciiCaesarCipher::rot13();
    let a = rot13.encrypt("Method ,.Park!?");
    println!("{}", a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let num: i8 = -2;
        assert_eq!(num.rem_euclid(26), 24);
    }

    #[test]
    fn caesar_identity() {
        let text = "Method Park";

        let rot13 = AsciiCaesarCipher::rot13();

        let encrypted = rot13.encrypt(text);

        let decrypted = rot13.decrypt(&encrypted);

        assert_eq!(decrypted, text);
    }
}
