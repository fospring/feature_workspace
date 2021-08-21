fn main() {
    cc::Build::new()
        .file("src/c/foo.c")
        .file("src/c/bar.c")
        .compile("c");
}