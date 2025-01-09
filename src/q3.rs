// 12344321
// 123  321
// 12    21
// 1      1
use crate::input::input;

pub fn q3() {
    let n = input(None::<&str>);
    let mut s = String::new();
    for i in 1..=n {
        s.push_str(&i.to_string());
    }
    let mut strings = Vec::new();
    while !s.trim().is_empty() {
        strings.push(s.clone());
        s = s.trim().to_string();
        s.remove(s.len() - 1);
        s.push_str(&" ".repeat(n - s.len()));
    }
    strings.iter().rev().for_each(|i| {
        println!("{i}{}", i.chars().rev().collect::<String>());
    });
}
