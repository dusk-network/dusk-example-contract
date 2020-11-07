use crate::leaf::ContractLeaf;
use canonical_host::MemStore;
use poseidon252::tree::{PoseidonMaxAnnotation, PoseidonTree};

const DEPTH: usize = 17;

#[derive(Debug, Clone)]
pub struct Host {
    store: MemStore,
    tree: PoseidonTree<ContractLeaf, PoseidonMaxAnnotation, MemStore, DEPTH>,
}

impl Host {
    pub fn store(&self) -> &MemStore {
        self.as_ref()
    }

    pub fn tree(&self) -> &PoseidonTree<ContractLeaf, PoseidonMaxAnnotation, MemStore, DEPTH> {
        self.as_ref()
    }

    pub fn tree_mut(
        &mut self,
    ) -> &mut PoseidonTree<ContractLeaf, PoseidonMaxAnnotation, MemStore, DEPTH> {
        self.as_mut()
    }
}

impl Default for Host {
    fn default() -> Self {
        let tree = PoseidonTree::new();
        let store = MemStore::new();

        Host { tree, store }
    }
}

impl AsRef<MemStore> for Host {
    fn as_ref(&self) -> &MemStore {
        &self.store
    }
}

impl AsMut<MemStore> for Host {
    fn as_mut(&mut self) -> &mut MemStore {
        &mut self.store
    }
}

impl AsRef<PoseidonTree<ContractLeaf, PoseidonMaxAnnotation, MemStore, DEPTH>> for Host {
    fn as_ref(&self) -> &PoseidonTree<ContractLeaf, PoseidonMaxAnnotation, MemStore, DEPTH> {
        &self.tree
    }
}

impl AsMut<PoseidonTree<ContractLeaf, PoseidonMaxAnnotation, MemStore, DEPTH>> for Host {
    fn as_mut(
        &mut self,
    ) -> &mut PoseidonTree<ContractLeaf, PoseidonMaxAnnotation, MemStore, DEPTH> {
        &mut self.tree
    }
}

mod query;
mod transaction;

#[cfg(test)]
mod tests;
