use crate::models::{Conta, ErroOperacao};

impl Conta {
    // Construtor
    pub fn nova(numero: u32, titular: String) -> Conta {
        Conta {
            numero,
            titular,
            saldo: 0.0,
            ativa: true,
        }
    }

    // Método que LÊ (&self)
    pub fn ver_saldo(&self) -> f64 {
        self.saldo
    }

    // Método que MODIFICA (&mut self) - Retorna Result
    pub fn depositar(&mut self, valor: f64) -> Result<(), ErroOperacao> {
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
    pub fn sacar(&mut self, valor: f64) -> Result<(), ErroOperacao> {
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

    // Método que CONSOME (self)
    pub fn fechar(self) {
        println!("❌ Conta {} ({}) fechada", self.numero, self.titular);
    }
}