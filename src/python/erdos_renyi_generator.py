import networkx as nx
import os

# Parâmetros
n = 1000 
p_values = [i / 10 for i in range(1, 10)]
output_dir = "erdos_renyi_networks"

# Cria o diretório de saída se não existir
if not os.path.exists(output_dir):
    os.makedirs(output_dir)

# Gera e salva as redes ER
for i, p in enumerate(p_values):

    G = nx.erdos_renyi_graph(n, p)
    
    filename = os.path.join(output_dir, f"p_equal_{p:.1f}.txt")
    
    with open(filename, 'w') as f:
        for edge in G.edges():
            f.write(f"{edge[0]} {edge[1]}\n")
    
    print(f"Rede com p={p:.1f} salva em {filename}")

print("Todas as redes foram geradas e salvas.")
