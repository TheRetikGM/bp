use music_sheet_gen::{
    gui::GuiApp,
    lily::Lilypond,
    lsystem::{interpret::*, *},
    notation::*,
    sanitizer::Sanitizer,
};

// TODO: REMOVE
fn old_main() {
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
            ext: ExtNoteName {
                note_name: NoteName::D,
                accidental: None,
            },
            signature_type: KeySignatureType::Maj,
        },
        first_note: Note {
            pitch: Pitch {
                ext: ExtNoteName {
                    note_name: NoteName::D,
                    accidental: None,
                },
                octave: Octave::O5,
            },
            duration: NoteLength::L1,
        },
    });

    // Interpret the generated string into a score
    let mut score = mint.translate(ls.state().word());

    // Sanitize the interpreted score to remove any unnecessary accidentals
    Sanitizer::sanitize(&mut score).unwrap();

    // Convert score into Lilypond structure.
    let lily: Lilypond = score.into();

    // Print out the lilypond format.
    eprintln!("WORD: {}", ls.state().word());
    println!("{lily}");
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    egui_logger::builder()
        .max_level(log::LevelFilter::Debug)
        .init()
        .expect("Failed to initialize egui_logger");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        persistence_path: directories::BaseDirs::new().map(|base_dirs| {
            base_dirs
                .data_dir()
                .join("music_sheet_gen")
                .join("data.ron")
        }),
        ..Default::default()
    };
    eframe::run_native(
        "Music sheet generation",
        native_options,
        Box::new(|cc| Ok(Box::new(GuiApp::new(cc)))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    egui_logger::builder()
        .max_level(log::LevelFilter::Debug)
        .init()
        .expect("Failed to initialize egui_logger");

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(GuiApp::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
