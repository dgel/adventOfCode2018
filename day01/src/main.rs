use std::io::{self, BufRead};
use std::collections::BTreeSet;

fn read_nums() -> Vec<i64> {
    let stdin = io::stdin();
    let locked = stdin.lock();
    let result = locked.lines().filter_map(|line| line.ok()).filter_map(|line| line.parse::<i64>().ok()).collect();
    result
}


fn find_first_repeat(nums: &[i64]) -> i64 {
    let mut seen = BTreeSet::new();
    let mut total = 0;
    seen.insert(total);
    loop {
        for &num in nums {
            total += num;
            if !seen.insert(total) {
                return total;
            }
        }
    }
}

fn main() {
    let nums = read_nums();
    println!("part 1: {}", nums.iter().sum::<i64>());
    println!("part 2: {}", find_first_repeat(&nums));
}
