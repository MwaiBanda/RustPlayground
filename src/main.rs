#[derive(Debug)]
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
    filter_int(|x| x > 5);

    (0..10).for_each(|x| println!("{}", x));
    (0..10).filter(|x| x < &5);
}

fn filter_int<Func: Fn(i32) -> bool> (call_back: Func) {
    if call_back(6) {
        println!("true");
    } else {
        println!("false");
    }

}


