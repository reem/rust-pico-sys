#![deny(warnings)]

extern crate gcc;

fn main() {
    gcc::compile_library(
        "libpico.a",
        &["extern/picohttpparser/picohttpparser.c"]);
}

