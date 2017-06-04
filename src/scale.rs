use util;

/// Returns all notes on chromatic scale
pub fn chromatic_notes(root: (char, i8)) -> Vec<(char, i8)> {
    let mut v = vec![];

    v.push(('a', 0));   /// a
    v.push(('a', 1));   /// a sharp
    v.push(('b', 0));   /// b
    v.push(('c', 0));   /// c
    v.push(('c', 1));   /// c sharp
    v.push(('d', 0));   /// d
    v.push(('d', 1));   /// d sharp
    v.push(('e', 0));   /// e
    v.push(('f', 0));   /// f 
    v.push(('f', 1));   /// f sharp
    v.push(('g', 0));   /// g
    v.push(('g', 1));   /// g sharp

    ///Split and join according to root note
    let root_index = v.iter().position(|&note| note == root || note == util::alt_note(root)).unwrap();
    let a = v[..root_index].to_vec();
    let b = v[root_index..].to_vec();

    let mut v = vec![];

    v.extend(b);
    v.extend(a);

    v
}

pub fn print_supported_scales() {
    println!("Supported scales:");
    for scale in supported_scales() {
        println!("\t{}", scale);
    }
    println!("Custom scale syntax: --scale 1,2,1,3,2,1");
}

pub fn supported_scales() -> Vec<String> {
    vec!["major", "minor", "harmonicminor", "melodicminor",
    "dorian", "phrygian", "lydian", "locrian", "mixolydian",
    "overtone", "augmented", "wholetone", "pentatonic"
    ].iter().map(|s| s.to_string()).collect()
}

/// Returns scales by name
pub fn get_scale(scale: &str) -> Vec<u8> {
    vec_sum(match scale {
        "major" | "ionian"                      => vec![2,2,1,2,2,2,1],
        "minor" | "naturalminor" | "aeolian"    => vec![2,1,2,2,1,2,2],
        "harmonicminor"                         => vec![2,1,2,2,1,3,1],
        "melodicminor"                          => vec![2,1,2,2,2,2],
        "dorian"                                => vec![2,1,2,2,2,1,2],
        "phrygian"                              => vec![1,2,2,2,1,2,2],
        "lydian"                                => vec![2,2,2,1,2,2,1],
        "locrian"                               => vec![1,2,2,1,2,2,2],
        "mixolydian"                            => vec![2,2,1,2,2,1,2],
        "overtone"                              => vec![2,2,2,1,2,1],
        "augmented"                             => vec![1,3,1,3,1],
        "wholetone"                             => vec![2,2,2,2,2,2],
        "pentatonic"                            => vec![2,2,3,2], 
        "chromatic"                             => vec![1,1,1,1,1,1,1,1,1,1,1,1],
        _                                       => parse_scale(scale)
    })
}

// Parses scale from "1,1,..." format
fn parse_scale(scalestr: &str) -> Vec<u8> {
    scalestr.split(",").map(|s| s.parse::<u8>().unwrap_or(1)).collect()
}

/// Returns array of summed items
fn vec_sum(interval: Vec<u8>) -> Vec<u8> {
    let mut vec: Vec<u8> = vec![0];
    vec.extend(interval.into_iter().scan(0, |sum, step| {
        *sum = *sum + step; 
        Some(*sum)
    }));
    vec
}

pub fn friendly_name(name: &str) -> String {
    String::from(match name {
        "major"                     => "Major",
        "minor" | "naturalminor"    => "Natural minor",
        "hmin" | "harmonicminor"            => "Harmonic minor",
        "augmented"                         => "Augmented",
        "wholetone"                         => "Wholetone",
        "melodicminor"                      => "Melodic minor",
        "overtone"                          => "Overtone",
        "chromatic"                         => "Chromatic",
        _                                   => name
    })
}
