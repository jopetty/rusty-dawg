pub mod vec;  // Implement VecBacking for DiskVec and DiskVecItem
pub mod disk_vec;
mod disk_mut_refs;  // Raw implementation of DiskVec data structure.

use graph::avl_graph::edge::Edge;
use graph::avl_graph::node::Node;

use graph::indexing::{EdgeIndex, IndexType, NodeIndex};
use graph::memory_backing::MemoryBacking;
use std::marker::PhantomData;
use std::path::Path;
use weight::Weight;
use serde::Serialize;
use serde::de::DeserializeOwned;

use self::disk_mut_refs::{DiskNodeMutRef, DiskEdgeMutRef};
use self::vec::Vec;

#[derive(Clone)]
pub struct DiskBacking<N, E, Ix> {
    dir_path: Box<Path>,
    marker: PhantomData<(N, E, Ix)>,
}

impl<N, E, Ix> DiskBacking<N, E, Ix> {
    pub fn new<P: AsRef<Path> + Clone + std::fmt::Debug>(dir_path: P) -> Self {
        Self {dir_path: Box::from(dir_path.as_ref()), marker: PhantomData}
    }
}

impl<N, E, Ix> MemoryBacking<N, E, Ix> for DiskBacking<N, E, Ix>
where
    Ix: IndexType + Copy + Serialize + DeserializeOwned,
    N: Weight + Serialize + DeserializeOwned + Default + Clone,
    E: Copy + Serialize + DeserializeOwned + Default,
{
    type NodeRef = Node<N, Ix>;
    type EdgeRef = Edge<E, Ix>;
    type NodeMutRef = DiskNodeMutRef<N, Ix>;
    type EdgeMutRef = DiskEdgeMutRef<E, Ix>;

    // This Vec type wraps a DiskVec in an Rc<RefCell<..>>
    type VecN = Vec<Node<N, Ix>>;
    type VecE = Vec<Edge<E, Ix>>;

    // The disk-backed implementations of new_node_vec and new_edge_vec should pass file_path when they construct a new Vector.

    fn new_node_vec(&self, capacity: Option<usize>) -> Self::VecN {
        let path = self.dir_path.join("nodes.vec");
        match capacity {
            Some(n) => Vec::new(path, n).unwrap(),
            None => Vec::new(path, 8).unwrap(),
        }
    }

    fn new_edge_vec(&self, capacity: Option<usize>) -> Self::VecE {
        let path = self.dir_path.join("edges.vec");
        match capacity {
            Some(n) => Vec::new(path, n).unwrap(),
            None => Vec::new(path, 8).unwrap(),
        }
    }
}
