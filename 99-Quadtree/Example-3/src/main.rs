use quadtree::*;

fn main() {
    let tree = Tree<2>::with_depth(2).unwrap();

    println!("{:#?}", tree);
}
