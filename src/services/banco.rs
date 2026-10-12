use crate::models::{Auditavel, Banco, Conta, ErroOperacao, Operacao};

impl Banco {
    pub fn novo(nome: String) -> Banco {
        Banco {
            nome,
            contas: Vec::new(),
            historico: Vec::new(),
        }
    }

    // Encontrar conta por número (retorna Option)
    pub fn encontrar_conta(&self, numero: u32) -> Option<usize> {
        self.contas.iter().position(|c| c.numero == numero)
    }

    // Criar conta
    pub fn criar_conta(&mut self, numero: u32, titular: String) {
        self.contas.push(Conta::nova(numero, titular.clone()));
        println!("✅ Conta {} criada para {}", numero, titular);
    }

    // Depositar (retorna Result)
    pub fn depositar(&mut self, numero: u32, valor: f64) -> Result<(), ErroOperacao> {
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
    pub fn sacar(&mut self, numero: u32, valor: f64) -> Result<(), ErroOperacao> {
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
    pub fn saldo(&self, numero: u32) -> Option<f64> {
        self.encontrar_conta(numero).map(|i| self.contas[i].ver_saldo())
    }

    // Ver histórico
    pub fn ver_historico(&self) {
        if self.historico.is_empty() {
            println!("📋 Histórico vazio");
            return;
        }

        println!("\n📋 --- Histórico de Operações ---");
        for (i, op) in self.historico.iter().enumerate() {
            println!("{}. {} - {} - R$ {:.2}", i + 1, op.numero_conta, op.tipo, op.valor);
        }
    }

    // Listar contas
    pub fn listar_contas(&self) {
        if self.contas.is_empty() {
            println!("❌ Nenhuma conta cadastrada");
            return;
        }

        println!("\n📊 --- Contas Cadastradas ---");
        for conta in &self.contas {
            let status = if conta.ativa { "✅ Ativa" } else { "❌ Inativa" };
            println!("Conta {}: {} - R$ {:.2} ({})", conta.numero, conta.titular, conta.saldo, status);
        }
    }
}