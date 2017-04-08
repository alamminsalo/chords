use std::fmt;

use util;
use scale;

/// Returns string evaluation from given interval in a chord
fn stringify_interval(interval: u8) -> String {
    String::from(match interval {

        /// 2nd
        2 => "sus2",

        /// 3rd
        3 => "m",   // Minor 3rd 
        4 => "",    // Major 3rd

        /// 4th
        5 => "sus4",

        /// 5th
        6 => "Dim5",  //Diminished 5th
        7 => "",        //Major 5th
        8 => "Aug",     //Augmented 5th

        /// 7th
        10 => "7",      //Dominant 7th
        11 => "maj7",   //Major 7th

        _ => ""
    })
}

pub struct Chord {
    name: String,
    notes: Vec<String>
}

fn attr_contains(attr: &Vec<String>, key: &str) -> bool {
    let s = key.to_string();
    attr.contains(&s)
}

fn resolve_attributes(attr: Vec<String>) -> String {
    let mut val = String::new();

    // Suspended 2th and 4th
    if attr_contains(&attr, "sus2") {
        val.push_str("sus2");
    }

    if attr_contains(&attr, "sus4") {
        val.push_str("sus4");
    }

    // 3rd and 5th
    if attr_contains(&attr, "m") 
        && attr_contains(&attr, "Dim5") {
            //Dim chord
            val.push_str("dim");
        }
    else if attr_contains(&attr, "m") {
        //Minor chord
        val.push_str("m");
    }
    else if attr_contains(&attr, "Aug") {
        //Augmented triad
        val.push_str("aug");
    }

    // 7th
    if attr_contains(&attr, "7") {
        //Dominant 7th
        val.push_str("7");
    }
    else if attr_contains(&attr, "maj7") {
        //Dominant 7th
        val.push_str("M7");
    }

    val
}

impl Chord {

    ///Constructor from given root note and interval vec
    pub fn new(root: String, intervals: Vec<u8>) -> Chord {

        let mut name = String::new();
        let mut notes = vec![];

        name.push_str(&root.to_uppercase());

        let root_note = util::str_to_note(&root);
        let chromatic = scale::chromatic_notes(root_note);
        let mut attr: Vec<String> = vec![];

        //Pick notes from chromatic scale according to interval values
        for interval in intervals.iter() {
            let note = chromatic[*interval as usize];
            notes.push(util::note_to_str(note));

            attr.push(stringify_interval(*interval as u8));
        }

        //Push attributes to name
        name.push_str(resolve_attributes(attr).as_ref());

        Chord{name: name, notes: notes}
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : {}" , &self.name, &self.notes.join(" "))
    }
}

#[test]
fn chord_test() {
    println!("{}", Chord::new("C".to_string(), vec![0,2,8,11]));
}

