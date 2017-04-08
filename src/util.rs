/** Note conversion utils **/

/// Formats notes for easier readability
/// (eg. A, A# -> A, B♭)
/// Highly´experimental
pub fn formatted_notes(notes: Vec<(char, i8)>) -> Vec<(char, i8)> {
    let mut formatted = vec![];

    for note in notes {
        if formatted.len() == 0 {
            formatted.push(note);
        }

        else {
            // If last note key signature matches the current note,
            // format the current note as flat elevated note.
            // Note that this doesn't alter root note
            if formatted[formatted.len() - 1].0 == note.0 && note.1 != 0 {
                let altnote = match note.0 {
                    'a' => ('b', -1),
                    'c' => ('d', -1),
                    'f' => ('g', -1),
                    'g' => ('a', -1),
                    _ => note
                };

                formatted.push(altnote);
            }

            else {
                formatted.push(note);
            }
        }
    }

    formatted
}

/// Returns scale in good readable format
pub fn formatted_scale(scale: &str) -> String {
    String::from(match scale {
        "maj" => "Major",
        "min" => "Minor",
        "hmin" => "Harmonic Minor",
        "chromatic" => "Chromatic",
        _ => scale
    })
}

pub fn note_to_str(note: (char, i8)) -> String {
    let mut s = String::new();

    s.push(note.0);

    match note.1 {
        1 => s.push('#'),

        -1 => s.push('♭'),

        _ => {}
    }

    s
}

pub fn str_to_note(note: &str) -> (char, i8) {
    let n = note.to_lowercase();

    let mut a = (n.chars().nth(0).unwrap(), 0);

    if n.len() > 1 {
        a.1 = match n.chars().nth(1).unwrap() {
            '#' => 1,
            '♭' => -1,
            _ => 0
        }
    }

    a
}


