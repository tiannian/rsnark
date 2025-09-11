use rust2go::{GoCompiler, RegenArgs};

pub struct NoBuildCompiler;

impl GoCompiler for NoBuildCompiler {
    fn go_build(
        &self,
        _go_src: &std::path::Path,
        _link: rust2go::LinkType,
        _output: &std::path::Path,
    ) {
    }
}

fn main() {
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
