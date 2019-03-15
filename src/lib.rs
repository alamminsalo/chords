#[macro_use]
extern crate serde_json;

mod attribute;
pub mod chord;
#[cfg(feature = "ffi_c")]
mod ffi;
mod json;
pub mod note;
pub mod scale;
mod util;

use self::chord::Chord;
#[cfg(feature = "ffi_c")]
pub use ffi::*;
use ordered_permutation as op;

// Returns notes in a given key and scale
fn get_notes(keystr: &str, scalestr: &str) -> Vec<(char, i8)> {
    let key = util::str_to_note(keystr);
    let chromatic_notes: Vec<(char, i8)> = scale::chromatic_notes(key);

    let scale = scale::get_scale(scalestr);

    chromatic_notes
        .into_iter()
        .enumerate()
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
    let root_index = notes
        .iter()
        .position(|&note| note == root_note)
        .expect("Failed to find root index!");

    let a = notes[..root_index].to_vec();
    let b = notes[root_index..].to_vec();

    let mut flipped = vec![];
    flipped.extend(b);
    flipped.extend(a);

    let chromatic_notes: Vec<(char, i8)> = scale::chromatic_notes(root_note);
    let mut intervals = vec![];

    for v in flipped {
        let interval = chromatic_notes
            .iter()
            .position(|&note| note == v || note == util::alt_note(v))
            .expect("Failed to find interval position for note");
        intervals.push(interval as u8);
    }

    let mut permutations = op::permutate(&intervals[1..]);

    // collect indexes in intervals and sort by weight
    permutations.sort_by(|a, b| {
        //collect indexes
        let aa = util::indexes(&a, &intervals);
        let bb = util::indexes(&b, &intervals);

        util::weight_levels(&aa).cmp(&util::weight_levels(&bb))
    });

    for mut p in permutations {
        // require 3 notes
        if p.len() > 1 {
            let weight = util::weight_levels(&util::indexes(&p, &intervals));
            let is_extended = weight > 4;

            if extended || !is_extended {
                // push root note
                p.insert(0, 0);
                let chord = Chord::new(&root_str, p, is_extended, weight);
                if chord.valid {
                    chords.push(chord);
                }
            }
        }
    }

    for chord in chords.iter_mut() {
        chord.format_notes(notes);
    }

    //Return chords
    chords
}

pub fn analyze(key: &str, scale: &str, extended: bool) -> (Vec<String>, Vec<Chord>) {
    //Notes in scale
    let mut notes = get_notes(&key, &scale);

    //Format notes for readability
    match scale {
        "chromatic" => {}
        _ => notes = util::formatted_notes(notes),
    }

    //Chords in scale
    let mut chords: Vec<Chord> = vec![];
    if scale != "chromatic" {
        for v in &notes {
            chords.extend(get_chords(*v, &notes, extended));
        }
    }

    // deduplicate
    chords = util::deduplicate(chords);

    //Return values
    (
        notes
            .into_iter()
            .map(|note| util::note_to_str(note).to_uppercase())
            .collect::<Vec<String>>(),
        chords,
    )
}

pub fn analyze_json(key: &str, scale: &str, extended: bool) -> String {
    let result = analyze(key, scale, extended);
    json::serialize(result.0, result.1)
}

// Return lib supported scales
pub fn supported_scales() -> Vec<String> {
    scale::supported_scales()
}

pub fn supported_scales_json() -> String {
    json!({ "scales": json!(scale::supported_scales()) }).to_string()
}
