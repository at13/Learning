fn insert(hnsw: hnsw, layer: &mut layer, node: node, entry_point: node) {
    // Insert the node into the layer
    let current_level: u64 = sample_l(layer.m_l);
    for level in hnsw.max_layers .. current_level {
        nearest_nodes = search(node, layer, 1, entry_point);
        entry_point = nearest_nodes[0];
    }
    for level in math::max(current_level, hnsw.max_layers) .. 0{
        nearest_nodes = search(node, layer, hnsw.max_neighbors[level], entry_point);
        for neighbor in nearest_nodes {
            connect(node, neighbor, level);
        }
    }

}