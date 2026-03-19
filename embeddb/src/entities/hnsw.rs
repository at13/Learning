struct hnsw {
     max_layers: uint,
     max_neighbors: Vec<uint>,
     total_connections: uint,
     layers: Vec<layer>,
}