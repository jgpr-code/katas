struct MinesField {
    rows: usize,
    cols: usize,
    data: Vec<char>,
}

impl From<&str> for MinesField {
    // a field looks like this
    // 1 1
    // *
    // or this
    // 3 4
    // *...
    // ..*.
    // ....
    fn from(field_str: &str) -> Self {
        let mut lines = field_str.lines();
        let (rows, cols) = lines
            .next()
            .unwrap()
            .split_once(" ")
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .unwrap();
        let mut data = Vec::new();
        for line in lines {
            let mut chars: Vec<char> = line.chars().collect();
            assert!(chars.len() == cols);
            data.append(&mut chars);
        }
        MinesField { rows, cols, data }
    }
}

impl MinesField {
    fn hint_field(&self) -> String {
        let mut answer = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.data[row * self.cols + col] == '*' {
                    answer.push('*')
                } else {
                    answer.push('0')
                }
            }
        }
        answer.push('\n');
        answer
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::MinesField;

    #[test]
    fn field_1x1_no_mines() {
        let input = "1 1\n.\n";
        let expected = "0\n";
        let field = MinesField::from(input);
        assert_eq!(expected, field.hint_field().as_str());
    }

    #[test]
    fn field_1x1_one_mine() {
        let input = "1 1\n*\n";
        let expected = "*\n";
        let field = MinesField::from(input);
        assert_eq!(expected, field.hint_field().as_str());
    }
}
