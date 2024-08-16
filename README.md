# Graph Analysis Tool

Este projeto em Rust realiza análises detalhadas em grafos, incluindo o cálculo de métricas de rede como grau médio, coeficiente de agrupamento médio, e distância média. Além disso, ele gera gráficos visuais, como a distribuição de graus e uma visualização da centralidade de grau, utilizando as bibliotecas **petgraph**, **rayon**, e **plotters**.

## Funcionalidades

- **Cálculo de Grau Médio:** Calcula o grau médio de um grafo não direcionado.
- **Cálculo de Coeficiente de Agrupamento Médio:** Determina o coeficiente de agrupamento médio, medindo a tendência dos vértices em formar triângulos.
- **Cálculo de Distância Média:** Avalia a distância média entre todos os pares de vértices no grafo.
- **Leitura de Arquivos Comprimidos:** Lê arestas de um arquivo `.txt.gz` e as converte em um grafo.
- **Plotagem de Distribuição de Graus:** Gera um gráfico da distribuição de graus dos vértices.
- **Visualização de Centralidade de Grau:** Cria uma visualização onde o tamanho dos vértices é proporcional à sua centralidade de grau.

## Pré-requisitos

Para compilar e executar este projeto, você precisa ter o Rust instalado. Além disso, as seguintes crates são utilizadas:

- `flate2`: Para descompressão de arquivos `.gz`.
- `petgraph`: Para manipulação e análise de grafos.
- `rayon`: Para paralelização das operações.
- `plotters`: Para geração de gráficos.

## Como Usar

1. **Clonar o Repositório**
   ```bash
   git clone https://github.com/seu_usuario/seu_repositorio.git
   cd seu_repositorio
   ```

2. **Compilar o Projeto**
   ```bash
   cargo build --release
   ```

3. **Executar o Projeto**
   ```bash
   cargo run --release -- <dados.txt.gz> <dir_nome> <arq_saida_nome>
   ```

   - `<dados.txt.gz>`: Caminho para o arquivo comprimido contendo as arestas do grafo.
   - `<dir_nome>`: Nome do diretório onde os arquivos de saída serão salvos.
   - `<arq_saida_nome>`: Nome base do arquivo de saída (será salvo com a extensão `.env`).

## Exemplo de Uso

```bash
cargo run --release -- data/graph_data.txt.gz output results
```

Esse comando irá:

1. Criar o diretório `output` (se não existir).
2. Calcular as métricas de rede e salvar os resultados em `output/results.env`.
3. Gerar gráficos de distribuição de graus e visualização de centralidade dentro do diretório `output`.

## Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou enviar pull requests.


## Contato

Para dúvidas ou sugestões, entre em contato pelo e-mail: lr82460@gmail.com
