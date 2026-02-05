pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut parent: Vec<usize> = (0..=n).collect();

    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;

        // find root u
        let mut root_u = u;
        while root_u != parent[root_u] {
            parent[root_u] = parent[parent[root_u]];
            root_u = parent[root_u];
        }
        // find root v
        let mut root_v = v;
        while root_v != parent[root_v] {
            parent[root_v] = parent[parent[root_v]];
            root_v = parent[root_v];
        }

        if root_u == root_v {
            return vec![u as i32, v as i32];
        } else {
            parent[root_u] = root_v;
        }
    }

    vec![]
}
