fn main() {
    println!("cargo:rerun-if-changed=migrations");
    println!("cargo:rerun-if-changed=dados_referencia");
    println!("cargo:rerun-if-changed=docs/REQUISITOS.md");
    println!("cargo:rerun-if-changed=docs/MOTOR.md");
    println!("cargo:rerun-if-changed=docs/FONTES.md");
}
