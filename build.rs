#![deny(warnings)]

extern crate cc;

fn main() {
    cc::Build::new()
        .file("picohttpparser/picohttpparser.c")
        .compile("libpico.a");
}

