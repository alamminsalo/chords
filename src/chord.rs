use std::fmt;

use util;
use scale;

/// Returns string evaluation from given interval in a chord
fn stringify_interval(interval: u8) -> String {
    String::from(match interval {

        /// 2nd
        2 => "sus2",

        /// 3rd
        3 => "b3",   // Minor 3rd 
        4 => "3",    // Major 3rd

        /// 4th
        5 => "sus4",

        /// 5th
        6 => "b5",  //Diminished 5th
        7 => "5",        //Major 5th
        8 => "aug",     //Augmented 5th

        /// 6th
        9 => "6",

        /// 7th
        10 => "7",      //Dominant 7th
        11 => "maj7",   //Major 7th

        _ => ""
    })
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

        ///TODO: better logic

        // 3rd and 5th
        if self.contains("b3") {
            if self.contains("b5") {
                //Dim chord
                val.push_str("dim");
            }
            else {
                //Minor chord
                val.push_str("m");
            }
        }

        //Aug check
        else if self.contains("aug") {
            //Augmented triad
            val.push_str("aug");
        }

        //Flat 5
        else if self.contains("b5") {
            val.push_str("b5");
        }

        // no5 
        else if !self.contains("5") {
            val.push_str("no5");
        }

        // No 3rd
        else {
            if !self.contains("3") 
                && !self.contains("sus2")
                    && !self.contains("sus4") {
                        val.push_str("no3");
                    }
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

        // 6th
        else if self.contains("6") {
            //6th note
            val.push_str("6");
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

pub struct Chord {
    name: String,
    notes: Vec<String>,
    pub extended: bool
}

impl Chord {

    ///Constructor from given root note and interval vec
    pub fn new(root: &String, intervals: Vec<u8>, extended: bool) -> Chord {

        let mut name = String::new();
        let mut notes = vec![];

        name.push_str(&root.to_uppercase());

        let root_note = util::str_to_note(&root);
        let chromatic = scale::chromatic_notes(root_note);
        let mut attr = Attributes{attr: vec![]};

        //Pick notes from chromatic scale according to interval values
        for interval in intervals.iter() {
            notes.push(util::note_to_str(chromatic[*interval as usize]));
            attr.push(stringify_interval(*interval as u8));
        }

        //Push attributes to name
        name.push_str(attr.resolve().as_ref());

        if extended {
            name.push('*');
        }

        Chord{name: name, notes: notes, extended: extended}
    }

    // Formats notes according to given src of notes
    pub fn format_notes(&mut self, src: &Vec<(char,i8)>) {
        let strings = src.iter().map(|&note| util::note_to_str(note)).collect::<Vec<String>>();
        for el in self.notes.iter_mut(){
            if !strings.contains(el) {
                *el = util::alt_note_str(el.clone());
            }
        }
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0:<10} ({1:})", 
               &self.name, 
               &self.notes.iter().map(|s| s.to_uppercase()).collect::<Vec<String>>().join(", "))
    }
}

