#[cfg(feature = "with-syntex")]
mod inner {
    extern crate rustler_codegen;

    use std::env;
    use std::path::Path;

    pub fn main() {
        let out_dir = env::var_os("OUT_DIR").unwrap();

        let src = Path::new("src/lib.in.rs");
        let dst = Path::new(&out_dir).join("lib.rs");

        rustler_codegen::expand(&src, &dst).unwrap();
    }
}

#[cfg(not(feature = "with-syntex"))]
mod inner {
    pub fn main() {}
}

fn main() {
    inner::main();
}
