use std::env;

mod scale;
mod util;

/// Returns notes in a given key and scale
fn get_notes(keystr: &str, scalestr: &str) -> Vec<(char, i8)> {

    let key = util::str_to_note(keystr);
    let chromatic_notes: Vec<(char, i8)> = scale::chromatic_notes(key);

    ///Return notes filtered by scale
    let scale = scale::get_scale(scalestr);
    let notes = chromatic_notes.into_iter().enumerate()
        .filter(|&(index, _)| scale.contains(&(index as u8)))
        .map(|(_, e)| e)
        .collect();

    notes
}


fn main() {

    ///defaults
    let mut key = String::from("C");
    let mut scale = String::from("chromatic");

    let mut iter = env::args();

    //Skip first arg
    iter.next();

    while let Some(arg) = iter.next() {
        match arg.as_ref() {
            "--key" => {
                key = iter.next().unwrap().to_lowercase();
            }

            "--scale" => {
                scale = iter.next().unwrap().to_lowercase();
            }

            "--help" | _ => {
                print_help();
                return;
            }
        }
    }

    let mut notes = get_notes(&key, &scale);

    //Format notes for readability
    match &scale[..] {
        "chromatic" => { 
            //Do nothing on chromatic
        }

        _ => {
            notes = util::formatted_notes(notes)
        }
    }

    //Print result
    println!("Notes in {} {} scale", &key.to_uppercase(), &util::formatted_scale(&scale));
    for v in notes {
        println!("{}", &util::note_to_str(v));
    }
}

fn print_help() {
    println!("Chords: scales and chordwork utility written in Rust language");
    println!("Syntax: chords [--key key --scale scale]\n");
    println!("Optional params:");
    println!("--key     Root key");
    println!("--scale   Scale for notes");
    println!("--help    Prints help");
    println!("\nBy default prints C chromatic scale");
}

