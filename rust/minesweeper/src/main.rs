struct MinesField {
    rows: usize,
    cols: usize,
    mines: Vec<(usize, usize)>,
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
        let mut mines = Vec::new();
        for (row, line) in lines.enumerate() {
            let chars: Vec<char> = line.chars().collect();
            assert!(chars.len() == cols);
            for (col, c) in chars.iter().enumerate() {
                if *c == '*' {
                    mines.push((row, col));
                } else if *c != '.' {
                    panic!("invalid input either *|. are expected!");
                }
            }
        }
        MinesField { rows, cols, mines }
    }
}

impl MinesField {
    fn inside(&self, row: i32, col: i32) -> bool {
        0 <= row && row < self.rows as i32 && 0 <= col && col < self.cols as i32
    }
    fn hint_field(&self) -> String {
        let mut counts = vec![vec![0; self.cols]; self.rows];
        let drow = vec![-1, -1, -1, 0, 0, 1, 1, 1];
        let dcol = vec![-1, 0, 1, -1, 1, -1, 0, 1];
        for &(mine_row, mine_col) in &self.mines {
            for i in 0..8 {
                let row = mine_row as i32 + drow[i];
                let col = mine_col as i32 + dcol[i];
                if self.inside(row, col) {
                    counts[row as usize][col as usize] += 1;
                }
            }
        }
        let mut answer: Vec<char> = vec!['*'; self.rows * self.cols];
        for row in 0..self.rows {
            for col in 0..self.cols {
                answer[row * self.cols + col] = char::from_digit(counts[row][col], 10).unwrap();
            }
        }
        for &(mine_row, mine_col) in &self.mines {
            answer[mine_row * self.cols + mine_col] = '*';
        }
        let mut ans = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                ans.push(answer[row * self.cols + col]);
            }
            ans.push('\n');
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::MinesField;
    use std::fs::read_to_string;

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

    #[test]
    fn field_3x4_two_mines() {
        let input = "3 4\n*...\n..*.\n....\n";
        let expected = "*211\n12*1\n0111\n";
        let field = MinesField::from(input);
        assert_eq!(expected, field.hint_field().as_str());
    }

    #[test]
    fn field_9x9_some_mines() {
        let input = read_to_string("input_9x9.txt").unwrap();
        let mut expected = read_to_string("solution_9x9.txt").unwrap();
        expected = expected.replace("\r\n", "\n");
        let field = MinesField::from(input.as_str());
        assert_eq!(expected, field.hint_field());
    }
}
