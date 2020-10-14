mod annoy;
mod common;
use crate::annoy::annoy::AnnoyIndexer;

fn main() {
    let a = vec![1.2, 2.3, 3.0];
    let b = vec![4.1, 5.0, 6.5];

    match common::calc::dot(&a, &b) {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }

    match common::calc::manhanttan_distance(&a, &b) {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }

    match common::calc::enclidean_distance(&a, &b) {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }

    println!("{}", common::calc::get_norm(&a));

    let angular = annoy::annoy::Angular {};
    let mut aix = annoy::annoy::AnnoyIndex::new(2, angular);

    let x: Vec<f64> = vec![1.1, 1.2, 60.6, 77.7, 88.8, 1.3, 1.4, 61.6, 78.7, 89.8];
    let y: Vec<f64> = vec![1.1, 1.2, 60.6, 77.7, 88.8, 1.3, 1.4, 61.6, 78.7, 89.8];

    for i in 0..x.len() {
        let f = vec![x[i], y[i]];
        aix.add_item(i as i32, &f, angular);
    }

    println!("{:?}", aix.build(1));
    println!("{:?}", aix.get_node(2));
    let f = vec![1.0, 1.0];
    println!("{:?}", aix);
    for i in 0..aix.nodes.len() {
        println!("{:?} {:?}",i, aix.nodes[i]);
    }
    println!("{:?}", aix.get_all_nns(&f, 2, 2));
}
