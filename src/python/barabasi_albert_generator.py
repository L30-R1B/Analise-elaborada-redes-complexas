import networkx as nx
import os

# Parâmetros
m = 4 
n_values = [i * 1000 for i in range(1, 11)] 
output_dir = "barabasi_albert_networks" 

# Cria o diretório de saída se não existir
if not os.path.exists(output_dir):
    os.makedirs(output_dir)

# Gera e salva as redes Barabási-Albert
for n in n_values:

    G = nx.barabasi_albert_graph(n, m)
    
    filename = os.path.join(output_dir, f"n_equal_{n}.txt")
    
    with open(filename, 'w') as f:
        for edge in G.edges():
            f.write(f"{edge[0]} {edge[1]}\n")
    
    print(f"Rede com N={n} salva em {filename}")

print("Todas as redes foram geradas e salvas.")
