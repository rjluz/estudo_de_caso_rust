use std::collections::HashMap;
use crate::preprocesso::normalizar;


#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub marca: String,
    pub categoria: String,
}

pub struct Indexador {
    nome_index: HashMap<String, Vec<u32>>,
    produtos: HashMap<u32, Produto>,
}

impl Indexador {
    pub fn new() -> Self {
        Self {
            nome_index: HashMap::new(),
            produtos: HashMap::new(),
        }
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        let nome_normalizado = normalizar(&produto.nome);
        self.nome_index
            .entry(nome_normalizado)
            .or_default()
            .push(produto.id);
        self.produtos.insert(produto.id, produto);
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Vec<&Produto> {
        let nome_normalizado = normalizar(nome);
        self.nome_index
            .get(&nome_normalizado)
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|id| self.produtos.get(id))
            .collect()
    }

    pub fn todos_os_produtos(&self) -> Vec<&Produto> {
        self.produtos.values().collect()
    }
}
