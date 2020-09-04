use bst::tree::Tree;
use rand::Rng;

fn main() {
    let nodes_count = 100;
    let mut tree = Tree::new();
    let mut rng = rand::thread_rng();
    for _i in 0..nodes_count {
        let value = rng.gen_range(0, nodes_count * 10);
        tree.put(value, value);
    }
    println!("{}", tree);
}
