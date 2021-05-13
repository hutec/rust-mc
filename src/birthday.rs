use rand::prelude::*;
use std::collections::HashSet;

pub fn birthday_sharing() {
    let mut rng = thread_rng();
    let n_people = 23; 
    let trials = 1_000_000;
    let mut success = 0;
    for _ in 0..trials {
        let mut birthdays : HashSet<i32> = HashSet::new();
        for _ in 0..n_people {
            let birthday : i32 = rng.gen_range(0..365);
            if birthdays.contains(&birthday) {
                success += 1;
                break;
            } else {
                birthdays.insert(birthday);
            }
        } 
    }
    println!("Probability of {} people sharing a birthday is {}", n_people, success as f32 / trials as f32);
}