use projeto::index::{Produto, Indexador};

#[test]
fn test_busca_por_nome() {
    let mut indexador = Indexador::new();

    let produto = Produto {
        id: 1,
        nome: "Notebook Gamer".to_string(),
        marca: "Dell".to_string(),
        categoria: "Eletrônicos".to_string(),
    };

    indexador.adicionar_produto(produto.clone());

    let resultados = indexador.buscar_por_nome("Notebook Gamer");

    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].id, 1);
}

use projeto::search::buscar;

#[test]
fn test_busca_com_filtros() {
    let mut indexador = Indexador::new();

    indexador.adicionar_produto(Produto {
        id: 1,
        nome: "Notebook Gamer".to_string(),
        marca: "Dell".to_string(),
        categoria: "Eletrônicos".to_string(),
    });

    indexador.adicionar_produto(Produto {
        id: 2,
        nome: "Notebook Ultrafino".to_string(),
        marca: "Lenovo".to_string(),
        categoria: "Eletrônicos".to_string(),
    });

    let resultados = buscar(&indexador, Some("Notebook Gamer"), Some("Dell"), None);
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].id, 1);

    let resultados2 = buscar(&indexador, None, Some("Lenovo"), Some("Eletrônicos"));
    assert_eq!(resultados2.len(), 1);
    assert_eq!(resultados2[0].id, 2);
}
