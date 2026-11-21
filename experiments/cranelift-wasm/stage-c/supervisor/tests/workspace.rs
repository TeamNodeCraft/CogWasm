//! Workspace scheduling: dependency order, per-crate caching, invalidation.

use std::fs;
use std::path::PathBuf;

use cogwasm_supervisor::{
    ArtifactStore, CrateDef, CrateKind, FakeCompiler, Preset, Supervisor, Workspace,
};
use tempfile::TempDir;

fn setup() -> (TempDir, Workspace) {
    let dir = TempDir::new().expect("temp dir");
    let lib_source = dir.path().join("math_lib.rs");
    let app_source = dir.path().join("app.rs");
    fs::write(&lib_source, "pub fn double(x: u64) -> u64 { x * 2 }\n").expect("write lib");
    fs::write(
        &app_source,
        "extern crate math_lib;\n#[unsafe(no_mangle)] pub extern \"C\" fn run(x: u64) -> u64 { math_lib::double(x) }\n",
    )
    .expect("write app");
    let workspace = Workspace {
        crates: vec![
            CrateDef {
                name: "math_lib".to_owned(),
                source: lib_source,
                deps: Vec::new(),
            },
            CrateDef {
                name: "app".to_owned(),
                source: app_source,
                deps: vec!["math_lib".to_owned()],
            },
        ],
    };
    (dir, workspace)
}

fn supervisor(dir: &TempDir) -> Supervisor<FakeCompiler> {
    Supervisor::new(
        ArtifactStore::new(dir.path().join("store")),
        FakeCompiler::new(),
    )
}

#[test]
fn builds_in_dependency_order_and_then_hits() {
    let (dir, workspace) = setup();
    let supervisor = supervisor(&dir);
    let first = workspace
        .build(&supervisor, &Preset::Opt.options(), dir.path())
        .unwrap();
    assert_eq!(first[0].name, "math_lib");
    assert_eq!(first[0].kind, CrateKind::Rlib);
    assert_eq!(first[1].name, "app");
    assert_eq!(first[1].kind, CrateKind::Cdylib);
    assert!(first.iter().all(|build| !build.hit));

    let second = workspace
        .build(&supervisor, &Preset::Opt.options(), dir.path())
        .unwrap();
    assert!(second.iter().all(|build| build.hit), "nothing changed");
}

#[test]
fn editing_a_dependency_rebuilds_it_and_its_dependent() {
    let (dir, workspace) = setup();
    let supervisor = supervisor(&dir);
    let first = workspace
        .build(&supervisor, &Preset::Opt.options(), dir.path())
        .unwrap();

    fs::write(
        &workspace.crates[0].source,
        "pub fn double(x: u64) -> u64 { x * 3 }\n",
    )
    .unwrap();
    let second = workspace
        .build(&supervisor, &Preset::Opt.options(), dir.path())
        .unwrap();

    assert!(!second[0].hit, "the library changed");
    assert!(!second[1].hit, "the app's extern changed");
    assert_ne!(first[1].key, second[1].key, "the app key follows the rlib");
}

#[test]
fn editing_the_app_rebuilds_only_the_app() {
    let (dir, workspace) = setup();
    let supervisor = supervisor(&dir);
    workspace
        .build(&supervisor, &Preset::Opt.options(), dir.path())
        .unwrap();

    fs::write(
        &workspace.crates[1].source,
        "extern crate math_lib;\n#[unsafe(no_mangle)] pub extern \"C\" fn run(x: u64) -> u64 { math_lib::double(x) + 1 }\n",
    )
    .unwrap();
    let second = workspace
        .build(&supervisor, &Preset::Opt.options(), dir.path())
        .unwrap();

    assert!(second[0].hit, "the library did not change");
    assert!(!second[1].hit, "the app changed");
}

#[test]
fn a_cycle_is_rejected() {
    let workspace = Workspace {
        crates: vec![
            CrateDef {
                name: "a".to_owned(),
                source: PathBuf::from("/tmp/a.rs"),
                deps: vec!["b".to_owned()],
            },
            CrateDef {
                name: "b".to_owned(),
                source: PathBuf::from("/tmp/b.rs"),
                deps: vec!["a".to_owned()],
            },
        ],
    };
    let error = workspace.validate().unwrap_err().to_string();
    assert!(error.contains("cycle"), "unexpected error: {error}");
}

#[test]
fn an_unknown_dependency_is_rejected() {
    let workspace = Workspace {
        crates: vec![CrateDef {
            name: "app".to_owned(),
            source: PathBuf::from("/tmp/app.rs"),
            deps: vec!["missing".to_owned()],
        }],
    };
    let error = workspace.validate().unwrap_err().to_string();
    assert!(error.contains("unknown"), "unexpected error: {error}");
}
