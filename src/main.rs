use music_sheet_gen::{
    lily::Lilypond,
    lsystem::{interpret::*, *},
    notation::*,
};

fn main() {
    // Create defined L-System
    let mut ls = CSSLSystem::new(
        "F++++F--F++F----F".to_owned(),
        &[
            "F -> F % 1/2",
            "F -> FF % 1/15",
            "F -> F+F % 1/15",
            "F -> F-F % 1/15",
            "FF -> [Fd+F-F] % 1/40",
            "FF -> [Fd-F+F] % 1/40",
            "FF -> [dF+F]F % 1/40",
            "FF -> [dF-F]F % 1/40",
            "F+F -> [Fd+F+F] % 1/40",
            "F-F -> [Fd-F-F] % 1/40",
            "F+F -> [dF+F]++F % 1/40",
            "F-F -> [dF-F]--F % 1/40",
            "F-F -> [Fd++F]--F % 1/40",
            "F-F -> [Fd-F]++F % 1/40",
            "F+F -> [Fd+++F--F] % 1/40",
            "F+F -> [Fd----F++F] % 1/40",
        ],
    );

    // 3 L-System rewrites.
    (0..10).for_each(|_| ls.step());

    // Create musical interpreter.
    let mint = MusicInterpret::new(MusicIntInfo {
        key: KeySignature {
            note: NoteName::D,
            signature_type: KeySignatureType::Maj,
        },
        first_note: Note {
            pitch: Pitch {
                note_name: NoteName::D,
                octave: Octave::O5,
                accidental: None,
            },
            duration: Duration::D1,
        },
    });

    // Interpret the generated string into a score
    let score = mint.translate(ls.state().word());

    // Convert score into Lilypond structure.
    let lily: Lilypond = score.into();

    // Print out the lilypond format.
    eprintln!("WORD: {}", ls.state().word());
    println!("{lily}");
}
