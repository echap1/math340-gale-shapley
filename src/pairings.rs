use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Write;
use csv::Trim;
use crate::entry::Entry;
use crate::preferences::Preferences;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Pairing {
    pub b: u32,
    pub r: u32
}

pub struct Pairings {
    pairings: HashSet<Pairing>
}

impl Pairing {
    pub fn new(b: u32, r: u32) -> Self {
        Self {
            b,
            r,
        }
    }

    pub fn from(e1: Entry, e2: Entry) -> Self {
        match e1 {
            Entry::B(b) => {
                match e2 {
                    Entry::B(_) => { panic!("Invalid entries!") }
                    Entry::R(r) => { Self::new(b, r) }
                }
            }
            Entry::R(r) => {
                match e2 {
                    Entry::B(b) => { Self::new(b, r) }
                    Entry::R(_) => { panic!("Invalid entries!") }
                }
            }
        }
    }

    pub fn b(&self) -> Entry { Entry::B(self.b) }

    pub fn r(&self) -> Entry { Entry::R(self.r) }

    pub fn prefers(&self, e: &Entry, prefs: &Preferences) -> bool {
        match e {
            Entry::B(b2) => { prefs.get(&self.r(), &self.b()) > prefs.get(&self.r(), &Entry::B(*b2)) }
            Entry::R(r2) => { prefs.get(&self.b(), &self.r()) > prefs.get(&self.b(), &Entry::R(*r2)) }
        }
    }
}

impl Debug for Pairing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("('B{}', 'R{}')", self.b, self.r))
    }
}

impl Debug for Pairings {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.pairings.fmt(f)
    }
}

impl Pairings {
    pub fn new(pairings: HashSet<Pairing>) -> Self {
        Self {
            pairings,
        }
    }

    pub fn add(&mut self, pairing: Pairing) {
        self.pairings.insert(pairing);
    }

    pub fn get(&self) -> &HashSet<Pairing> {
        &self.pairings
    }

    pub fn with(&self, e: &Entry) -> Option<Pairing> {
        match e {
            Entry::B(b) => {
                for p in &self.pairings {
                    if &p.b == b { return Some(*p) }
                }
            }
            Entry::R(r) => {
                for p in &self.pairings {
                    if &p.r == r { return Some(*p) }
                }
            }
        }

        None
    }

    pub fn new_pairing(&mut self, maybe_was_paired: &Entry, to_be_paired: Entry) -> Option<Entry> {
        let old_pairing = self.with(maybe_was_paired);

        self.pairings.insert(Pairing::from(*maybe_was_paired, to_be_paired));

        match old_pairing {
            Some(pairing) => {
                let old_pairing = self.pairings.take(&pairing).unwrap();

                match to_be_paired {
                    Entry::B(_) => { Some(Entry::B(old_pairing.b)) }
                    Entry::R(_) => { Some(Entry::R(old_pairing.r)) }
                }
            }
            None => { None }
        }
    }

    pub fn from_csv(filename: String) -> Self {
        let mut pairings: HashSet<Pairing> = HashSet::new();

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b',')
            .flexible(true)
            .double_quote(false)
            .trim(Trim::All)
            .from_path(filename)
            .unwrap();

        for result in rdr.deserialize() {
            let record: (String, String) = result.unwrap();
            // println!("{:?}", record);

            let n1 = record.0[1..].trim_end_matches(':').parse::<u32>().unwrap();
            let n2 = record.1[1..].trim_end_matches(':').parse::<u32>().unwrap();

            if record.0.starts_with("B") {
                pairings.insert(Pairing::new(n1, n2));
            } else {
                pairings.insert(Pairing::new(n2, n1));
            }
        }

        Self {
            pairings
        }
    }

    pub fn write_to_file(&self, filename: String) {
        let mut f = File::create(filename).unwrap();
        writeln!(f, "{:#?}", self).unwrap();
    }
}