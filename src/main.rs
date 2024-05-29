mod pcp;
use crate::pcp::MateriaPrima;
use crate::pcp::Pedido;
use crate::pcp::Produto;

fn main() {
    println!("Sistema de Planejamento e Controle de Produção (PCP)");
    
    let materias_primas = std::fs::read_to_string("materias_primas.json").expect("Erro ao ler arquivo");
    let materias_primas: Vec<MateriaPrima> = serde_json::from_str(&materias_primas).expect("Erro ao desserializar");

    let produtos = std::fs::read_to_string("produtos.json").expect("Erro ao ler arquivo");
    let produtos: Vec<Produto> = serde_json::from_str(&produtos).expect("Erro ao desserializar");

    let pedidos = std::fs::read_to_string("pedidos.json").expect("Erro ao ler arquivo");
    let pedidos: Vec<Pedido> = serde_json::from_str(&pedidos).expect("Erro ao desserializar");

    
}