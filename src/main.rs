use rusqlite::{Connection, Result};

fn cria_banco() -> Result<Connection> {
        let banco = Connection::open("info_banco.db")?;

        banco.execute(
            "CREATE TABLE IF NOT EXISTS contas (
                id INTEGER PRIMARY KEY,
                nome VARCHAR(50) NOT NULL,
                saldo REAL NOT NULL,
                nome_sujo BOOLEAN NOT NULL,
                data_nascimento DATE NOT NULL,
                cpf VARCHAR(15) NOT NULL UNIQUE,
                tem_divida BOOLEAN NOT NULL
            )",
            [],
        )?;

    Ok(banco)

} 

fn main() {
    let meu_banco_de_dados = cria_banco().expect("Falha ao criar banco de dados");

    meu_banco_de_dados.execute( 
        "INSERT INTO contas (nome, saldo, nome_sujo, data_nascimento, cpf, tem_divida) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        &["Henrique Batata", "12.28", "false", "1821-15-12", "123.456.789-00", "false"],
    ).expect("Falha ao inserir dados"); 
    

    println!("AMEM!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!1");


}
