use std::{fs, path::PathBuf};

use claims::{assert_err, assert_ok};
use semver::Version;
use wash_lib::parser::{
    get_config, ActorConfig, CommonConfig, LanguageConfig, RustConfig, RustWasmTarget,
    TinyGoConfig, TinyGoWasmTarget, TypeConfig,
};

#[test]
/// When given a specific toml file's path, it should parse the file and return a ProjectConfig.
fn rust_actor() {
    let result = get_config(
        Some(PathBuf::from("./tests/parser/files/rust_actor.toml")),
        None,
    );

    let config = assert_ok!(result);

    assert_eq!(
        config.language,
        LanguageConfig::Rust(RustConfig {
            cargo_path: Some("./cargo".into()),
            target_path: Some("./target".into())
        })
    );

    assert_eq!(
        config.project_type,
        TypeConfig::Actor(ActorConfig {
            claims: vec!["wasmcloud:httpserver".to_string()],
            registry: Some("localhost:8080".to_string()),
            push_insecure: false,
            key_directory: PathBuf::from("./keys"),
            filename: Some("testactor.wasm".to_string()),
            wasm_target: RustWasmTarget::CoreModule,
            wasi_preview2_adapter_path: None,
            call_alias: Some("testactor".to_string())
        })
    );

    assert_eq!(
        config.common,
        CommonConfig {
            name: "testactor".to_string(),
            version: Version::parse("0.1.0").unwrap(),
            path: PathBuf::from("./tests/parser/files/")
                .canonicalize()
                .unwrap(),
            wasm_bin_name: None,
        }
    );
}

#[test]
fn tinygo_actor() {
    let result = get_config(
        Some(PathBuf::from("./tests/parser/files/tinygo_actor.toml")),
        None,
    );

    let config = assert_ok!(result);

    assert_eq!(
        config.language,
        LanguageConfig::TinyGo(TinyGoConfig {
            tinygo_path: Some("path/to/tinygo".into()),
            wasm_target: Some(TinyGoWasmTarget::CoreModule.into()),
        })
    );

    assert_eq!(
        config.project_type,
        TypeConfig::Actor(ActorConfig {
            claims: vec!["wasmcloud:httpserver".to_string()],
            registry: Some("localhost:8080".to_string()),
            push_insecure: false,
            key_directory: PathBuf::from("./keys"),
            filename: Some("testactor.wasm".to_string()),
            call_alias: Some("testactor".to_string()),
            wasi_preview2_adapter_path: None,
            wasm_target: WasmTarget::CoreModule,
        })
    );

    assert_eq!(
        config.common,
        CommonConfig {
            name: "testactor".to_string(),
            version: Version::parse("0.1.0").unwrap(),
            path: PathBuf::from("./tests/parser/files/")
                .canonicalize()
                .unwrap(),
            wasm_bin_name: None,
        }
    );
}

#[test]
/// When given a folder, should automatically grab a wasmcloud.toml file inside it and parse it.
fn folder_path() {
    let result = get_config(Some(PathBuf::from("./tests/parser/files/folder")), None);

    let config = assert_ok!(result);

    assert_eq!(
        config.language,
        LanguageConfig::Rust(RustConfig {
            cargo_path: Some("./cargo".into()),
            target_path: Some("./target".into())
        })
    );
}

/// Gets the full path of a local path. Test helper.
fn get_full_path(path: &str) -> String {
    match fs::canonicalize(path) {
        Ok(path) => path.to_str().unwrap().to_string(),
        Err(_) => panic!("get_full_path helper error. Could not find path: {path}"),
    }
}

#[test]
fn no_actor_config() {
    let result = get_config(
        Some(PathBuf::from("./tests/parser/files/no_actor.toml")),
        None,
    );

    let err = assert_err!(result);

    assert_eq!(
        format!(
            "Missing actor config in {}",
            get_full_path("./tests/parser/files/no_actor.toml")
        ),
        err.to_string().as_str()
    );
}

#[test]
fn no_provider_config() {
    let result = get_config(
        Some(PathBuf::from("./tests/parser/files/no_provider.toml")),
        None,
    );

    let err = assert_err!(result);

    assert_eq!(
        format!(
            "Missing provider config in {}",
            get_full_path("./tests/parser/files/no_provider.toml")
        ),
        err.to_string().as_str()
    );
}

#[test]
fn no_interface_config() {
    let result = get_config(
        Some(PathBuf::from("./tests/parser/files/no_interface.toml")),
        None,
    );

    let err = assert_err!(result);

    assert_eq!(
        format!(
            "Missing interface config in {}",
            get_full_path("./tests/parser/files/no_interface.toml")
        ),
        err.to_string().as_str()
    );
}

#[test]
/// When given a folder with no wasmcloud.toml file, should return an error.
fn folder_path_with_no_config() {
    let result = get_config(Some(PathBuf::from("./tests/parser/files/noconfig")), None);

    let err = assert_err!(result);
    assert_eq!(
        format!(
            "No wasmcloud.toml file found in {}",
            get_full_path("./tests/parser/files/noconfig")
        ),
        err.to_string().as_str()
    );
}

