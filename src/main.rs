#[path = "models/models.rs"]
mod models;

#[path = "services/auditavel.rs"]
mod auditavel;

#[path = "services/conta.rs"]
mod conta;

#[path = "services/banco.rs"]
mod banco;

use crate::models::Banco;
use std::io::{self, Write};

// ============ MAIN: INTERFACE INTERATIVA ============
fn main() {
    let conta = models::Conta::nova(1, String::from("Maria Silva"));
    println!("Conta criada: {:?}", conta);
}
