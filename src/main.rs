//Trabalho Individual

mod pcp;
use std::collections::HashMap;
use crate::pcp::{MateriaPrima,Produto,Pedido,calcular_necessidades,calcular_quantidade,calcular_custo_acumulado,gerar_pedidos_compra,calcular_datas_pedido};

fn main() {
    println!();
    println!("Sistema de Planejamento e Controle de Produção (PCP)\n");
    println!();
    
    let materias_primas = std::fs::read_to_string("materias_primas.json").expect("Erro ao ler arquivo");
    let materias_primas: Vec<MateriaPrima> = serde_json::from_str(&materias_primas).expect("Erro ao desserializar");
    //como cada leitor de arquivo retornaria um vetor de um tipo diferente eu achei melhor fazer sem função mesmo
    
    let produtos = std::fs::read_to_string("produtos.json").expect("Erro ao ler arquivo");
    let _produtos: Vec<Produto> = serde_json::from_str(&produtos).expect("Erro ao desserializar");
    // eu tive que mudar o arquivo de carga dos produtos, porque não estava lendo as matérias primas como do tipo MateriaPrima, e sim como strings (a mesma coisa com o arquivo dos pedidos)
    
    let pedidos = std::fs::read_to_string("pedidos.json").expect("Erro ao ler arquivo");
    let pedidos: Vec<Pedido> = serde_json::from_str(&pedidos).expect("Erro ao desserializar");
    
    let minimos: HashMap<String, u32> = vec![
        (String::from("MP1"), 10),
        (String::from("MP2"), 5),
        (String::from("MP3"), 5),
        (String::from("MP4"), 15),
        (String::from("MP5"), 10),
        (String::from("MP6"), 10),
        (String::from("MP7"), 15),
        (String::from("MP8"), 5),
        (String::from("MP9"), 10),
        (String::from("MP10"), 10)
    ].into_iter().collect();//pedidos minimos de matérias primas

    let tempo_entrega = 3;
    //tempo que leva para as matérias primas chegarem a tempo / transporte (as matérias-primas serão pedidas X dias antes da data de entrega do produto)
    
    let necessidades = calcular_necessidades(&pedidos);
    let pedidos_compra = gerar_pedidos_compra(necessidades, &minimos);
    let datas_pedido = calcular_datas_pedido(pedidos_compra, &pedidos, tempo_entrega, &materias_primas);
    let quantidade_produtos = calcular_quantidade(&pedidos);
    
    println!("Quantidade de cada produto:");
    println!();
    calcular_quantidade(&pedidos);
    for (produto, quantidade) in &quantidade_produtos {
        println!("Produto: {}, Quantidade: {}", produto, quantidade);
        for pedido in &pedidos {
            if quantidade == &(pedido.produto.capacidade_producao+1) { //para que só seja impresso uma vez
                println!("O {} excede a capacidade máxima de produção de {} unidades", pedido.produto.nome, pedido.produto.capacidade_producao);
                break;
            }
        }
    }
    println!();
    println!();
    println!("Pedidos de compra:");
    let custo_total = calcular_custo_acumulado(&datas_pedido);
    println!("Custo total de todas matérias-primas: R${},00", custo_total);
    println!();
    for datas_pedido in datas_pedido.iter(){
        println!("Materia Prima: {}",datas_pedido.materia_prima);
        println!("Data de Pedido: {}",datas_pedido.data_pedido);
        println!("Quantidade: {}",datas_pedido.quantidade);
        println!("Limite Armazenamento: {}",datas_pedido.limite_armazenamento);
        println!("Custo por unidade: R${},00",datas_pedido.custo_unidade);
        println!("Custo Total: R${},00",datas_pedido.custo_total);
        println!();
    }
}