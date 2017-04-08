use std::env;

mod scale;
mod util;
mod chord;

use chord::Chord;

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

//Returns list of chords a given rootnote can create with given list of notes
fn get_chords(root_note: (char, i8), notes: &Vec<(char, i8)>) -> Vec<Chord> {
    let chords = vec![];

    //Flip vec to root note
    let root_index = notes.iter().position(|&note| note == root_note).unwrap();
    let a = notes[..root_index].to_vec();
    let b = notes[root_index..].to_vec();

    let mut flipped = vec![];
    flipped.extend(b);
    flipped.extend(a);

    let chromatic_notes: Vec<(char, i8)> = scale::chromatic_notes(root_note);
    let mut intervals = vec![];

    for v in &flipped[1..] {
        let interval = chromatic_notes.iter().position(|&note| note == *v).unwrap();
        intervals.push(interval as u8);
    }

    // Triad
    let chord = Chord::new(util::note_to_str(root_note), vec![0, intervals[1], intervals[3]]);
    println!("{}",chord);
    
    // 7th
    chords
}

fn main() {

    ///defaults
    let mut key = String::from("C");
    let mut scale = String::from("maj");

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
    for v in &notes {
        println!("{}", &util::note_to_str(*v));
    }

    //Print chords
    println!("\nChords found:");
    for v in &notes {
        get_chords(*v, &notes);
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

