//! Selector tests: sampling rules, objectives, hysteresis and persistence.

use cogwasm_supervisor::{CompileOptions, Measurement, Objective, Preset, Scoreboard};
use tempfile::TempDir;

fn spec(preset: Preset) -> CompileOptions {
    preset.options()
}

fn measure(spec: CompileOptions, exec_ms: f64, size_bytes: u64) -> Measurement {
    Measurement {
        spec,
        compile_ms: Some(100),
        exec_ms: Some(exec_ms),
        size_bytes,
    }
}

#[test]
fn a_spec_needs_the_minimum_number_of_samples() {
    let mut board = Scoreboard::new();
    for _ in 0..3 {
        board.record("p", measure(spec(Preset::Fast), 10.0, 100));
    }
    for _ in 0..2 {
        board.record("p", measure(spec(Preset::Opt), 5.0, 90));
    }
    assert_eq!(
        board.best("p", Objective::Exec, 3),
        Some(spec(Preset::Fast)),
        "the faster spec has too few samples"
    );
    assert_eq!(board.best("p", Objective::Exec, 2), Some(spec(Preset::Opt)));
}

#[test]
fn the_fastest_spec_wins_on_exec() {
    let mut board = Scoreboard::new();
    for _ in 0..3 {
        board.record("p", measure(spec(Preset::Fast), 10.0, 100));
        board.record("p", measure(spec(Preset::Opt), 4.0, 90));
    }
    assert_eq!(board.best("p", Objective::Exec, 3), Some(spec(Preset::Opt)));
    assert_eq!(board.best("p", Objective::Size, 3), Some(spec(Preset::Opt)));
}

#[test]
fn the_smallest_spec_wins_on_size() {
    let mut board = Scoreboard::new();
    for _ in 0..3 {
        board.record("p", measure(spec(Preset::Opt), 4.0, 90));
        board.record("p", measure(spec(Preset::Release), 4.1, 60));
    }
    assert_eq!(
        board.best("p", Objective::Size, 3),
        Some(spec(Preset::Release))
    );
    assert_eq!(board.best("p", Objective::Exec, 3), Some(spec(Preset::Opt)));
}

#[test]
fn balanced_weighs_speed_and_size() {
    let mut board = Scoreboard::new();
    for _ in 0..3 {
        board.record("p", measure(spec(Preset::Fast), 10.0, 100));
        board.record("p", measure(spec(Preset::Opt), 9.0, 200));
        board.record("p", measure(spec(Preset::Release), 12.0, 50));
    }
    assert_eq!(
        board.best("p", Objective::Balanced, 3),
        Some(spec(Preset::Release))
    );
}

#[test]
fn hysteresis_keeps_the_current_spec() {
    let mut board = Scoreboard::new();
    for _ in 0..3 {
        board.record("p", measure(spec(Preset::Fast), 10.0, 100));
        board.record("p", measure(spec(Preset::Opt), 9.6, 90));
    }
    let current = spec(Preset::Fast);
    assert_eq!(
        board.choose("p", Objective::Exec, 3, Some(&current), 0.05),
        Some(spec(Preset::Fast)),
        "a 4% gain is inside the 5% margin"
    );
    assert_eq!(
        board.choose("p", Objective::Exec, 3, Some(&current), 0.02),
        Some(spec(Preset::Opt)),
        "a 4% gain beats a 2% margin"
    );
}

#[test]
fn a_scoreboard_round_trips_through_json() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("profiles.json");
    let mut board = Scoreboard::new();
    board.record("p", measure(spec(Preset::Opt), 5.0, 90));
    board.save(&path).unwrap();

    let loaded = Scoreboard::load(&path).unwrap();
    assert_eq!(loaded.samples("p").len(), 1);
    assert_eq!(
        loaded.best("p", Objective::Exec, 1),
        Some(spec(Preset::Opt))
    );
}

#[test]
fn spec_text_round_trips() {
    let custom = CompileOptions::from_spec(
        "opt-level=3,mir-opt-level=1,inline=yes,inline-threshold=250,strip=symbols,codegen-units=4,debug=no",
    )
    .unwrap();
    let text = custom.to_spec();
    assert_eq!(CompileOptions::from_spec(&text).unwrap(), custom);
}
