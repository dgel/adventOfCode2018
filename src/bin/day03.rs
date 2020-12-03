extern crate advent_of_code_2018;
extern crate combine;

use advent_of_code_2018::Mat;

use combine::*;
use combine::parser::char::{char, spaces};
use combine::parser::range::take_while1;
use combine::stream::state::State;

use std::io::{self, Read};
use std::collections::BTreeSet;

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn parse(input: &str) -> Vec<Claim> {
    let num = || from_str(take_while1(|c: char| c.is_digit(10)));
    let id = || (char('#'), num()).map(|(_, num)| num);
    let pos = || (num().skip(char(',')), num());
    let size = || (num().skip(char('x')), num());
    let line = || (id().skip(spaces()), char('@').skip(spaces()), pos().skip((char(':'), spaces())), size().skip(spaces())).map(|(id, _, pos, size)| {
        Claim {
            id: id,
            x: pos.0,
            y: pos.1,
            width: size.0,
            height: size.1,
        }
    });
    let mut file = (many1(line()), eof()).map(|(i, _)| i);

    match file.parse(State::new(input)) {
        Ok((val, _)) => val,
        Err(err) => {
            println!("Error: {}", err);
            Vec::new()
        },
    }
}

fn get_num_overlapping(claims: &[Claim]) -> u32 {
    let mut matrix = Mat::<u8>::new(1000, 1000, 0);
    for claim in claims {
        for y in claim.y..(claim.y + claim.height) {
            for x in claim.x..(claim.x + claim.width) {
                matrix[(x,y)] = std::cmp::min(matrix[(x,y)] + 1, 255);
            }
        }
    }

    let mut total = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            if matrix[(x,y)] > 1 {
                total += 1;
            }
        }
    }

    total
}

fn get_non_overlapping(claims: &[Claim]) -> Option<usize> {
    let mut non_overlapping = BTreeSet::new();
    for claim in claims {
        non_overlapping.insert(claim.id);
    }

    let mut matrix = Mat::<usize>::new(1000, 1000, std::usize::MAX);
    for claim in claims {
        for y in claim.y..(claim.y + claim.height) {
            for x in claim.x..(claim.x + claim.width) {
                let current = matrix[(x,y)];
                if current != std::usize::MAX {
                    non_overlapping.remove(&current);
                    non_overlapping.remove(&claim.id);
                } else {
                    matrix[(x,y)] = claim.id;
                }
            }
        }
    }

    if non_overlapping.len() == 1 {
        non_overlapping.iter().next().map(|v| *v)
    } else {
        None
    }
}

fn main() {
    let stdin = io::stdin();
    let mut locked = stdin.lock();

    let mut input = String::new();
    if locked.read_to_string(&mut input).is_ok() {
        let claims = parse(&input);

        println!("part 1: {}", get_num_overlapping(&claims));

        let non_overlapping = get_non_overlapping(&claims);
        match non_overlapping {
            Some(val) => println!("part 2: {}", val),
            None => println!("part 2: no unique non-overlapping claim"),
        }
    }

}

