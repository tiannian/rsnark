use std::fs;

use rust2go::{GoCompiler, RegenArgs};

pub struct NoBuildCompiler;

impl GoCompiler for NoBuildCompiler {
    fn go_build(
        &self,
        _go_src: &std::path::Path,
        _link: rust2go::LinkType,
        output: &std::path::Path,
    ) {
        let h_file = include_str!("./assets/provers-gnark.h");

        fs::create_dir_all(output).unwrap();

        fs::write(output.parent().unwrap().join("libgo.h"), h_file).unwrap();
    }
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");

    if std::env::var("DOCS_RS").is_ok() {
        rust2go::Builder::new()
            .with_go_src("./go")
            .with_go_compiler(NoBuildCompiler)
            .build();
    } else {
        rust2go::Builder::new()
            .with_go_src("./go")
            .with_regen_arg(RegenArgs {
                src: "./src/ffi.rs".into(),
                dst: "./go/gen.go".into(),
                go118: true,
                ..Default::default()
            })
            .build();
    }
}
