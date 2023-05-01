use std::cmp::max;
use std::collections::HashMap;
use csv::Trim;
use serde::Deserialize;
use crate::entry::Entry;

#[derive(Debug)]
pub struct Preferences {
    pref_map: HashMap<Entry, HashMap<Entry, u32>>,
    pub size: u32
}

#[derive(Deserialize, Debug)]
struct PrefRecord {
    name: String,
    pref_list: Vec<String>
}

impl Preferences {
    pub fn get(&self, k: &Entry, v: &Entry) -> u32 {
        *self.pref_map.get(k).unwrap().get(v).unwrap()
    }

    pub fn get_all(&self, k: &Entry) -> Vec<Entry> {
        let prefs = self.pref_map.get(k).unwrap();
        let mut entries = vec![None; self.size as usize];

        for (e, idx) in prefs {
            entries[*idx as usize] = Some(*e);
        }

        entries.iter().map(|e| e.unwrap()).collect()
    }

    pub fn from_csv(filename: String) -> Self {
        let mut pref_map: HashMap<Entry, HashMap<Entry, u32>> = HashMap::new();
        let mut size = 0u32;

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b',')
            .flexible(true)
            .double_quote(false)
            .trim(Trim::All)
            .from_path(filename)
            .unwrap();

        for result in rdr.deserialize() {
            let record: PrefRecord = result.unwrap();
            // println!("{:?}", record);

            let n = record.name[1..].trim_end_matches(':').parse::<u32>().unwrap();
            let is_b = record.name.starts_with("B");

            size = max(size, n);

            let mut p_idx = 0u32;

            let mut map = HashMap::new();

            for p in record.pref_list {
                if p.len() <= 1 { continue }
                let p_n = p[1..].parse::<u32>().unwrap();

                map.insert(p_n, p_idx);
                p_idx += 1;
            }

            if is_b { pref_map.insert(Entry::B(n), map.iter().map(|e| (Entry::R(*e.0), *e.1)).collect()); }
            else { pref_map.insert(Entry::R(n), map.iter().map(|e| (Entry::B(*e.0), *e.1)).collect()); }
        }

        Self {
            pref_map,
            size: size + 1
        }
    }
}