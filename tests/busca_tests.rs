use projeto::index::{Indexador, Produto};
use projeto::search::buscar;

#[test]
fn teste_busca_basica() {
    let mut indexador = Indexador::new();

    let p1 = Produto {
        id: 1,
        nome: "Smartphone X".to_string(),
        marca: "MegaTech".to_string(),
        categoria: "Eletrônicos".to_string(),
    };
    let p2 = Produto {
        id: 2,
        nome: "Camisa Azul".to_string(),
        marca: "FashionCo".to_string(),
        categoria: "Vestuário".to_string(),
    };

    indexador.adicionar_produto(p1.clone());
    indexador.adicionar_produto(p2.clone());

    // Busca por nome
    let resultados = buscar(&indexador, Some("smartphone x"), None, None);
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].id, p1.id);

    // Busca por marca
    let resultados = buscar(&indexador, None, Some("fashionco"), None);
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].id, p2.id);

    // Busca por categoria
    let resultados = buscar(&indexador, None, None, Some("eletrônicos"));
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].id, p1.id);

    // Busca combinada
    let resultados = buscar(&indexador, Some("camisa azul"), Some("fashionco"), Some("vestuário"));
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].id, p2.id);

    // Busca que não retorna nada
    let resultados = buscar(&indexador, Some("tablet"), None, None);
    assert_eq!(resultados.len(), 0);
}
