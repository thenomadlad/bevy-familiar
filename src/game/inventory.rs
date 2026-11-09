use std::collections::BTreeMap;

use bevy::prelude::*;

const MAX_INDEX: usize = 27;

#[derive(Resource, Default, Clone, PartialEq, Eq)]
pub(super) struct Inventory {
    items: BTreeMap<usize, usize>,
}

impl Inventory {
    pub(super) fn add(&mut self, inventory_item: InventoryItem) {
        let entry = self.items.entry(inventory_item.0).or_default();
        *entry = *entry + 1;
    }

    pub(super) fn get_all(&self) -> Vec<(InventoryItem, usize)> {
        self.items
            .iter()
            .map(|(key, count)| (InventoryItem(*key), *count))
            .collect()
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub(super) struct InventoryItem(usize);

impl Default for InventoryItem {
    fn default() -> Self {
        Self::random_item()
    }
}

impl InventoryItem {
    pub(super) fn random_item() -> Self {
        InventoryItem(rand::random_range(1..=MAX_INDEX))
    }

    pub(super) fn index(&self) -> usize {
        self.0
    }
}
