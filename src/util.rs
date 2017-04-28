/** Note conversion utils **/

/// Formats notes for easier readability
/// (eg. A, A# -> A, B♭)
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
pub fn alt_note(note: (char, i8)) -> (char, i8) {
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
        "a♭" => "g#",
        _ => &notestr
    })
}

pub fn alt_note_str(note: String) -> String {
    note_to_str(alt_note(str_to_note(&note)))
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
            '#' | 's' => 1,
            '♭' | 'b' => -1,
            _ => 0
        }
    }

    a
}

pub fn indexes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().map(|&x| b.iter().position(|&y| x == y).unwrap() as u8).collect()
}

pub fn weight_levels(a: &[u8]) -> i8 {
    let mut result = 0;

    result += match a.len() as u8 {
        2 => 0,
        3 => 2,
        4 => 4,
        5 => 6,
        _ => 8 
    };

    for v in a {
        result += match *v {
            1 => 4,
            2 => -2,
            3 => 5,
            4 => -3, 
            5 => 4, 
            6 => 2, 
            _ => 9
        };
    }
    
    result
}

