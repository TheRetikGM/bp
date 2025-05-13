# Bakalářská práce

- Univerzita (university): **VYSOKÉ UČENÍ TECHNICKÉ V BRNĚ** (BRNO UNIVERSITY OF TECHNOLOGY)
- Jméno (name): **Hudební notace pro klavír jako formální jazyk**
- Autor (author): **Jakub Kloub**
- xlogin: **xkloub03**
- Vedoucí práce (supervisor): **prof. RNDr. ALEXANDR MEDUNA, CSc.**
- Datum (date): **BRNO 2025**
- Odkaz na [github](https://github.com/theretikgm/bp) (kvůlí assetům)


## Otestováno

- Prostředí: **Fedora 42** x86_64

## Požadavky - Linux

- Instalace nástroje `rustup`

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

- Instalace a povolení potřebného `Rust` překladače

    ```bash
    rustup install nightly-2025-02-17
    rustup override set nightly-2025-02-17
    rustup component add rustfmt --toolchain nightly-2025-02-17
    rustup component add clippy --toolchain nightly-2025-02-17
    ```

- Program `LilyPond` > 2.23

    ```bash
    dnf install lilypond
    ```

- Program `FluidSynth`

    ```bash
    dnf install fluidsynth
    ```

- `alsa-lib`

    ```bash
    dnf install alsa-lib
    ```

## Požadavky - Linux - Alernativa bez funkčního zvuku

- Použití nástroje `nix`

    ```bash
    nix develop "."
    ```

## Sestavení a spuštění

```bash
cargo test
cargo run -- assets/Nice-Steinway-Lite-v3.0.sf2
```

## Adresářová struktura

```bash
src ............................................... Hlavní zdrojové soubory aplikace
├── error.rs ...................................... Obecné chyby aplikace a návratový typ Result
├── ext.rs ........................................ Rozšíření pro různé datové typy
├── gui ........................................... Modul grafického rozhraní
│   ├── gui_app.rs ................................ Hlavní aplikace pro EFrame
│   ├── utils ..................................... Nástroje pro GUI
│   │   └── texture.rs ............................ Wrapper pro egui texturu
│   ├── utils.rs .................................. Modulový souboru nástrojů GUI
│   ├── widgets ................................... Definice GUI prvků
│   │   ├── audio_player.rs ....................... Widget pro AudioController
│   │   ├── axiom_edit.rs ......................... Widget pro úpravu axiomu
│   │   ├── length_selector.rs .................... Widget pro úpravu délky doby
│   │   ├── note_name_selector.rs ................. Widget pro zvolení jména noty
│   │   ├── octave_selector.rs .................... Widget pro zvolení oktávy
│   │   ├── rule_edit.rs .......................... Widget pro úpravu pravidel
│   │   └── rule_sums.rs .......................... Widget součtů pravděpodobností praidel
│   ├── widgets.rs ................................ Reexport widgetů
│   ├── windows ................................... Okna která lze dokovat
│   │   ├── control_panel.rs ...................... Okno s řízením genereace not
│   │   ├── grammar_edit.rs ....................... Okno s úpravou gramatiky L-systému
│   │   ├── interpret_parameters.rs ............... Okno s úpravou parametrů interpreteru
│   │   ├── logger.rs ............................. Okno s loggerem
│   │   ├── score_visualizer.rs ................... Okno zobrazující skóre
│   │   └── statistics.rs ......................... Okno zobrazující statistiky L-systému
│   └── windows.rs ................................ Reexport oken
├── gui.rs ........................................ Reexport GUI aplikace
├── lib.rs ........................................ Deklarování všech modulů aplikace
├── lily .......................................... Modul interní reprezentace LilyPond
│   ├── lily_note.rs .............................. Definice LilyPond noty a spojených struktur
│   ├── lilypond.rs ............................... Definice LilyPond skóre
│   ├── lily_stave.rs ............................. Definice LilyPond oktávy
│   └── lily_symbol.rs ............................ Definice LilyPond symoblů
├── lily.rs ....................................... Definice enumerací LilyPondu
├── lsystem ....................................... Modul LSystem
│   ├── interpret ................................. Modulu Interpret
│   │   └── scale.rs .............................. Definice stupnic
│   ├── interpret.rs .............................. Definice interpreteru
│   ├── l_rewriter.rs ............................. Definice LRewriter
│   ├── l_rule.rs ................................. Definice LRule
│   ├── l_rule_set.rs ............................. Definice LRuleSet
│   └── l_system.rs ............................... Definice LSystem
├── lsystem.rs .................................... Reexport tříd modulu LSystem
├── main.rs ....................................... Vstpní bod programu - vytvoření EFrame
├── notation ...................................... Obecná interní reprezentace skóre
│   ├── score.rs .................................. Definice skóre
│   ├── stave.rs .................................. Definice oktávy
│   └── symbol.rs ................................. Definice symbolů
├── notation.rs ................................... Reexport a definice enumerací skóre
├── sanitizer ......................................Modul Sanitizer
│   ├── lily_sanitizer.rs ......................... Definice sanitizéru LilyPond reprezentace
│   ├── score_sanitizer.rs ........................ Definice sanitizéru obecné reprezentace
│   └── to_pref_synonym.rs ........................ Definice pomocné metody sanitizéru
├── sanitizer.rs .................................. Definice obecného sanitizéru (trait)
├── utils ......................................... Obecné nástroje programu
│   └── audio_controller.rs ....................... Definice audio kontroléru
└── utils.rs ...................................... Reexport nástrojů
tests ............................................. Modulární testy
├── l_rewriter.rs ................................. Testy integrace třídy CSSLRewriter
└── l_rule_set.rs ................................. Testy integrace třídy LRuleSet
```