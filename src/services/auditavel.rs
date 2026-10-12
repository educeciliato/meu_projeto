use crate::models::{Auditavel, Operacao};

impl Auditavel for Operacao {
    fn registrar(&self) {
        println!(
            "📋 Conta {} - {} - R$ {:.2}",
            self.numero_conta, self.tipo, self.valor
        );
    }
}
