use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MateriaPrima {
    pub nome: String,
}
#[derive(Deserialize, Debug)]
pub struct Produto {
    pub nome: String,
    pub materias_primas: Vec<(MateriaPrima, u32)>, // lista de materias primas e suas quantidades
    pub tempo_fabricacao: i32,
    pub capacidade_producao: i32, // adicionado campo de capacidade de produção
}

#[derive(Deserialize, Debug)]
pub struct Pedido {
    pub produto: Produto,
    pub data_entrega: String,
}

impl MateriaPrima {
    pub fn readMateria(file_name: &str) -> Vec<MateriaPrima> {
        let materias_primas = std::fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        let materias_primas: Vec<MateriaPrima> = serde_json::from_str(&materias_primas).expect("Erro ao desserializar");
        materias_primas
    }
}