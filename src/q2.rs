use std::collections::HashMap;

use crate::input::input;

pub fn q2() {
    let mut analytics = HashMap::new();
    loop {
        let i: i64 = input(None::<&str>);
        if i == -100 {
            break;
        } else if i < 0 || i > 9 {
            return;
        }
        analytics.entry(i).and_modify(|i| *i += 1).or_insert(1);
    }
    let mut sorted = analytics.iter().map(|i| (*i.0, *i.1)).collect::<Vec<_>>();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    println!("Character Analysis:");
    for (i, j) in sorted {
        println!("{i}: {} ({j})", "*".repeat(j as usize));
    }
}
