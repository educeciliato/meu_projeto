
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
    let mut banco = Banco::novo("🏦 Banco Rust".to_string());
    
    // Criar contas de teste
    banco.criar_conta(1001, "João Silva".to_string());
    banco.criar_conta(1002, "Maria Santos".to_string());
    banco.criar_conta(1003, "Pedro Costa".to_string());
    
    let mut rodando = true;
    
    while rodando {
        println!("\n╔════════════════════════════════════╗");
        println!("║  {}                    ║", banco.nome);
        println!("╚════════════════════════════════════╝");
        println!("1. Criar conta");
        println!("2. Depositar");
        println!("3. Sacar");
        println!("4. Ver saldo");
        println!("5. Listar contas");
        println!("6. Ver histórico");
        println!("7. Sair");
        print!("\nEscolha uma opção: ");
        io::stdout().flush().unwrap();
        
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).unwrap();
        let opcao = entrada.trim().parse::<u32>().unwrap_or(0);
        
        match opcao {
            1 => {
                print!("Número da conta: ");
                io::stdout().flush().unwrap();
                let mut n = String::new();
                io::stdin().read_line(&mut n).unwrap();
                let numero = n.trim().parse().unwrap_or(0);
                
                if numero == 0 {
                    println!("❌ Número inválido!");
                    continue;
                }
                
                if banco.encontrar_conta(numero).is_some() {
                    println!("❌ Conta já existe!");
                    continue;
                }
                
                print!("Nome do titular: ");
                io::stdout().flush().unwrap();
                let mut nome = String::new();
                io::stdin().read_line(&mut nome).unwrap();
                let nome = nome.trim().to_string();
                
                if nome.is_empty() {
                    println!("❌ Nome não pode estar vazio!");
                    continue;
                }
                
                banco.criar_conta(numero, nome);
            },
            
            2 => {
                print!("Número da conta: ");
                io::stdout().flush().unwrap();
                let mut n = String::new();
                io::stdin().read_line(&mut n).unwrap();
                let numero = n.trim().parse().unwrap_or(0);
                
                print!("Valor a depositar: R$ ");
                io::stdout().flush().unwrap();
                let mut v = String::new();
                io::stdin().read_line(&mut v).unwrap();
                let valor = v.trim().parse().unwrap_or(0.0);
                
                match banco.depositar(numero, valor) {
                    Ok(()) => {
                        println!("✅ Depósito bem-sucedido!");
                    },
                    Err(e) => {
                        println!("❌ Erro: {:?}", e);
                    },
                }
            },
            
            3 => {
                print!("Número da conta: ");
                io::stdout().flush().unwrap();
                let mut n = String::new();
                io::stdin().read_line(&mut n).unwrap();
                let numero = n.trim().parse().unwrap_or(0);
                
                print!("Valor a sacar: R$ ");
                io::stdout().flush().unwrap();
                let mut v = String::new();
                io::stdin().read_line(&mut v).unwrap();
                let valor = v.trim().parse().unwrap_or(0.0);
                
                match banco.sacar(numero, valor) {
                    Ok(()) => {
                        println!("✅ Saque bem-sucedido!");
                    },
                    Err(e) => {
                        println!("❌ Erro: {:?}", e);
                    },
                }
            },
            
            4 => {
                print!("Número da conta: ");
                io::stdout().flush().unwrap();
                let mut n = String::new();
                io::stdin().read_line(&mut n).unwrap();
                let numero = n.trim().parse().unwrap_or(0);
                
                match banco.saldo(numero) {
                    Some(s) => {
                        println!("\n💰 Saldo: R$ {:.2}", s);
                    },
                    None => {
                        println!("❌ Conta não encontrada");
                    },
                }
            },
            
            5 => {
                banco.listar_contas();
            },
            
            6 => {
                banco.ver_historico();
            },
            
            7 => {
                println!("\n👋 Obrigado por usar o Banco Rust!");
                println!("Até logo!\n");
                rodando = false;
            },
            
            _ => {
                println!("❌ Opção inválida! Escolha de 1 a 7");
            },
        }
    }
}

