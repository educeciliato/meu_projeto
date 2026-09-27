use std::io::{self, Write};

// ============ AULA 3: ENUMS PARA ERROS ============
#[derive(Debug)]
enum ErroOperacao {
    ContaInativa,
    SaldoInsuficiente,
    ValorInvalido,
    ContaNaoEncontrada,
}

// ============ AULA 3: TRAITS ============
trait Auditavel {
    fn registrar(&self);
}

// ============ AULA 3: STRUCT OPERACAO ============
#[derive(Debug)]
struct Operacao {
    numero_conta: u32,
    tipo: String,
    valor: f64,
}

impl Auditavel for Operacao {
    fn registrar(&self) {
        println!(
            "📋 Conta {} - {} - R$ {:.2}",
            self.numero_conta, self.tipo, self.valor
        );
    }
}

// ============ AULA 2: STRUCT CONTA ============
#[derive(Debug)]
struct Conta {
    numero: u32,
    titular: String,
    saldo: f64,
    ativa: bool,
}

// ============ AULA 2: IMPLEMENTAR MÉTODOS (impl) ============
impl Conta {
    // Construtor
    fn nova(numero: u32, titular: String) -> Conta {
        Conta {
            numero,
            titular,
            saldo: 0.0,
            ativa: true,
        }
    }

    // Método que LÊ (&self)
    fn ver_saldo(&self) -> f64 {
        self.saldo
    }

    // Método que MODIFICA (&mut self) - Retorna Result
    fn depositar(&mut self, valor: f64) -> Result<(), ErroOperacao> {
        if !self.ativa {
            return Err(ErroOperacao::ContaInativa);
        }
        if valor <= 0.0 {
            return Err(ErroOperacao::ValorInvalido);
        }

        self.saldo += valor;
        Ok(())
    }

    // Método que MODIFICA (&mut self) - Retorna Result
    fn sacar(&mut self, valor: f64) -> Result<(), ErroOperacao> {
        if !self.ativa {
            return Err(ErroOperacao::ContaInativa);
        }
        if valor <= 0.0 {
            return Err(ErroOperacao::ValorInvalido);
        }
        if self.saldo < valor {
            return Err(ErroOperacao::SaldoInsuficiente);
        }

        self.saldo -= valor;
        Ok(())
    }
}

// ============ AULA 3: STRUCT BANCO ============
struct Banco {
    nome: String,
    contas: Vec<Conta>,
    historico: Vec<Operacao>,
}

// ============ AULA 3: IMPLEMENTAR BANCO ============
impl Banco {
    fn novo(nome: String) -> Banco {
        Banco {
            nome,
            contas: Vec::new(),
            historico: Vec::new(),
        }
    }

    // Encontrar conta por número (retorna Option)
    fn encontrar_conta(&self, numero: u32) -> Option<usize> {
        self.contas.iter().position(|c| c.numero == numero)
    }

    // Criar conta
    fn criar_conta(&mut self, numero: u32, titular: String) {
        self.contas.push(Conta::nova(numero, titular.clone()));
        println!("✅ Conta {} criada para {}", numero, titular);
    }

    // Depositar (retorna Result)
    fn depositar(&mut self, numero: u32, valor: f64) -> Result<(), ErroOperacao> {
        match self.encontrar_conta(numero) {
            Some(indice) => {
                self.contas[indice].depositar(valor)?;

                let op = Operacao {
                    numero_conta: numero,
                    tipo: "Depósito".to_string(),
                    valor,
                };
                op.registrar();
                self.historico.push(op);

                Ok(())
            }
            None => Err(ErroOperacao::ContaNaoEncontrada),
        }
    }

    // Sacar (retorna Result)
    fn sacar(&mut self, numero: u32, valor: f64) -> Result<(), ErroOperacao> {
        match self.encontrar_conta(numero) {
            Some(indice) => {
                self.contas[indice].sacar(valor)?;

                let op = Operacao {
                    numero_conta: numero,
                    tipo: "Saque".to_string(),
                    valor,
                };
                op.registrar();
                self.historico.push(op);

                Ok(())
            }
            None => Err(ErroOperacao::ContaNaoEncontrada),
        }
    }

    // Ver saldo (retorna Option)
    fn saldo(&self, numero: u32) -> Option<f64> {
        self.encontrar_conta(numero)
            .map(|i| self.contas[i].ver_saldo())
    }

    // Ver histórico
    fn ver_historico(&self) {
        if self.historico.is_empty() {
            println!("📋 Histórico vazio");
            return;
        }

        println!("\n📋 --- Histórico de Operações ---");
        for (i, op) in self.historico.iter().enumerate() {
            println!(
                "{}. {} - {} - R$ {:.2}",
                i + 1,
                op.numero_conta,
                op.tipo,
                op.valor
            );
        }
    }

    // Listar contas
    fn listar_contas(&self) {
        if self.contas.is_empty() {
            println!("❌ Nenhuma conta cadastrada");
            return;
        }

        println!("\n📊 --- Contas Cadastradas ---");
        for conta in &self.contas {
            let status = if conta.ativa {
                "✅ Ativa"
            } else {
                "❌ Inativa"
            };
            println!(
                "Conta {}: {} - R$ {:.2} ({})",
                conta.numero, conta.titular, conta.saldo, status
            );
        }
    }
}

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
        println!("║  {}                     ║", banco.nome);
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
            }

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
                    }
                    Err(e) => {
                        println!("❌ Erro: {:?}", e);
                    }
                }
            }

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
                    }
                    Err(e) => {
                        println!("❌ Erro: {:?}", e);
                    }
                }
            }

            4 => {
                print!("Número da conta: ");
                io::stdout().flush().unwrap();
                let mut n = String::new();
                io::stdin().read_line(&mut n).unwrap();
                let numero = n.trim().parse().unwrap_or(0);

                match banco.saldo(numero) {
                    Some(s) => {
                        println!("\n💰 Saldo: R$ {:.2}", s);
                    }
                    None => {
                        println!("❌ Conta não encontrada");
                    }
                }
            }

            5 => {
                banco.listar_contas();
            }

            6 => {
                banco.ver_historico();
            }

            7 => {
                println!("\n👋 Obrigado por usar o Banco Rust!");
                println!("Até logo!\n");
                rodando = false;
            }

            _ => {
                println!("❌ Opção inválida! Escolha de 1 a 7");
            }
        }
    }
}
