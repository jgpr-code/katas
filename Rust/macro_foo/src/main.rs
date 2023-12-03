#![feature(lazy_cell)]

macro_rules! regex {
    ($re:literal) => {{
        static RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
            println!("init regex {}", $re);
            regex::Regex::new($re).unwrap()
        });
        &RE
    }};
}

macro_rules! modname {
    () => {{
        let path = module_path!();
        let re = regex!(r"day\d{2}");
        let m = re
            .find(path)
            .expect("macro is only valid inside paths containing 'day\\d{2}' pattern");
        m.as_str()
    }};
}

mod day7 {
    pub fn modname() -> String {
        String::from("day07")
    }

    pub fn macro_modname() -> String {
        String::from(modname!())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_modname() {
            assert_eq!(modname(), "day07");
        }
        #[test]
        fn test_macro_modname() {
            assert_eq!(macro_modname(), "day07");
            assert_eq!(modname!(), "day07");
            //assert_eq!(module_path!(), "day07");
        }
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", format!("{}{}", file!(), line!()).as_str());
    println!("{}", day7::modname());
    let r = regex!(r"\d+");
    let foo = r.find("h123d").unwrap();
    println!("match was {}", foo.as_str());
    let a = regex!(r"(?x)(?P<opname>[-a-zA-Z#+]+)");
    let m = a.find("needle").unwrap();
    println!("m {}", m.as_str());
}
