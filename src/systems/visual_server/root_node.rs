use gdnative::prelude::*;

pub struct RootNode {
    pub canvas_item_rid: Rid,
    pub store: Dictionary,
}

impl RootNode {
    pub fn new(node: &TRef<Node2D>, store: Dictionary) -> Self {
        Self {
            canvas_item_rid: node.get_canvas_item(),
            store,
        }
    }

    pub fn set_cache(&mut self, key: &str, value: Variant) {
        let store_unique = unsafe { self.store.new_ref().assume_unique() };

        store_unique.insert(key, value);
    }

    pub fn get_cache(&mut self, key: &str) -> Variant {
        let store_unique = unsafe { self.store.new_ref().assume_unique() };

        store_unique.get(key)
    }
}
