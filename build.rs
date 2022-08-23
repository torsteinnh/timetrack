use clap::CommandFactory;
use clap_complete::{generate_to, Shell};

include!("src/cli.rs");


fn main() {

    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    let mut app = Cli::into_app();
    let out_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");

    for shell in [Shell::Bash, Shell::PowerShell] {
        generate_to(shell, &mut app, "tt", out_dir.clone()).unwrap();
    }
}