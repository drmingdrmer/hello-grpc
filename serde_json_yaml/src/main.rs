use serde_json;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct C {
    x:i32,
    y:i32,
    n :Node,
    m : BTreeMap<i32, Node>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Node {
    ip: i32,
    port: i32,
}

fn main() {
    let mut x = C{
        x:1,
        y:2,
        n: Node{ip:1, port:2},
        m :BTreeMap::new(),
    };

    x.m.insert(4, Node{ip:3, port:4});

    let ss = serde_json::to_string(&x).unwrap();
    println!("ss={}", ss);

    let yy = serde_yaml::to_string(&x).unwrap();
    println!("yy={}", yy);
}
