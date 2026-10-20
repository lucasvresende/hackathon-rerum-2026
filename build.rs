fn main() {
    println!("cargo:rerun-if-changed=migrations");
    println!("cargo:rerun-if-changed=dados_referencia");
    println!("cargo:rerun-if-changed=REQUISITOS.md");
    println!("cargo:rerun-if-changed=hackathon/TEMA_2_DIETAS_LITERATURA_E_MOTOR.md");
}
