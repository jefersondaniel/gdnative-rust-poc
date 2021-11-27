use gdnative::prelude::*;

pub struct RootNode {
    pub canvas_item_rid: Rid,
}

impl RootNode {
    pub fn new(node: &TRef<Node2D>) -> Self {
        Self {
            canvas_item_rid: node.get_canvas_item(),
        }
    }
}
