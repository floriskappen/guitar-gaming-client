use regex::Regex;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NoteEvent {
    pub start_time_seconds: f32,
    pub duration_seconds: f32,
    // pub pitch_midi: usize,
    pub string_index: usize,
    pub fret_index: usize,
    pub identifier: Uuid
}

pub fn note_to_offset(note: &str) -> Option<usize> {
    match note {
        "C"  => Some(0),
        "Db" => Some(1),
        "D"  => Some(2),
        "Eb" => Some(3),
        "E"  => Some(4),
        "F"  => Some(5),
        "Gb" => Some(6),
        "G"  => Some(7),
        "Ab" => Some(8),
        "A"  => Some(9),
        "Bb" => Some(10),
        "B"  => Some(11),
        _ => None,
    }
}

/// "Ab7" -> 92 (formula: (Octave + 1) * 12 + Note Offset)
pub fn note_with_octave_string_to_midi_pitch(note: &str) -> Option<usize> {
    let re = Regex::new(r"(?P<note>[A-Ga-g][b#]?)(?P<octave>-?\d)").unwrap();

    if let Some(caps) = re.captures(note) {
        let note = caps.name("note").unwrap().as_str();
        let octave_str = caps.name("octave").unwrap().as_str();

        if let Ok(octave) = octave_str.parse::<usize>() {
            if let Some(note_offset) = note_to_offset(note) {
                return Some((octave + 1) * 12 + note_offset)
            }
        }
    }

    None
}
