extern crate chords;

use std::env;

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

    // Run analysis
    let (notes, chords) = chords::analyze(&key, &scale, extended);

    //Print results
    println!("Notes in {} {} scale:", &key.to_uppercase(), &chords::scale::friendly_name(&scale));
    println!("{}\n", notes.join(" "));
    println!("Chords found:");
    for c in chords {
//        println!("{}", c);
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
    chords::scale::print_supported_scales();
}


