use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use flate2::read::GzDecoder;
use petgraph::graph::{Graph, UnGraph};
use rayon::prelude::*;
use plotters::prelude::*;
use std::fs;

/// Calcula o grau médio de um grafo.
///
/// # Argumentos
///
/// * `graph` - O grafo para o qual o grau médio deve ser calculado.
///
/// # Retorna
///
/// * O grau médio do grafo como um valor `f64`.
fn average_degree(graph: &UnGraph<(), ()>) -> f64 {
    let total_degrees: usize = graph
        .node_indices()
        .par_bridge()
        .map(|i| graph.neighbors(i).count())
        .sum();
    total_degrees as f64 / graph.node_count() as f64
}

/// Calcula o coeficiente de agrupamento médio de um grafo.
///
/// # Argumentos
///
/// * `graph` - O grafo para o qual o coeficiente de agrupamento médio deve ser calculado.
///
/// # Retorna
///
/// * O coeficiente de agrupamento médio do grafo como um valor `f64`.
fn average_clustering(graph: &UnGraph<(), ()>) -> f64 {
    let clustering_sum: f64 = graph
        .node_indices()
        .par_bridge()
        .map(|node| {
            let neighbors: Vec<_> = graph.neighbors(node).collect();
            let neighbor_count = neighbors.len();
            if neighbor_count < 2 {
                return 0.0;
            }

            let mut connected_neighbor_pairs = 0;
            for i in 0..neighbor_count {
                for j in i + 1..neighbor_count {
                    if graph.contains_edge(neighbors[i], neighbors[j]) {
                        connected_neighbor_pairs += 1;
                    }
                }
            }

            let possible_pairs = neighbor_count * (neighbor_count - 1) / 2;
            connected_neighbor_pairs as f64 / possible_pairs as f64
        })
        .sum();

    clustering_sum / graph.node_count() as f64
}

/// Calcula a distância média de um grafo.
///
/// # Argumentos
///
/// * `graph` - O grafo para o qual a distância média deve ser calculada.
///
/// # Retorna
///
/// * A distância média do grafo como um valor `f64`.
fn average_distance(graph: &UnGraph<(), ()>) -> f64 {
    let (total_distance, pair_count): (f64, usize) = graph
        .node_indices()
        .par_bridge()
        .map(|start_node| {
            let distances = petgraph::algo::dijkstra(graph, start_node, None, |_| 1.0);
            distances
                .iter()
                .filter(|(&end_node, _)| start_node != end_node)
                .map(|(_, &dist)| dist)
                .fold((0.0, 0), |(sum, count), dist| (sum + dist, count + 1))
        })
        .reduce(|| (0.0, 0), |(sum1, count1), (sum2, count2)| (sum1 + sum2, count1 + count2));

    total_distance / pair_count as f64
}

/// Lê as arestas de um arquivo `.txt.gz` e as retorna como um vetor de pares de inteiros.
///
/// # Argumentos
///
/// * `file_path` - O caminho para o arquivo `.txt.gz` contendo as arestas.
///
/// # Retorna
///
/// * Um vetor de pares de inteiros representando as arestas do grafo.
fn read_edges_from_gz(file_path: &str) -> Vec<(i32, i32)> {
    let file = File::open(file_path).expect("Não foi possível abrir o arquivo");
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);

    reader
        .lines()
        .filter_map(|line| {
            if let Ok(line) = line {
                let parts: Vec<i32> = line
                    .split_whitespace()
                    .map(|x| x.parse().expect("Erro ao converter string para i32"))
                    .collect();
                if parts.len() == 2 {
                    return Some((parts[0], parts[1]));
                }
            }
            None
        })
        .collect()
}