#[test]
/// When given a random file, should return an error.
fn random_file() {
    let result = get_config(Some(PathBuf::from("./tests/parser/files/random.txt")), None);

    let err = assert_err!(result);
    assert_eq!(
        format!(
            "Invalid config file: {}",
            get_full_path("./tests/parser/files/random.txt")
        ),
        err.to_string().as_str()
    );
}

#[test]
/// When given a nonexistent file or path, should return an error.
fn nonexistent_file() {
    let result = get_config(
        Some(PathBuf::from("./tests/parser/files/nonexistent.toml")),
        None,
    );

    let err = assert_err!(result);
    assert_eq!(
        "Path ./tests/parser/files/nonexistent.toml does not exist",
        err.to_string().as_str()
    );
}

#[test]
fn nonexistent_folder() {
    let result = get_config(
        Some(PathBuf::from("./tests/parser/files/nonexistent/")),
        None,
    );

    let err = assert_err!(result);
    assert_eq!(
        "Path ./tests/parser/files/nonexistent/ does not exist",
        err.to_string().as_str()
    );
}

#[test]
fn minimal_rust_actor() {
    let result = get_config(
        Some(PathBuf::from(
            "./tests/parser/files/minimal_rust_actor.toml",
        )),
        None,
    );

    let config = assert_ok!(result);

    assert_eq!(
        config.language,
        LanguageConfig::Rust(RustConfig {
            cargo_path: None,
            target_path: None,
        })
    );

    assert_eq!(
        config.project_type,
        TypeConfig::Actor(ActorConfig {
            claims: vec!["wasmcloud:httpserver".to_string()],
            registry: None,
            push_insecure: false,
            key_directory: PathBuf::from("./keys"),
            filename: None,
            call_alias: None,
            wasi_preview2_adapter_path: None,
            wasm_target: WasmTarget::CoreModule,
        })
    );

    assert_eq!(
        config.common,
        CommonConfig {
            name: "testactor".to_string(),
            version: Version::parse("0.1.0").unwrap(),
            path: PathBuf::from("./tests/parser/files/")
                .canonicalize()
                .unwrap(),
            wasm_bin_name: None,
        }
    )
}

#[test]
fn cargo_toml_actor() {
    let result = get_config(
        Some(PathBuf::from(
            "./tests/parser/files/withcargotoml/minimal_rust_actor_with_cargo.toml",
        )),
        None,
    );

    let config = assert_ok!(result);

    assert_eq!(
        config.language,
        LanguageConfig::Rust(RustConfig {
            cargo_path: None,
            target_path: None,
        })
    );

    assert_eq!(
        config.project_type,
        TypeConfig::Actor(ActorConfig {
            claims: vec!["wasmcloud:httpserver".to_string()],
            registry: None,
            push_insecure: false,
            key_directory: PathBuf::from("./keys"),
            filename: None,
            call_alias: None,
            wasi_preview2_adapter_path: None,
            wasm_target: WasmTarget::CoreModule,
        })
    );

    assert_eq!(
        config.common,
        CommonConfig {
            name: "withcargotoml".to_string(),
            version: Version::parse("0.200.0").unwrap(),
            path: PathBuf::from("./tests/parser/files/withcargotoml")
                .canonicalize()
                .unwrap(),
            wasm_bin_name: None,
        }
    )
}

/// wasm_target=wasm32-wasi-preview2 is properly parsed
/// see: https://github.com/wasmCloud/wash/issues/640
#[test]
fn minimal_rust_actor_preview2() {
    let result = get_config(
        Some(PathBuf::from(
            "./tests/parser/files/minimal_rust_actor_preview2.toml",
        )),
        None,
    );

    let config = assert_ok!(result);
    assert!(matches!(
        config.project_type,
        TypeConfig::Actor(ActorConfig {
            wasm_target: RustWasmTarget::WasiPreview2,
            ..
        })
    ));
}

/// wasm_target=wasm32-wasi-preview1 is properly parsed
/// see: https://github.com/wasmCloud/wash/issues/640
#[test]
fn minimal_rust_actor_preview1() {
    let result = get_config(
        Some(PathBuf::from(
            "./tests/parser/files/minimal_rust_actor_preview1.toml",
        )),
        None,
    );

    let config = assert_ok!(result);
    assert!(matches!(
        config.project_type,
        TypeConfig::Actor(ActorConfig {
            wasm_target: RustWasmTarget::WasiPreview1,
            ..
        })
    ));
}

/// wasm_target=wasm32-unknown-unknown is properly parsed
/// see: https://github.com/wasmCloud/wash/issues/640
#[test]
fn minimal_rust_actor_core_module() {
    let result = get_config(
        Some(PathBuf::from(
            "./tests/parser/files/minimal_rust_actor_core_module.toml",
        )),
        None,
    );

    let config = assert_ok!(result);
    assert!(matches!(
        config.project_type,
        TypeConfig::Actor(ActorConfig {
            wasm_target: RustWasmTarget::CoreModule,
            ..
        })
    ));
}
