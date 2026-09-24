fn main() {
    println!("cargo:rerun-if-changed=src/boot.asm");
    println!("cargo:rerun-if-changed=linker.ld");

    nasm_rs::Build::new()
        .file("src/boot.asm")
        .target("x86_64-unknown-none")
        .compile("boot")
        .expect("Failed to assemble boot.asm");

    println!("cargo:rustc-link-lib=static=boot");
    println!("cargo:rustc-link-arg=-Tlinker.ld");
}