use ethos_zero::{Actualizing, File, Generating, Potential};
use message_library::LIBRARY_ETHOS;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=ethos/signal.ethos");
    println!("cargo:rerun-if-changed=src/generated/signal.rs");
    let root = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest"));
    let source = std::fs::read_to_string(root.join("ethos/signal.ethos")).expect("source");
    let file = Potential::<File>::from(source.clone())
        .actualize()
        .unwrap_or_else(|_| panic!("read Signal"));
    let generated = file
        .generate()
        .unwrap_or_else(|_| panic!("generate Signal"));
    assert_eq!(
        generated,
        std::fs::read_to_string(root.join("src/generated/signal.rs")).expect("generated")
    );
    let contract = format!("{LIBRARY_ETHOS}\n{source}");
    let rust = format!("pub const ETHOS: &str = {contract:?};\n");
    std::fs::write(
        PathBuf::from(std::env::var_os("OUT_DIR").expect("output directory"))
            .join("contract_source.rs"),
        rust,
    )
    .expect("write contract source");
}
