#[derive(PartialEq, Clone, Copy)]
pub enum Note {
    A,
    As,
    B,
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
}

impl Into<String> for Note {
    fn into(self) -> String {
        match self {
            Note::A => "A",
            Note::As => "A#",
            Note::B => "B",
            Note::C => "C",
            Note::Cs => "C#",
            Note::D => "D",
            Note::Ds => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::Fs => "F#",
            Note::G => "G",
            Note::Gs => "G#",
        }
        .into()
    }
}

impl From<String> for Note {
    fn from(s: String) -> Self {
        match &s[..] {
            "A" => Note::A,
            "A#" => Note::As,
            "B" => Note::B,
            "C" => Note::C,
            "C#" => Note::Cs,
            "D" => Note::D,
            "D#" => Note::Ds,
            "E" => Note::E,
            "F" => Note::F,
            "F#" => Note::Fs,
            "G" => Note::G,
            "G#" => Note::Gs,
            _ => Note::A,
        }
    }
}
