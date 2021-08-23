fn main() {
    cc::Build::new()
        .file("src/c/foo.c")
        .file("src/c/bar.c")
        .file("src/c/time.cpp")
        .cpp(true)
        .compile("c.a");
}