use quadtree::*;

fn main() {
    let tree = BinaryTree::until_depth(2);

    println!("{:#?}", tree);
}
