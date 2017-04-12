use std::env;

mod scale;
mod util;
mod chord;
mod attribute;

use chord::Chord;

/// Returns notes in a given key and scale
fn get_notes(keystr: &str, scalestr: &str) -> Vec<(char, i8)> {

    let key = util::str_to_note(keystr);
    let chromatic_notes: Vec<(char, i8)> = scale::chromatic_notes(key);

    ///Return notes filtered by scale
    let scale = scale::get_scale(scalestr);

    chromatic_notes.into_iter().enumerate()
        .filter(|&(index, _)| scale.contains(&(index as u8)))
        .map(|(_, e)| e)
        .collect()
}

//Returns list of chords a given rootnote can create with given list of notes
fn get_chords(root_note: (char, i8), notes: &Vec<(char, i8)>, extended: bool) -> Vec<Chord> {
    let mut chords = vec![];

    //Root note as string presentation
    let root_str = util::note_to_str(root_note);

    //Flip vec to root note
    let root_index = notes.iter().position(|&note| note == root_note)
        .expect("Failed to find root index!");

    let a = notes[..root_index].to_vec();
    let b = notes[root_index..].to_vec();

    let mut flipped = vec![];
    flipped.extend(b);
    flipped.extend(a);

    let chromatic_notes: Vec<(char, i8)> = scale::chromatic_notes(root_note);
    let mut intervals = vec![];

    for v in &flipped[1..] {
        let interval = chromatic_notes.iter()
            .position(|&note| note == *v || note == util::alt_note(*v))
            .expect("Failed to find interval position for note");
        intervals.push(interval as u8);
    }

    // Triads
    if intervals.len() > 3 {
        chords.push(Chord::new(&root_str, vec![0, intervals[1], intervals[3]], false));
        chords.push(Chord::new(&root_str, vec![0, intervals[2], intervals[3]], false));
    }

    // +1
    if intervals.len() > 4 {
        chords.push(Chord::new(&root_str, vec![0, intervals[1], intervals[3], intervals[4]], false));
        chords.push(Chord::new(&root_str, vec![0, intervals[2], intervals[3], intervals[4]], false));

        if extended {
            chords.push(Chord::new(&root_str, vec![0, intervals[1], intervals[2], intervals[3], intervals[4]], true));
        }
    }

    // +2
    if intervals.len() > 5 {
        chords.push(Chord::new(&root_str, vec![0, intervals[1], intervals[3], intervals[5]], false));

        if extended {
            chords.push(Chord::new(&root_str, vec![0, intervals[2], intervals[3], intervals[5]], true));

            // +3
            chords.push(Chord::new(&root_str, vec![0, intervals[1], intervals[3], intervals[4], intervals[5]], true));
            chords.push(Chord::new(&root_str, vec![0, intervals[2], intervals[3], intervals[4], intervals[5]], true));
            chords.push(Chord::new(&root_str, vec![0, intervals[1], intervals[2], intervals[3], intervals[4], intervals[5]], true));
        }
    }

    if intervals.len() > 6 {
        if extended {
            // +4
            chords.push(Chord::new(&root_str, vec![0, 
                                   intervals[1], 
                                   intervals[2], 
                                   intervals[3], 
                                   intervals[4], 
                                   intervals[5], 
                                   intervals[6]],
                                   true));
        }
    }

    for chord in chords.iter_mut() {
        chord.format_notes(notes);
    }

    //Return chords
    chords
}

fn main() {

    ///defaults
    let mut key = String::from("C");
    let mut scale = String::from("major");
    let mut extended = false;

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

            "--extended" => {
                extended = true;
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
    println!("Notes in {} {} scale:", &key.to_uppercase(), &scale::friendly_name(&scale));
    println!("{}", notes.iter().map(|&note| util::note_to_str(note).to_uppercase()).collect::<Vec<String>>().join(" "));

    if &scale[..] != "chromatic" {
        //Get chords in scale
        let mut chords = vec![];
        for v in &notes {
            chords.extend(get_chords(*v, &notes, extended));
        }

        //Print chords
        println!("\nChords found:");
        for c in chords {
            println!("{}", c);
        }
    }
}

fn print_help() {
    println!("Chords: scales and chordwork utility written in Rust language");
    println!("By default yelds C major scale");
    println!("Syntax: chords [--key key --scale scale]");
    println!("Optional params:");
    println!("\t--key       Root key");
    println!("\t--scale     Scale for notes");
    println!("\t--extended  Print extended chords (marked with *)");
    println!("\t--help      Prints help");
    scale::print_supported_scales();
}

