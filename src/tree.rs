use serde::Serialize;

use crate::Party;



/// Simple Tree struct - we borrow the values just to put a structure on them
#[derive(Serialize)]
pub struct PrimeTreeNode {
    pub key: i32,
    pub data: Party,
    pub children: Vec<PrimeTreeNode>,
}


impl PrimeTreeNode {

    // Creates a new tree node 
    pub fn new (key: i32, data: Party) -> Self {
        Self {key, data, children: Vec::new()}
    }


    // Adds a child to the tree
    pub fn add_child (&mut self, key: i32, data: Party) -> &mut Self {

        let new_node = Self {key, data, children: Vec::new()};

        self.children.push(new_node);
        self.children.last_mut().unwrap()

    }

}