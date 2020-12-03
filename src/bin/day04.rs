extern crate combine;

use std::io::{self, Read};
use std::collections::BTreeMap;

use combine::*;
use combine::parser::char::{char, spaces, string};
use combine::parser::choice::choice;
use combine::parser::range::take_while1;
use combine::stream::state::State;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct DateTime {
    year: u32,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Action {
    ShiftStart{ guard: u32 },
    FallAsleep,
    WakeUp,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Event {
    time: DateTime,
    action: Action,
}

fn parse(input: &str) -> Vec<Event> {
    let num_u32 = || from_str(take_while1(|c: char| c.is_digit(10)));
    let num_u16 = || from_str(take_while1(|c: char| c.is_digit(10)));
    
    let date = (num_u32().skip(char('-')), num_u16().skip(char('-')), num_u16());
    let time = (num_u16().skip(char(':')), num_u16());
    let timestamp = between(char('['), char(']'), (date.skip(spaces()), time)).map(|(date, time)| {
        DateTime{
            year: date.0,
            month: date.1,
            day: date.2,
            hour: time.0,
            minute: time.1,
        }
    });

    let action = choice((
            (string("Guard #"), num_u32().skip(string(" begins shift"))).map(|(_, num)| Action::ShiftStart{guard: num}),
            string("falls asleep").map(|_| Action::FallAsleep),
            string("wakes up").map(|_| Action::WakeUp),
            ));

    let line = (timestamp.skip(spaces()), action.skip(spaces())).map(|(ts, ac)| Event{time:ts, action:ac});
    let mut file = many1(line).skip(eof());

    match file.parse(State::new(input)) {
        Ok((val, _)) => val,
        Err(err) => {
            println!("Parse error: {}", err);
            Vec::new()
        }
    }
}

fn add_minutes(map: &mut BTreeMap<u32, Vec<u32>>, guard: u32, start_min: usize, end_min: usize) {
    let vec = map.entry(guard).or_insert_with(|| vec![0;60]);
    for minute in vec[start_min..end_min].iter_mut() {
        *minute += 1;
    }
}

fn get_minutes_asleep(events: &[Event]) -> BTreeMap<u32, Vec<u32>> {
    let mut map = BTreeMap::<u32, Vec<u32>>::new();

    let mut current_guard = None;
    let mut sleep_minute = None;
    
    for event in events {
        match event.action {
            Action::ShiftStart{guard} => {
                if let (Some(min), Some(guard)) = (sleep_minute, current_guard) {
                    add_minutes(&mut map, guard, min as usize, 60);
                }
                sleep_minute = None;
                current_guard = Some(guard);
            },
            Action::FallAsleep => sleep_minute = Some(event.time.minute),
            Action::WakeUp => {
                if let (Some(min), Some(guard)) = (sleep_minute, current_guard) {
                    add_minutes(&mut map, guard, min as usize, event.time.minute as usize);
                    sleep_minute = None;
                }
            },
        }
    }

    map
}

fn best_guard_hash(map: &BTreeMap<u32, Vec<u32>>) -> u32 {
    let mut max_sum = 0;
    let mut best_guard = 0;

    for (guard, minutes) in map.iter() {
        let sum = minutes.iter().sum();
        if sum > max_sum {
            max_sum = sum;
            best_guard = *guard;
        }
    }
    
    let mut max_mins = 0;
    let mut best_minute = 0;
    for (i, &sleep_minutes) in map[&best_guard].iter().enumerate() {
        if sleep_minutes > max_mins {
            max_mins = sleep_minutes;
            best_minute = i;
        }
    }

    best_minute as u32 * best_guard
}

fn most_frequently_asleep_hash(map: &BTreeMap<u32, Vec<u32>>) -> u32 {
    let mut best_guard = 0;
    let mut max_mins = 0;
    let mut best_minute = 0;

    for (&guard, minutes) in map.iter() {
        for (i, &sleep_minutes) in minutes.iter().enumerate() {
            if sleep_minutes > max_mins {
                max_mins = sleep_minutes;
                best_minute = i;
                best_guard = guard;
            }
        }
    }

    best_minute as u32 * best_guard
}

fn main() {
    let stdin = io::stdin();
    let mut locked = stdin.lock();

    let mut input = String::new();
    if locked.read_to_string(&mut input).is_ok() {
        let mut events = parse(&input);
        events.sort();

        let minutes_by_guard = get_minutes_asleep(&events);
        println!("part 1: {}", best_guard_hash(&minutes_by_guard));
        println!("part 2: {}", most_frequently_asleep_hash(&minutes_by_guard));
    }
}