/// Plota a distribuição de graus em um gráfico.
///
/// # Argumentos
///
/// * `graph` - O grafo para o qual a distribuição de graus deve ser plotada.
/// * `dir_name` - O nome do diretório onde o arquivo de imagem será salvo.
fn plot_degree_distribution(graph: &UnGraph<(), ()>, dir_name: &str) {
    let degrees: Vec<usize> = graph
        .node_indices()
        .map(|i| graph.neighbors(i).count())
        .collect();

    let max_degree = *degrees.iter().max().unwrap_or(&0);
    let degree_count = degrees.iter().fold(vec![0; max_degree + 1], |mut counts, &deg| {
        counts[deg] += 1;
        counts
    });
    let file_png = &format!("{}/{}_degree_distribution.png", dir_name, dir_name);
    let root_area = BitMapBackend::new(file_png, (640, 480)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Distribuição de Graus", ("sans-serif", 50).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(20)
        .build_cartesian_2d(0..max_degree as i32, 0..degree_count.iter().cloned().max().unwrap_or(0) as i32)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Grau")
        .y_desc("Frequência")
        .draw()
        .unwrap();

    chart
        .draw_series(
            degree_count
                .iter()
                .enumerate()
                .map(|(deg, count)| {
                    Circle::new((deg as i32, *count as i32), 5, RED.filled())
                })
        )
        .unwrap();
}

/// Plota a rede com tamanhos dos vértices proporcionais à centralidade.
///
/// # Argumentos
///
/// * `graph` - O grafo para o qual a visualização da centralidade deve ser plotada.
/// * `dir_name` - O nome do diretório onde o arquivo de imagem será salvo.
fn plot_network_with_centrality(graph: &UnGraph<(), ()>, dir_name: &str) {
    let centrality: HashMap<_, _> = graph
        .node_indices()
        .map(|i| {
            let degree_centrality = graph.neighbors(i).count() as f64;
            (i, degree_centrality)
        })
        .collect();

    let file_png = &format!("{}/{}_network_with_centrality.png", dir_name, dir_name);
    let root_area = BitMapBackend::new(file_png, (800, 600)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Rede com Centralidade de Grau", ("sans-serif", 50).into_font())
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(20)
        .build_cartesian_2d(0..800, 0..600)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Posição")
        .y_desc("Centralidade")
        .draw()
        .unwrap();

    chart
        .draw_series(
            centrality
                .iter()
                .map(|(&i, &centrality_value)| {
                    Circle::new((i.index() as i32 * 10, centrality_value as i32 * 10), 5, BLUE.filled())
                })
        )
        .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Uso: {} <dados.txt.gz> <dir_nome> <arq_saida.txt>", args[0]);
        std::process::exit(1);
    }

    match fs::create_dir_all(&args[2]) {
        Ok(_) => println!("Diretório criado com sucesso!"),
        Err(e) => eprintln!("Erro ao criar o diretório: {}", e),
    }

    let file_path = &args[1];
    let output_file = &format!("{}/{}", &args[2], &args[3]);

    let edges = read_edges_from_gz(file_path);

    let mut graph = Graph::new_undirected();
    let mut node_indices = HashMap::new();
    for &(u, v) in &edges {
        let u_index = *node_indices.entry(u).or_insert_with(|| graph.add_node(()));
        let v_index = *node_indices.entry(v).or_insert_with(|| graph.add_node(()));
        graph.add_edge(u_index, v_index, ());
    }

    let avr_clustering = average_clustering(&graph);
    let avr_degree = average_degree(&graph); 
    let avr_distance = average_distance(&graph);

    let mut output = File::create(output_file).expect("Não foi possível criar o arquivo de saída");
    writeln!(output, "AVERAGE_CLUSTERING={}", avr_clustering).expect("Erro ao escrever no arquivo de saída");
    writeln!(output, "AVERAGE_DEGREE={}", avr_degree).expect("Erro ao escrever no arquivo de saída");
    writeln!(output, "AVERAGE_DISTANCE={}", avr_distance).expect("Erro ao escrever no arquivo de saída");

    println!("Resultado salvo em '{}'", output_file);

    plot_degree_distribution(&graph, &args[2]);
    plot_network_with_centrality(&graph, &args[2]);
}
