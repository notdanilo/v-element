mod cpf;
mod rg;
mod cnpj;

pub use cpf::*;
pub use rg::*;
pub use cnpj::*;

pub enum BrazilIdentity {
    CadastroPessoaFisica(CadastroPessoaFisica),
    RegistroGeral(RegistroGeral),
    CNPJ(CadastroNacionalPessoaJuridica)
}
