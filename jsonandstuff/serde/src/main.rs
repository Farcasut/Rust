
use std::fs;
use serde_derive::Deserialize;
#[derive(Debug, Deserialize, Clone)]
struct Person
{
    name: String,
    phone:String,
    age: u8
}
fn main() {
    let mut youngest:Person=Person { name: "Youngest".to_string(), phone: "".to_string(), age: 255};
    let mut oldest:Person=Person { name: "oldest".to_string(), phone: "".to_string(), age: 0};
    let file = fs::read_to_string("person.json").unwrap();
    for i in file.split('\n')
    {
        if i.len()==0
        {
            break;
        }
        let current:Person = serde_json::from_str(&i).unwrap();
        if current.age>oldest.age
            {oldest=current.clone();}
        if current.age<youngest.age
        {
            youngest=current.clone();
        }
    }
    print!("{:?}\n{:?}\n",youngest, oldest);
}
