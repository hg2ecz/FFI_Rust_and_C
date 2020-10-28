fn main() {
    cc::Build::new()
        .file("src/vecpow.c")
        .flag("-Ofast")
        .flag("-march=native")
        .flag("-funroll-all-loops")
        .compile("libvecpow.a");
}
