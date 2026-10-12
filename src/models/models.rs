#[derive(Debug)]
pub enum ErroOperacao {
    ContaInativa,
    SaldoInsuficiente,
    ValorInvalido,
    ContaNaoEncontrada,
}

pub trait Auditavel {
    fn registrar(&self);
}

#[derive(Debug)]
pub struct Operacao {
    pub numero_conta: u32,
    pub tipo: String,
    pub valor: f64,
}

#[derive(Debug)]
pub struct Conta {
    pub numero: u32,
    pub titular: String,
    pub saldo: f64,
    pub ativa: bool,
}

#[derive(Debug)]
pub struct Banco {
    pub nome: String,
    pub contas: Vec<Conta>,
    pub historico: Vec<Operacao>,
}