pub struct CadastroNacionalPessoaJuridica {

}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use pdf::file::File;

    #[test]
    fn from_pdf() {
        let path = Path::new("data/CNPJ DANILO GUANABARA.pdf");
        let file = File::open(path).unwrap();
        println!("{:#?}", file.pages().next().unwrap().unwrap());
    }
}