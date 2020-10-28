fn main() {
    cc::Build::new()
        .file("c_src/vecpow.c")
        .flag("-Ofast")
        .flag("-march=native")
        .flag("-funroll-all-loops")
        .compile("libvecpow.a");
}
