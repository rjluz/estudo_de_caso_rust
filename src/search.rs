use crate::index::{Produto, Indexador};
use crate::preprocesso::normalizar;


pub fn buscar<'a>(
    indexador: &'a Indexador,
    nome: Option<&str>,
    marca: Option<&str>,
    categoria: Option<&str>,
) -> Vec<&'a Produto> {
    let mut resultados: Vec<&Produto> = if let Some(nome) = nome {
        let nome_norm = normalizar(nome);
        indexador.buscar_por_nome(&nome_norm)
    } else {
        indexador.todos_os_produtos()
    };

    if let Some(marca) = marca {
        let marca_norm = normalizar(marca);
        resultados = resultados
            .into_iter()
            .filter(|p| normalizar(&p.marca) == marca_norm)
            .collect();
    }

    if let Some(categoria) = categoria {
        let categoria_norm = normalizar(categoria);
        resultados = resultados
            .into_iter()
            .filter(|p| normalizar(&p.categoria) == categoria_norm)
            .collect();
    }

    resultados
}
