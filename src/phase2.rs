use std::collections::HashSet;
use crate::DATA_DIR;
use crate::entry::Entry;
use crate::preferences::Preferences;

#[allow(dead_code)]
pub fn phase2() {
    const SIZES: &[u32] = &[6, 10, 20];
    const IDX: &[u32] = &[1, 2, 3, 4];
    // const SIZES: &[u32] = &[6];
    // const IDX: &[u32] = &[1, 2, 3, 4];

    for size in SIZES {
        for idx in IDX {
            let pref = Preferences::from_csv(format!("{}/size{}-{}.txt", DATA_DIR, size, idx));

            let halls = halls_condition(&pref);

            println!("Size {} index {}: Halls condition - {}", size, idx, halls)
        }
    }
}

pub fn halls_condition(pref: &Preferences) -> bool {
    let mut neighbor_set = HashSet::with_capacity(pref.size as usize);

    // Iterate through all subsets
    for p in 0..(1u128 << pref.size) {
        // Size of the subset
        let s_size = p.count_ones();

        neighbor_set.clear();

        for i in 0..pref.size {  // B{i}
            // Is B{i} in this subset
            if p & (1 << i) != 0 {
                neighbor_set.extend(pref.get_all_hashset(&Entry::B(i)));
            }
        }

        let halls = s_size <= neighbor_set.len() as u32;

        // println!("Subset {:b} has neighbors {:?}, Length {}", p, neighbor_set, if halls { "OK" } else { "INSUFFICIENT" });

        if !halls { return false; }
    }

    true
}