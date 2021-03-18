pub use digest::{Digest, Output};
use std::vec::Vec;

pub struct Leaf<D: Digest, T> {
    pub hash: Output<D>,
    pub data: T,
}

impl<D: Digest, T> Leaf<D, T> {
    pub fn new(hash: Output<D>, data: T) -> Self {
        Self { hash, data }
    }
}

pub struct MerkleTree<D: Digest, T> {
    nodes: Vec<Output<D>>,
    leaves: Vec<Leaf<D, T>>,
}

impl<D: Digest, T> MerkleTree<D, T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            leaves: Vec::new(),
        }
    }

    pub fn hash(&self) -> Output<D> {
        if self.nodes.is_empty() {
            Output::<D>::default()
        } else {
            self.nodes[0].clone()
        }
    }

    pub fn insert(&mut self, i: usize, leaf: Leaf<D, T>) {
        self.leaves.insert(i, leaf);
        if self.leaves.len() == self.nodes.len() + 2 {
            // we need to create a new level of nodes. insert new 'parents' for
            // leaves. We then have to rehash all nodes.
            let l = self.nodes.len() + (self.leaves.len() * 2);
            self.nodes.resize(l, Output::<D>::default());
            self.rehash_nodes(0, l);
        } else {
            self.rehash_nodes(i, self.nodes.len());
        }
    }

    pub fn remove(&mut self, i: usize) {
        self.leaves.remove(i);
        if self.leaves.is_empty() {
            self.nodes.clear();
        } else if self.leaves.len() == (self.nodes.len() + 1) / 2 {
            let l = self.nodes.len() - self.leaves.len();
            self.nodes.truncate(l);
            self.rehash_nodes(0, l);
        } else {
            self.rehash_nodes(self.leaf_parent(i), self.nodes.len());
        }
    }

    pub fn replace(&mut self, i: usize, leaf: Leaf<D, T>) {
        self.leaves[i] = leaf;
        let j = self.leaf_parent(i);
        self.rehash_nodes(j, j);
    }

    pub fn push(&mut self, leaf: Leaf<D, T>) {
        self.insert(self.leaves.len(), leaf);
    }

    fn rehash_nodes(&mut self, start: usize, end: usize) {
        let (mut start, mut end) = (start, end);
        loop {
            for i in (start..end).rev() {
                self.rehash_node(i);
            }
            if start == 0 {
                return
            }
            start = self.node_parent(start);
            end = self.node_parent(end);    
        }
    }

    fn rehash_node(&mut self, i: usize) {
        let mut d = D::new();
        if i * 2 + 1 >= self.nodes.len() {
            // Our children are leaves.
            let j = self.left_child_leaf(i);
            if j < self.leaves.len() {
                d.update(self.leaves[j].hash.as_slice())
            }
            if j + 1 < self.leaves.len() {
                d.update(self.leaves[j + 1].hash.as_slice())
            }
        } else {
            // Our children are nodes. If we are here, we should have a full
            // level of nodes below us.
            let j = self.left_child_node(i);
            d.update(self.nodes[j].as_slice());
            d.update(self.nodes[j + 1].as_slice());
        }
        self.nodes[i] = d.finalize();
    }

    fn leaf_parent(&self, i: usize) -> usize {
        self.nodes.len() / 2 + i / 2 - 1
    }

    fn node_parent(&self, i: usize) -> usize {
        (i - 1) / 2
    }

    fn left_child_node(&self, i: usize) -> usize {
        i * 2 + 1
    }

    fn left_child_leaf(&self, i: usize) -> usize {
        (i - self.nodes.len() / 2) * 2
    }
}

impl<D: Digest, T> Default for MerkleTree<D, T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
