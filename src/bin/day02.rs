use std::collections::BTreeMap;
use std::io::{self, BufRead};

fn counts(table: &mut BTreeMap<char, u32>, input: &str) -> (bool, bool) {
    table.clear();
    for ch in input.chars() {
        *table.entry(ch).or_insert(0) += 1;
    }
    let mut has_doubles = false;
    let mut has_triples = false;
    for count in table.values() {
        match count {
            2 => has_doubles = true,
            3 => has_triples = true,
            _ => (),
        }
    }
    (has_doubles, has_triples)
}

fn checksum(lines: &[String]) -> u32 {
    let mut table = BTreeMap::new();
    let mut double_counts = 0;
    let mut triple_counts = 0;
    for line in lines.iter() {
        let (has_doubles, has_triples) = counts(&mut table, line);
        if has_doubles {
            double_counts += 1;
        }
        if has_triples {
            triple_counts += 1;
        }
    }
    return double_counts * triple_counts;
}

fn number_of_differing_chars(s1: &str, s2: &str) -> u32 {
    s1.chars()
        .zip(s2.chars())
        .map(|(ch1, ch2)| if ch1 != ch2 { 1 } else { 0 })
        .sum()
}

fn similar_strings(lines: &[String]) -> Option<(String, String)> {
    for (i, s1) in lines.iter().enumerate() {
        for s2 in lines[i + 1..].iter() {
            if number_of_differing_chars(s1, s2) == 1 {
                return Some((s1.clone(), s2.clone()));
            }
        }
    }
    None
}

fn print_common_chars(s1: &str, s2: &str) {
    for (ch1, ch2) in s1.chars().zip(s2.chars()) {
        if ch1 == ch2 {
            print!("{}", ch1);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().flat_map(|x| x).collect::<Vec<_>>();
    println!("part 1: {}", checksum(&lines));
    print!("part 2: ");
    match similar_strings(&lines) {
        Some((s1, s2)) => print_common_chars(&s1, &s2),
        None => print!("No similar strings found"),
    }
    println!("");
}
