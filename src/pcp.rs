use serde::Deserialize;
use std::collections::HashMap;
use chrono::{NaiveDate, Duration};
use std::cmp;

#[derive(Deserialize, Debug)]
pub struct MateriaPrima {
    pub nome: String,
    pub custo: u32,
    pub limite_armazenamento: u32,
}
#[derive(Deserialize, Debug)]
pub struct Produto {
    pub nome: String,
    pub materias_primas: Vec<(MateriaPrima, u32)>, // lista de materias primas e suas quantidades
    pub tempo_fabricacao: u32,
    pub capacidade_producao: u32, // adicionado campo de capacidade de produção
}
#[derive(Deserialize, Debug)]
pub struct Pedido {
    pub produto: Produto,
    pub data_entrega: String,
}
#[derive(Debug)]
pub struct CompraPedidos{
    pub materia_prima: String,
    pub data_pedido: String,
    pub quantidade: u32,
    pub custo_unidade: u32,
    pub custo_total: u32,
    pub limite_armazenamento: u32,
}

pub fn calcular_necessidades(pedidos: &[Pedido]) -> HashMap<String, u32> {
    let mut necessidades = HashMap::new();
    for pedido in pedidos {
        for (materia_prima, quantidade) in &pedido.produto.materias_primas {
            *necessidades.entry(materia_prima.nome.clone()).or_insert(0) += quantidade;
        }
    }
    necessidades
} //essa função serve principalmente para ordenar melhor as quantidades de cada matéria prima de cada produto em uma única variável (pega os valores de quantidade de cada matéria prima dos produtos e coloca eles na variável necessidades)

pub fn calcular_quantidade(pedidos: &[Pedido]) -> HashMap<String, u32> {
    let mut qnt_produtos = HashMap::new();
    for pedido in pedidos {
        let nome_produto = &pedido.produto.nome;
        let entry = qnt_produtos.entry(nome_produto.clone()).or_insert(0);
        *entry += 1;
    }
    qnt_produtos
}

pub fn calcular_custo_acumulado(datas_pedido: &[CompraPedidos]) -> u32 {
    let mut custo_total_materias_primas = 0;
    for datas_pedido in datas_pedido {
        custo_total_materias_primas += datas_pedido.custo_total;
    }
    custo_total_materias_primas
}

pub fn gerar_pedidos_compra(necessidades: HashMap<String, u32>, minimos: &HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut pedidos = Vec::new();
    for (materia_prima, &quantidade_necessaria) in &necessidades {
        let quantidade_pedido = if let Some(&minimo) = minimos.get(materia_prima) {
            cmp::max(quantidade_necessaria, minimo) //se o valor de quantidade for menor que o mínimo, o pedido será igual ao mínimo
        } else {
            quantidade_necessaria// se não tiver um mínimo para o nome da matéria prima, a quantidade_pedido vai receber o valor da padrão da quantidade_necessaria
        };
        pedidos.push((materia_prima.clone(), quantidade_pedido));
    }
    pedidos
} //essa função vai calcular a quantidade de cada matéria prima que precisa ser comprada levando em consideração a quantidade mínima de cada matéria prima

pub fn calcular_datas_pedido(pedidos_compra: Vec<(String, u32)>, pedidos: &[Pedido], tempo_entrega: i64, materias_primas: &[MateriaPrima],) -> Vec<CompraPedidos> {
    let mut datas_pedido = Vec::new();

    for (materia_prima, quantidade) in pedidos_compra {
        let mp = materias_primas.iter().find(|mp| mp.nome == materia_prima).unwrap();
        let quantidade_real = if quantidade > mp.limite_armazenamento {
            mp.limite_armazenamento
        } else {
            quantidade
        };

        let mut datas_entrega: Vec<NaiveDate> = Vec::new();
        for pedido in pedidos {
            if pedido.produto.materias_primas.iter().any(|(mp, _)| mp.nome == materia_prima) {
                let data_entrega = NaiveDate::parse_from_str(&pedido.data_entrega, "%Y-%m-%d").unwrap();
                let data_uso = data_entrega - Duration::days(pedido.produto.tempo_fabricacao.into());
                datas_entrega.push(data_uso);
            }
        }
        let data_entrega_minima = datas_entrega.iter().min().unwrap();
        let data_pedido = *data_entrega_minima - Duration::days(tempo_entrega);
        let custo_total = mp.custo * quantidade_real;
        let custo_unidade = custo_total / quantidade_real;

        datas_pedido.push(CompraPedidos {
            materia_prima: materia_prima.clone(),
            data_pedido: data_pedido.format("%d-%m-%Y").to_string(),
            quantidade: quantidade_real,
            custo_total,
            custo_unidade,
            limite_armazenamento: mp.limite_armazenamento,
        });
    }
    datas_pedido
}
//essa função serve para calcular a data de entrega, baseando-se no menor tempo possível ()