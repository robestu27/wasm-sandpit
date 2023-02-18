use serde::Serialize;



/// Simple Tree struct - we borrow the values just to put a structure on them
#[derive(Serialize)]
pub struct PrimeTreeNode {
    pub key: i32,
    pub label: String,
    pub children: Vec<PrimeTreeNode>,
}


impl PrimeTreeNode {

    // Creates a new tree node 
    pub fn new (key: i32, label: String) -> Self {
        Self {key: key, label: label, children: Vec::new()}
    }


    // Adds a child to the tree
    pub fn add_child (&mut self, key: i32, label: String) -> &mut Self {

        let new_node = Self {key: key, label: label, children: Vec::new()};

        self.children.push(new_node);
        self.children.last_mut().unwrap()

    }

}