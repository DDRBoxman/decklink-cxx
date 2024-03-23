fn main() {
    cxx_build::bridge("src/main.rs")
        .file("./include/callback.cc")
        .file("./decklink/Mac/include/DeckLinkAPIDispatch.cpp")
        .flag_if_supported("-std=c++14")
        .compile("cxx-demo");

    println!("cargo:rustc-link-lib=framework=CoreFoundation");

    println!("cargo:rerun-if-changed=src/main.rs");
}
