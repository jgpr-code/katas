use librualg::trie::Trie;
use std::fs;

fn main() {
    let contents = fs::read_to_string("wordlist.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut trie = Trie::new();
    for word in lines.iter() {
        trie.insert(*word);
    }

    println!("contains s: {}", trie.contains(&"s"));
    println!("contains donnés: {}", trie.contains(&"donnés"));
    println!("contains donné: {}", trie.contains(&"donné"));

    let mut count = 0;
    for word in lines.iter() {
        if word == &"donnés" {
            println!("now");
        }
        let chars: Vec<char> = word.chars().collect();
        let wl = chars.len();
        //if wl == 6 {
        for splitter in 1..wl {
            let first = chars[0..splitter].iter().collect::<String>();
            let second = chars[splitter..].iter().collect::<String>();
            if word == &"donnés" {
                println!("trying: {} + {}", first, second);
            }
            if trie.contains(&first) && trie.contains(&second) {
                // println!("{} = {} + {}", word, first, second);
                count += 1;
            }
        }
        //}
    }
    println!("{} with 6 letters", count);
}
