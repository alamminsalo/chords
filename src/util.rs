/** Note conversion utils **/

/// Formats notes for easier readability
/// (eg. A, A# -> A, B♭)
/// Highly´experimental
pub fn formatted_notes(notes: Vec<(char, i8)>) -> Vec<(char, i8)> {
    let mut formatted: Vec<(char,i8)> = vec![];

    for note in notes {
        //If note key exists in formatted, attempt to add alt note
        if formatted.iter().position(|&v| v.0 == note.0) != None {
            formatted.push(alt_note(note));
        }

        else {
            formatted.push(note);
        }
    }

    formatted
}

/// Returns alt note
/// (eg. A# -> B♭)
fn alt_note(note: (char, i8)) -> (char, i8) {
    let notestr = note_to_str(note);
    str_to_note(match &notestr[..] {
        "a#" => "b♭",
        "b♭" => "a#",
        "b#" => "c",
        "c♭" => "b",
        "c#" => "d♭",
        "d♭" => "c#",
        "d#" => "e♭",
        "e♭" => "d#",
        "e#" => "f",
        "f"  => "e#",
        "f#" => "g♭",
        "g♭" => "f#",
        "g#" => "a♭",
        _ => &notestr
    })
}

/// Returns scale in good readable format
pub fn formatted_scale(scale: &str) -> String {
    String::from(match scale {
        "maj" | "major"  => "Major",
        "min" | "minor"  => "Minor",
        "hmin" | "harmonic minor"  => "Harmonic Minor",
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


