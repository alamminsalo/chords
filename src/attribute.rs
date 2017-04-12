
pub struct Attributes {
    attr: Vec<String>
}

impl Attributes {

    pub fn new() -> Attributes {
        Attributes{attr: vec![]}
    }

    pub fn push(&mut self, s: &str) {
        self.attr.push(s.to_string());
    }

    pub fn contains(&self, key: &str) -> bool {
        let s = key.to_string();
        self.attr.contains(&s)
    }

    pub fn push_interval(&mut self, interval: u8) {
        self.push(match interval {
            /// 1st ???
            1 => "odd",
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

            0 | _ => "root"
        });
    }

    //Resolve attributes to chord name
    pub fn resolve(&mut self) -> String {
        let mut val = String::new();

        // m/M3/dim/sus2/sus4/no3 resolve
        if self.contains("b3") {
            if self.contains("b5") {
                val.push_str("dim");
            }
            else {
                val.push_str("m");
            }
        }
        else if !self.contains("3") {
            //Check sus2, sus4, no3
            if self.contains("sus2") {
                val.push_str("sus2");
            }
            else if self.contains("sus4") {
                val.push_str("sus4");
            }
            else {
                self.push("no3");
            }
        }

        // aug/b5/M5/no5 resolve
        if self.contains("aug") {
            val.push_str("aug");
        }
        else if self.contains("b5") {
            if !self.contains("b3") {
                val.push_str("b5");
            }
        }
        else if self.contains("5") {
            if self.contains("no3") {
                val.push_str("5");
            }
        }
        else {
            self.push("no5");
        }

        // triad check
        if !self.contains("no5") && (self.contains("3") || self.contains("b3")) {
            self.push("triad");
        }

        // 7, 9, 11, 13 stacks
        if self.contains("triad") {
            // 9th
            if self.contains("7") {
                if self.contains("sus2") {
                    // 9th
                    self.push("9");

                    // 11th
                    if self.contains("sus4") {
                        self.push("11");

                        // 13th
                        if self.contains("6") {
                            self.push("13");
                        }
                    }
                }
            }
        }

        let mut suffix = String::new();
        if val.len() > 3 {
            suffix.push_str("/");
        }

        if self.contains("maj7") {
            //Add prefix 'maj'
            suffix.push_str("maj");
        }

        if self.contains("13") {
            suffix.push_str("13");
        }
        else if self.contains("11") {
            suffix.push_str("11");
        }
        else if self.contains("9") {
            suffix.push_str("9");
        }
        else if self.contains("7") {
            suffix.push_str("7");
        }
        else if self.contains("maj7") {
            suffix.clear();
            suffix.push_str("maj7");
        }
        else {
            //Clear suffix
            suffix = String::new();
        }

        if self.contains("6") {
            if suffix.len() > 0 || val.len() > 3 {
                // check add6
                suffix.push_str("add6");
            }
            else {
                suffix.push_str("6");
            }
        }

        val.push_str(suffix.as_ref());

        // no3, no5
        if self.contains("no3") {
            val.push_str("no3");
        }
        if self.contains("no5") {
            val.push_str("no5");
        }

//        println!("Resolved ({}) as {}", self.attr.join(","), val);

        //Return finished string value
        val
    }
}
