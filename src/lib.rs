use daggy::{Dag, NodeIndex, Walker};
use daggy::petgraph::visit::{Bfs, Dfs};

#[derive(Debug, Default)]
pub struct Transaction {
    pub timestamp: Option<usize>
}

pub struct EmptyWeight;

pub type IndexType = u32;
pub type Index = NodeIndex<IndexType>;
pub type DagType = Dag<Transaction, EmptyWeight, IndexType>;

/// The depth of a node in a rooted tree is the number of edges in the path from the root to the node.
/// Returns a vector of depths of the nodes by index
pub fn get_depths(dag: &DagType) -> Vec<usize> {
    let mut depths: Vec::<Option<usize>> = vec![None; dag.node_count()];
    depths[0] = Some(0); // the depths from 0 to 0 = 0

    let mut dfs_walker = Dfs::new(&dag, Index::new(0));
    while let Some(parent) = dfs_walker.walk_next(&dag) {
        dag.children(parent).iter(&dag).for_each(|(_, child)| {
            if depths[child.index()].is_none() {
                depths[child.index()] = depths[parent.index()].map(|distance| distance + 1);
            }
        });
    }
    let depths = depths.into_iter()
        .map(|x| x.expect("All nodes must have a depth"))
        .collect();
    depths
}

/// Average number of in-references per node
pub fn avg_in_refs(dag: &DagType) -> f32 {
    let mut in_refs: Vec::<usize> = vec![0; dag.node_count()];
    let mut dfs_walker = Dfs::new(&dag, Index::new(0));
    while let Some(parent) = dfs_walker.walk_next(&dag) {
        in_refs[parent.index()] = dag.children(parent).iter(&dag).count();
    }
    let in_refs_sum: usize = in_refs.iter().sum();
    in_refs_sum as f32 / dag.node_count() as f32
}

/// Average depth of the DAG
pub fn avg_depth(dag: &DagType) -> f32 {
    let depths = get_depths(dag);

    let depth_sum: usize = depths.into_iter().sum();
    depth_sum as f32 / dag.node_count() as f32
}

/// Average number of transactions per depth (not including depth 0)
pub fn avg_txs_depth(dag: &DagType) -> f32 {
    if dag.node_count() == 1 {
        // in this case we don't have enough nodes to calc this stat
        // I can use None but I prefer NaN to print it more easily
        return std::f32::NAN
    }
    let depths = get_depths(dag);
    let max_depth = depths.into_iter().max().expect("There must be a max depth by definition");
    (dag.node_count() - 1) as f32 / max_depth as f32
}

/// The number of tips of the ledger
pub fn total_tips(dag: &DagType) -> usize {
    let mut tips = 0;
    let mut bfs_walker = Bfs::new(&dag, Index::new(0));
    while let Some(parent) = bfs_walker.walk_next(&dag) {
        if dag.children(parent).iter(&dag).count() == 0 {
            tips = tips + 1;
        }
    }
    tips
}

/// Parse the given file and return a correct Dag
pub fn parse_database(file: std::fs::File) -> DagType {
    use std::io::BufRead;

    let mut database = std::io::BufReader::new(file);

    let mut dag = DagType::new();
    let _origin = dag.add_node(Transaction::default());

    // start reading the file

    let mut buf = String::new();

    // Parse N
    let bytes_read = database.read_line(&mut buf).expect("Could not read N");
    assert!(bytes_read > 0, "File is empty");

    let n: usize = buf.trim().parse().expect("N is not an int");

    // Add N nodes to the DAG
    for _ in 0..n {
        dag.add_node(Transaction::default());
    }

    for i in 0..n {
        buf.clear();
        let bytes_read = database.read_line(&mut buf).expect("Could not read node data");
        assert!(bytes_read > 0, "Expected {} nodes but there are only {} in the file", n, i);

        let node_data: Vec<&str> = buf.trim().split(' ').collect();
        assert_eq!(node_data.len(), 3, "There should be exactly 3 ints in the node data at line {}", i+1);

        let left_parent: usize = node_data[0].parse().expect("Left parent is not an int");
        let right_parent: usize = node_data[1].parse().expect("Right parent is not an int");
        let timestamp: usize = node_data[2].parse().expect("Timestamp is not an int");

        dag[Index::new(i+1)].timestamp.replace(timestamp);

        assert!(left_parent > 0, "Left parent should be > 0 at line {}", i+1);
        assert!(right_parent > 0, "Right parent should be > 0 at line {}", i+1);

        if dag.add_edge(Index::new(left_parent-1), Index::new(i+1), EmptyWeight).is_err() {
            panic!("The DAG would cycle adding left parent {} to node {}", left_parent, i+2);
        }
        if dag.add_edge(Index::new(right_parent-1), Index::new(i+1), EmptyWeight).is_err() {
            panic!("The DAG would cycle adding right parent {} to node {}", right_parent, i+2)
        }
    }

    let bytes_read = database.read_line(&mut buf).expect("Could not read node data");
    assert!(bytes_read == 0, "Expected {} nodes only but there are more data left in the file", n);

    dag
}
