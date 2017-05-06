
pub struct Attributes {
    intervals: Vec<u8>
}

impl Attributes {

    pub fn new() -> Attributes {
        Attributes{intervals: vec![]}
    }

    fn has(&self, a: &[u8]) -> bool {
        a.iter().all(|x| self.intervals.contains(x))
    }

    pub fn push_interval(&mut self, interval: u8) {
        self.intervals.push(interval);
    }

    pub fn remove(&mut self, i: u8) {
        self.intervals = self.intervals.iter()
            .filter(|&x| *x != i)
            .map(|&x| x)
            .collect::<Vec<u8>>();
    }

    fn resolve_nth(&self) -> String {
        let mut val = String::new();

        // 7th stack
        if self.has(&[10]) || self.has(&[11]) {

            // M prefix
            if self.has(&[11]) {
                val.push_str("M");
            }

            // 13
            if self.has(&[2,5,9]) {
                val.push_str("13");
            }
            // 11
            else if self.has(&[2,5]) {
                val.push_str("11");
            }
            // 9
            else if self.has(&[2]) {
                val.push_str("9");

                if self.has(&[9]) {
                    val.push_str("add13");
                }
            }
            // minor 9
            else if self.has(&[1]) {
                val.push_str("m9");

                if self.has(&[9]) {
                    val.push_str("add13");
                }
            }

            // 7
            else {
                val.push_str("7");

                if self.has(&[5]) {
                    val.push_str("add11");

                    if self.has(&[9]) {
                        val.push_str(",13");
                    }
                }

                else if self.has(&[9]) {
                    val.push_str("add13");
                }
            }
        }

        // not 7
        else {
            // 6
            if self.has(&[9]) {
                val.push_str("add6");
            }

            // adds
            if self.has(&[2]) || self.has(&[1]) {
                val.push_str("add9");
            }

            else if self.has(&[5]) {
                val.push_str("add11");
            }
        }

        val
    }

    //Resolve attributes to chord name
    pub fn resolve(&mut self) -> String {
        let mut has3 = true;
        let mut val = String::new();

        // 3 or m3
        if self.has(&[4]) || self.has(&[3]) {

            // m3
            if self.has(&[3]) {
                // dim
                if self.has(&[6]) {
                    // dim7
                    if self.has(&[9]) {
                        self.remove(9);
                    }
                    val.push_str("dim");
                }
                // m
                else {
                    val.push_str("m");
                }
            }

            // 6
            if &val != "dim" && self.has(&[7,9]) {
                val.push_str("6");
                self.remove(9);
            }
        }

        // sus2
        else if self.has(&[2]) {
            val.push_str("sus2");
            self.remove(2);
        }

        // sus4
        else if self.has(&[5]) {
            val.push_str("sus4");
            self.remove(5);
        }

        // 5
        else if self.has(&[7]) {
            val.push_str("5");
        }

        else {
            has3 = false;
        }

        // aug check
        if self.has(&[8]) {
            if self.has(&[7]) || self.has(&[6]) {
                val.push_str("+aug");
            }
            else {
                val.push_str("aug");
            }
        }

        let has5 = !(!self.has(&[6]) && !self.has(&[7]) && !self.has(&[8]));

        let nth = self.resolve_nth();

        //additional markings
        if nth.len() > 0 || !has5 || !has3 {
            val.push_str("(");
        }

        // nth
        if nth.len() > 0 {
            val.push_str(nth.as_ref());
        }

        // no5
        if !has5 {
            val.push_str("no5");
        }

        // no3
        if !has3 {
            val.push_str("no3");
        }

        //close add. markings
        if nth.len() > 0 || !has5 || !has3 {
            val.push_str(")");
        }

        val
    }
}
