use rust2go::RegenArgs;

fn main() {
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
