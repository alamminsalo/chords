use crate::chord::Chord;
use serde_json::Value;

pub fn serialize(notes: Vec<String>, chords: Vec<Chord>) -> String {
    json!({
        "notes": json!(notes),
        "chords": json!(chords.into_iter().map(|c| serialize_chord(c)).collect::<Vec<Value>>()) })
    .to_string()
}

fn serialize_chord(chord: Chord) -> Value {
    json!({
        "name": chord.name,
        "notes": json!(&chord.notes.iter().map(|c| c.to_uppercase()).collect::<Vec<String>>()),
        "extended": chord.extended
    })
}
