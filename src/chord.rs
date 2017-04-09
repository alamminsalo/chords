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

        /// 6th
        9 => "6",

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

pub struct Attributes {
    attr: Vec<String>
}

impl Attributes {
    fn push(&mut self, s: String) {
        self.attr.push(s);
    }
    //Resolve attributes to chord name
    fn resolve(&self) -> String {

        let mut val = String::new();

        // 3rd and 5th
        if self.contains("m") {
            if self.contains("Dim5") {
                //Dim chord
                val.push_str("dim");
            }
            else {
                //Minor chord
                val.push_str("m");
            }
        }
        
        if self.contains("Aug") {
            //Augmented triad
            val.push_str("aug");
        }

        // 6th
        if self.contains("6") {
            //6th note
            val.push_str("6");
        }

        // Suspended 2th
        if self.contains("sus2") {
            if self.contains("7") {
                // 9th chord
                val.push_str("9");
            }
            else if self.contains("maj7") {
                //Dominant 9th
                val.push_str("M9");
            }
            else {
                //sus2
                val.push_str("sus2");
            }
        }

        // Suspended 4th
        else if self.contains("sus4") {
            if self.contains("7") {
                //Dominant 11th
                val.push_str("11");
            }
            else if self.contains("maj7") {
                //Major 11
                val.push_str("M11");
            }
            else {
                //Sus4
                val.push_str("sus4");
            }
        }

        // 7th
        else if self.contains("7") {
            //Dominant 7th
            val.push_str("7");
        }
        else if self.contains("maj7") {
            //Dominant 7th
            val.push_str("M7");
        }


        //Return finished string value
        val
    }

    fn contains(&self, key: &str) -> bool {
        let s = key.to_string();
        self.attr.contains(&s)
    }
}

impl Chord {

    ///Constructor from given root note and interval vec
    pub fn new(root: &String, intervals: Vec<u8>) -> Chord {

        let mut name = String::new();
        let mut notes = vec![];

        name.push_str(&root.to_uppercase());

        let root_note = util::str_to_note(&root);
        let chromatic = scale::chromatic_notes(root_note);
        let mut attr = Attributes{attr: vec![]};

        //Pick notes from chromatic scale according to interval values
        for interval in intervals.iter() {
            let note = chromatic[*interval as usize];
            notes.push(util::note_to_str(note));

            attr.push(stringify_interval(*interval as u8));
        }

        //Push attributes to name
        name.push_str(attr.resolve().as_ref());

        Chord{name: name, notes: notes}
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}" , &self.name, &self.notes.join(" "))
    }
}

#[test]
fn chord_test() {
    println!("{}", Chord::new("C".to_string(), vec![0,2,8,11]));
}

