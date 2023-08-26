use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main(){
    let person = Person {
        name: String::from("John"),
        age: 32,
    };
    println!("Hello, {:?}!", person);

    let json = json!([{
        "name": "Mwai Banda",
        "age": 22,
    }]);

    let json_str = serde_json::to_string(&person).unwrap();
    
    let p2: Vec<Person> = serde_json::from_value(json).unwrap();
    
    println!("{:?}", json_str);
    println!("{:?}", p2);
    filter_int(|x| x > 5);

    (0..10).for_each(|x| print!("{}", x));
    println!();
    (0..10).filter(|x| x < &5).map(|x| x + 1).for_each(|x| print!("{}", x));
    println!();
    println!("{}", (0..10).filter(|x| x < &5).map(|x| x + 1).reduce(|a,b| a + b).unwrap_or(0));

}

fn filter_int<Func: Fn(i32) -> bool> (call_back: Func) {
    if call_back(6) {
        println!("true");
    } else {
        println!("false");
    }

}


