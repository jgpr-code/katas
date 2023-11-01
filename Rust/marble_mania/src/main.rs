use std::rc::Rc;
use std::{cell::RefCell, time::*};
// very ugly solution

// implement own linked list with Rc<RefCell>

struct Node {
    value: usize,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

struct LinkedList {
    head: Rc<RefCell<Node>>,
    cursor: Rc<RefCell<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        let head = Rc::new(RefCell::new(Node {
            value: 0,
            next: None,
            prev: None,
        }));
        LinkedList {
            head: Rc::clone(&head),
            cursor: Rc::clone(&head),
        }
    }
    fn insert(&mut self, value: usize) {
        let node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: Some(Rc::clone(&self.cursor)),
        }));
        let curs = self.cursor.borrow_mut();
        // curs.
    }
}

// ADT
// member:
// - last_inserted_pos
// - current_player
// next_pos_to_insert() -> pos
// remove_special() -> pos

struct MarbleMania {
    player_scores: Vec<usize>,
}

fn marbles(players: usize, marbles: usize) -> (usize, usize) {
    let mut scores: Vec<usize> = vec![0; players];
    let mut placed = Vec::new();
    let mut current_player: usize = 0;
    let mut current_pos: usize = 0;
    let mut current_len: usize = 0;

    for marble in 0..=marbles {
        if marble != 0 && marble % 23 == 0 {
            scores[current_player] += marble;
            let remove = get_pos(-8, current_pos as i32, current_len as i32);
            scores[current_player] += placed.remove(remove);
            current_len -= 1;
            current_pos = remove + 1;
        } else {
            current_pos = get_pos(1, current_pos as i32, current_len as i32);
            placed.insert(current_pos, marble);
            current_len += 1;
            current_pos += 1;
        }
        current_pos %= current_len;
        current_player = (current_player + 1) % players;
        if marble < 30 {
            println!("{:?}", placed);
        }
    }

    let mut max_score = 0;
    let mut max_index = 0;
    for (index, score) in scores.into_iter().enumerate() {
        if score > max_score {
            max_score = score;
            max_index = index;
        }
    }
    //  16  8  17  4  18  19 2 24 20(25)10 21  5 22 11  1 12  6 13  3 14  7 15
    // [16, 8, 17, 4, 18, 19, 24, 2, 25, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15, 0]
    (max_index, max_score)
}

fn get_pos(offset: i32, current_pos: i32, current_len: i32) -> usize {
    if current_len == 0 {
        return 0;
    }
    let mut pos = current_pos + offset;
    if pos < 0 {
        pos += current_len;
    }
    if pos >= current_len {
        pos %= current_len;
    }

    pos as usize
}

// 0

fn main() {
    let mut foo = vec![1, 2, 3];
    foo.insert(2, 4);
    println!("{:?}", foo);
    // 0 1 2 3, len=4, cursor=4 => +2=>6 % 4 = 2 => +2=>4 % 4 =0

    // 10 players; last marble is worth 1618 points: high score is 8317​

    // 13 players; last marble is worth 7999 points: high score is 146373​

    // 17 players; last marble is worth 1104 points: high score is 2764​

    // 21 players; last marble is worth 6111 points: high score is 54718​

    // 30 players; last marble is worth 5807 points: high score is 37305
    println!("{},{} = {}", 10, 1618, marbles(10, 1618).1);
    println!("{},{} = {}", 13, 7999, marbles(13, 7999).1);
    println!("{},{} = {}", 17, 1104, marbles(17, 1104).1);
    println!("{},{} = {}", 21, 6111, marbles(21, 6111).1);
    println!("{},{} = {}", 30, 5807, marbles(30, 5807).1);
    let n_players = 17;
    let n_marbles = 500_000;
    let now = Instant::now();
    let result = marbles(n_players, n_marbles);

    let elapsed_time = now.elapsed();
    println!(
        "Running marbles({},{}) = {:?} took {} seconds.",
        n_players,
        n_marbles,
        result,
        elapsed_time.as_secs_f32()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_marbles_1() {
        assert_eq!(marbles(10, 1618).1, 8317);
    }
    #[test]
    fn test_marbles_2() {
        assert_eq!(marbles(13, 7999).1, 146373);
    }
    #[test]
    fn test_marbles_3() {
        assert_eq!(marbles(17, 1104).1, 2764);
    }
    #[test]
    fn test_marbles_4() {
        assert_eq!(marbles(21, 6111).1, 54718);
    }
    #[test]
    fn test_marbles_5() {
        assert_eq!(marbles(30, 5807).1, 37305);
    }
}
