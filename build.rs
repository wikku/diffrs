fn main() {
    cc::Build::new()
        .file("libsais/libsais.c")
        .compile("sais")
}
