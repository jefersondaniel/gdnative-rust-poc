use std::sync::{Arc, Mutex};

use gdnative::prelude::*;

pub struct RootNode {
    pub node: Arc<Mutex<Ref<Node2D>>>,
    pub canvas_item_rid: Rid,
}

impl RootNode {
    pub fn new(node: &TRef<Node2D>) -> Self {
        Self {
            node: Arc::new(Mutex::new(node.claim())),
            canvas_item_rid: node.get_canvas_item(),
        }
    }
}
