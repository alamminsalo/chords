use std::fmt;

use crate::attribute::Attributes;
use crate::scale;
use crate::util;

pub struct Chord {
    pub name: String,
    pub notes: Vec<String>,
    pub extended: bool,
    pub weight: i8,
    pub valid: bool,
}

impl Chord {
    ///Constructor from given root note and interval vec
    pub fn new(root: &String, intervals: Vec<u8>, extended: bool, weight: i8) -> Chord {
        let mut name = String::new();
        let mut notes = vec![];

        name.push_str(&root.to_uppercase());

        let root_note = util::str_to_note(&root);
        let chromatic = scale::chromatic_notes(root_note);
        let mut attr = Attributes::new();

        //Pick notes from chromatic scale according to interval values
        for interval in intervals.iter() {
            notes.push(util::note_to_str(chromatic[*interval as usize]));
            attr.push_interval(*interval as u8);
        }

        //Push attributes to name
        name.push_str(attr.resolve().as_ref());
        let valid = !name.contains("no5no3") && !name.contains("inv3s");

        Chord {
            name: name,
            notes: notes,
            extended: extended,
            weight: weight,
            valid: valid,
        }
    }

    // Formats notes according to given src of notes
    pub fn format_notes(&mut self, src: &Vec<(char, i8)>) {
        let strings = src
            .iter()
            .map(|&note| util::note_to_str(note))
            .collect::<Vec<String>>();
        for el in self.notes.iter_mut() {
            if !strings.contains(el) {
                *el = util::alt_note_str(el.clone());
            }
        }
    }

    pub fn equals(&self, other: &Chord) -> bool {
        if self.notes.len() != other.notes.len() {
            return false;
        }
        for v in self.notes.iter() {
            if !other.notes.contains(v) {
                return false;
            }
        }

        true
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Formatted name
        let mut name = self.name.clone();
        if self.extended {
            name.push('*');
        }

        write!(
            f,
            "{0:<18} ({1:})",
            &name,
            &self
                .notes
                .iter()
                .map(|s| s.to_uppercase())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Clone for Chord {
    fn clone(&self) -> Chord {
        Chord {
            name: self.name.clone(),
            notes: self.notes.clone(),
            extended: self.extended,
            weight: self.weight,
            valid: self.valid,
        }
    }
}
