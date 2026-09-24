fn main() {
    println!("cargo:rerun-if-changed=src/boot.asm");
    println!("cargo:rerun-if-changed=linker.ld");


    nasm_rs::Build::new()
        .file("src/boot.asm")
        .target("x86_64-unknown-none")
        .compile("boot");

    println!("cargo:rustc-link-arg=-Tlinker.ld");
}