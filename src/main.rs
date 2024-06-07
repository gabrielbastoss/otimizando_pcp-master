use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

#[derive(Debug)]
struct Produto {
    nome: String,
    data_entrega: DateTime<Utc>,
    quantidade: u32,
}

#[derive(Debug)]
struct MateriaPrima {
    nome: String,
    quantidade_por_produto: HashMap<String, u32>,
}

#[derive(Debug)]
struct PedidoCompra {
    nome_materia_prima: String,
    data_pedido: DateTime<Utc>,
    quantidade_total: u32,
    quantidade_minima: u32,
}

fn calcular_necessidades(produtos: &[Produto], materias_primas: &[MateriaPrima]) -> HashMap<String, u32> {
    let mut necessidades = HashMap::new();

    for produto in produtos {
        for materia in materias_primas {
            if let Some(&quantidade_por_produto) = materia.quantidade_por_produto.get(&produto.nome) {
                let quantidade_necessaria = quantidade_por_produto * produto.quantidade;
                let entry = necessidades.entry(materia.nome.clone()).or_insert(0);
                *entry += quantidade_necessaria;
            }
        }
    }

    necessidades
}


fn planejar_compras(
    necessidades: &HashMap<String, u32>,
    estoque_atual: &HashMap<String, u32>,
    lead_time: &HashMap<String, Duration>,
    data_atual: DateTime<Utc>,
    quantidade_minima: &HashMap<String, u32>,
    capacidade_armazenamento: &HashMap<String, u32>,
) -> Vec<PedidoCompra> {
    let mut pedidos = Vec::new();

    for (nome_materia_prima, &quantidade_necessaria) in necessidades {
        let quantidade_em_estoque = *estoque_atual.get(nome_materia_prima).unwrap_or(&0);
        let quantidade_a_comprar = if quantidade_em_estoque >= quantidade_necessaria {
            0
        } else {
            quantidade_necessaria - quantidade_em_estoque
        };

        if quantidade_a_comprar > 0 {
            let data_pedido = data_atual + *lead_time.get(nome_materia_prima).unwrap_or(&Duration::days(0));
            let quantidade_minima = *quantidade_minima.get(nome_materia_prima).unwrap_or(&1);
            let capacidade = *capacidade_armazenamento.get(nome_materia_prima).unwrap_or(&u32::MAX);
            let quantidade_total = quantidade_a_comprar.min(capacidade);

            if quantidade_total >= quantidade_minima {
                pedidos.push(PedidoCompra {
                    nome_materia_prima: nome_materia_prima.clone(),
                    data_pedido,
                    quantidade_total,
                    quantidade_minima,
                });
            }
        }
    }

    pedidos
}

fn main() {

    let produtos = vec![
        Produto { nome: "ProdutoA".to_string(), data_entrega: Utc::now() + Duration::days(10), quantidade: 100 },
        Produto { nome: "ProdutoB".to_string(), data_entrega: Utc::now() + Duration::days(20), quantidade: 150 },
    ];

    let materias_primas = vec![
        MateriaPrima { nome: "MateriaPrima1".to_string(), quantidade_por_produto: [("ProdutoA".to_string(), 2), ("ProdutoB".to_string(), 3)].iter().cloned().collect() },
        MateriaPrima { nome: "MateriaPrima2".to_string(), quantidade_por_produto: [("ProdutoA".to_string(), 1)].iter().cloned().collect() },
    ];

    let estoque_atual: HashMap<String, u32> = [("MateriaPrima1".to_string(), 50), ("MateriaPrima2".to_string(), 20)].iter().cloned().collect();
    let lead_time: HashMap<String, Duration> = [("MateriaPrima1".to_string(), Duration::days(5)), ("MateriaPrima2".to_string(), Duration::days(7))].iter().cloned().collect();
    let quantidade_minima: HashMap<String, u32> = [("MateriaPrima1".to_string(), 10), ("MateriaPrima2".to_string(), 5)].iter().cloned().collect();
    let capacidade_armazenamento: HashMap<String, u32> = [("MateriaPrima1".to_string(), 200), ("MateriaPrima2".to_string(), 100)].iter().cloned().collect();

    let necessidades = calcular_necessidades(&produtos, &materias_primas);
    let pedidos = planejar_compras(&necessidades, &estoque_atual, &lead_time, Utc::now(), &quantidade_minima, &capacidade_armazenamento);

    for pedido in pedidos {
        println!("{:?}", pedido);
    }
}
