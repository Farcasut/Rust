#![allow(non_snake_case)]
use std::{fs::read_to_string, collections::HashMap};
use serde_derive::Deserialize;
struct pair
{
    cuvant : String,
    aparitii:u8
}
#[derive(Debug, Deserialize)]
struct js
{
    name:String,
    category:String,
    group:String,
    htmlCode:Vec<String>,
    unicode:Vec<String>

}

fn problema1(text : String)
{
    let mut words:HashMap<String, u8>= HashMap::new();
    let mut maxLength=0;
    let file = read_to_string(text).expect("Err la open").to_lowercase();
    for i in file.split(|x| (x==' ' || x==',' || x=='.'))
    {
        if i.len()>maxLength
        {
        maxLength=i.len();
        }
        if i!="" && i!="\n"
        {
             *words.entry(i.to_string()).or_default()+=1;
        }
    }
    maxLength+=1;
   // words.sort_unstable_by_key(|x| );
    let mut ord= Vec::<pair>::new();
    for i in words
    {
        let pair=pair{cuvant:i.0, aparitii:i.1};
        ord.push(pair);
    }
    ord.sort_unstable_by(|x, y| y.aparitii.cmp(&x.aparitii));
    for i in ord
    {
        println!("{:<maxLength$}=>{}", i.cuvant, i.aparitii);
    }
 }


fn problema2()->Result< (), ureq::Error> 
{
    let data : String = 
    ureq::get("https://emojihub.herokuapp.com/api/all")
        .call()?
        .into_string()?;
    let mut json:Vec<js> = serde_json::from_str(&data).unwrap();
    //print!("{}",data);
    let mut gruped:HashMap<String, Vec<js>>= HashMap::new();
    for i in json
    {
        gruped.entry(i.group.clone()).or_insert(Vec::<js>::new()).push(i);
    }
    for i in gruped
    {
        print!("---------------Group name: {}-----------------\n", i.0);
        for j in i.1
        {
            print!("{}\n", j.name);
        }
        println!();
    }
   return Ok(());
}

fn main() {
    //problema1("text".to_string());
    problema2();
}
