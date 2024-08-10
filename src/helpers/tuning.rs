pub const NOTE_STRINGS: [&str; 12] = ["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];

pub fn note_from_pitch(frequency: f32) -> i32 {
    let note_num = 12.0 * (frequency / 440.0).log2();
    (note_num.round() as i32) + 69
}

pub fn frequency_from_note_number(note: i32) -> f32 {
    440.0 * (2.0f32).powf((note as f32 - 69.0) / 12.0)
}

pub fn cents_off_from_pitch(frequency: f32, note: i32) -> i32 {
    let freq_from_note = frequency_from_note_number(note);
    (1200.0 * (frequency / freq_from_note).log2()).round() as i32
}

pub fn octave_from_note(note: i32) -> i32 {
    (note / 12) - 1
}
