
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
    let root_index = v.iter().position(|&note| note == root).unwrap();
    let a = v[..root_index].to_vec();
    let b = v[root_index..].to_vec();

    let mut v = vec![];

    v.extend(b);
    v.extend(a);

    v
}

/// Returns scale intervals
pub fn get_scale(scale: &str) -> Vec<u8> {
    interval_to_scale(match scale {
        "maj" | "major"             => major_interval(),
        "min" | "minor"             => minor_interval(),
        "hmin" | "harmonic minor"   => harmonic_minor_interval(),
        "chromatic" | _             => chromatic_interval()
    })
}

#[test]
fn test_scale() {
//
//    println!("\nMajor scale:");
//    for v in get_scale("maj") {
//        println!("{}", v);
//    }
//
//    println!("\nMinor scale:");
//    for v in get_scale("min") {
//        println!("{}", v);
//    }
//
//    println!("\nHarmonic minor scale:");
//    for v in get_scale("hmin") {
//        println!("{}", v);
//    }
//
//    println!("\nChromatic scale:");
//    for v in get_scale("chromatic") {
//        println!("{}", v);
//    }
}

/// Normalizes interval to indexes in chromatic scale
fn interval_to_scale(interval: Vec<u8>) -> Vec<u8> {
    let mut v = vec![];

    //Add root note
    v.push(0);

    let mut index = 0;
    for i in interval {
        index += i;
        v.push(index);
    }

    v
}

fn chromatic_interval() -> Vec<u8> {
    vec![1,1,1,1,1,1,1,1,1,1,1,1]
}

fn major_interval() -> Vec<u8> {
    vec![2,2,1,2,2,2,1]
}

fn minor_interval() -> Vec<u8> {
    vec![2,1,2,2,1,2,2]
}

fn harmonic_minor_interval() -> Vec<u8> {
    vec![2,1,2,2,1,3,1]
}

