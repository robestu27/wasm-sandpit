use serde::Serialize;



/// Simple Tree struct - we borrow the values just to put a structure on them
#[derive(Serialize)]
pub struct TreeNode<'a, T: Serialize> {
    pub value: &'a T,
    pub children: Vec<TreeNode<'a, T>>,
}


impl <'a, T: Serialize> TreeNode<'a, T> {

    // Creates a new tree node 
    pub fn new (value: &'a T) -> TreeNode<'a, T> {
        Self {value: value, children: Vec::new()}
    }


    // Adds a child to the tree
    pub fn add_child (&mut self, value: &'a T) -> &mut TreeNode<'a, T> {

        let new_node = Self {value: value, children: Vec::new()};

        self.children.push(new_node);
        self.children.last_mut().unwrap()

    }

}