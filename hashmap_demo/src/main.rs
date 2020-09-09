use std::collections::HashMap;

fn main() {
    let mut score: HashMap<String, i32> = HashMap::new();
    score.insert(String::from("Red"), 10);

    let keys = vec![String::from("Blue"), String::from("Red")];
    let values = vec![10, 20];
    let score: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

    let k = String::from("Red");
    if let Some(v) = score.get(&k) {
        println!("v = {}", v);
    }

    for (k, v) in &score {
        println!("k = {}, v = {}", k, v);
    }

    //键不存在就插入
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("Blue"), 4);
    map.entry(String::from("Red")).or_insert(3);
    map.entry(String::from("Blue")).or_insert(5);
    println!("map = {:#?}", map);

    println!("Hello, world!");
}
