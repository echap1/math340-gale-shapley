extern crate core;

use std::collections::HashSet;

use crate::entry::Entry;
use crate::pairings::{Pairing, Pairings};
use crate::preferences::Preferences;

mod preferences;
mod entry;
mod pairings;

const DATA_DIR: &str = "data";
const OUT_DIR: &str = "out";
const SIZES: &[u32] = &[6, 10, 25, 100];

fn main() {
    for size in SIZES {
        let pref = Preferences::from_csv(format!("{}/size_{}_priorities.csv", DATA_DIR, size));

        println!("\n\n##################\nSIZE {}:", size);

        let solution_b_to_r = optimal(&pref, true);
        if get_rogues(&solution_b_to_r, &pref).get().len() == 0 {
            println!("Solution (B->R) is stable!");
        } else {
            println!("Solution (B->R) is not stable!");
        }
        solution_b_to_r.write_to_file(format!("{}/size_{}_solution_b_to_r.txt", OUT_DIR, size));

        let solution_r_to_b = optimal(&pref, false);
        if get_rogues(&solution_r_to_b, &pref).get().len() == 0 {
            println!("Solution (R->B) is stable!");
        } else {
            println!("Solution (R->B) is not stable!");
        }
        solution_r_to_b.write_to_file(format!("{}/size_{}_solution_r_to_b.txt", OUT_DIR, size));

        if solution_b_to_r.get() == solution_r_to_b.get() {
            println!("Solution B->R == R->B");
        } else {
            println!("Solution B->R != R->B");
        }

        for idx in 0..=3 {
            let pairings = Pairings::from_csv(format!("{}/size_{}_parings_{}.csv", DATA_DIR, size, idx));
            // println!("{}: {:?}\n\n\n", idx, get_rogues(&pairings, &pref));
            let rogues = get_rogues(&pairings, &pref);
            if rogues.get().len() == 0 {
                println!("Pairing idx {} is stable!", idx);
            } else {
                rogues.write_to_file(format!("{}/size_{}_rogues_{}.txt", OUT_DIR, size, idx));
            }
        }

        println!("##################");
    }
}

pub fn get_rogues(pairings: &Pairings, prefs: &Preferences) -> Pairings {
    let mut rogues = HashSet::new();

    // For every pairing
    for p in pairings.get() {
        // Loop over all other r's
        for r2 in 0..prefs.size {
            if r2 == p.r { continue }
            let r2 = Entry::R(r2);

            // Does B prefer new R
            if p.prefers(&r2, prefs) {
                // Does new R prefer b
                if pairings.with(&r2).unwrap().prefers(&p.b(), prefs) {
                    rogues.insert(Pairing::from(p.b(), r2));
                }
            }
        }
    }

    Pairings::new(rogues)
}

pub fn optimal(prefs: &Preferences, proposer_is_b: bool) -> Pairings {
    let proposer = match proposer_is_b {
        true => { |n| Entry::B(n) }
        false => { |n| Entry::R(n) }
    };
    let acceptor = match proposer_is_b {
        true => { |n| Entry::R(n) }
        false => { |n| Entry::B(n) }
    };

    let mut unpaired_proposers: HashSet<Entry> = HashSet::new();
    let mut unpaired_acceptors: HashSet<Entry> = HashSet::new();
    let mut pairings: Pairings = Pairings::new(HashSet::new());

    for idx in 0..prefs.size {
        unpaired_proposers.insert(proposer(idx));
        unpaired_acceptors.insert(acceptor(idx));
    }

    loop {
        // Pick an unpaired proposer, if there are none we are done
        let proposer = match unpaired_proposers.iter().next().cloned() {
            None => { break }
            Some(v) => { unpaired_proposers.take(&v).unwrap() }
        };

        // Go down the list of preferences until an acceptor wants them
        for pref in prefs.get_all(&proposer) {
            if unpaired_acceptors.take(&pref).is_some() || pairings.with(&pref).unwrap().prefers(&proposer, prefs) {
                match pairings.new_pairing(&pref, proposer) {
                    Some(new_unpaired_proposer) => { unpaired_proposers.insert(new_unpaired_proposer); }
                    None => {}
                }
                break;
            }
        }
    }

    pairings
}