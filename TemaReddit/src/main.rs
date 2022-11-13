
use std::{collections::HashMap,thread::sleep,env, time::{UNIX_EPOCH, Duration}};
use regex::Regex;
use chrono::prelude::DateTime;
use chrono::Utc;
use core::time;
use serde_derive::Deserialize;
#[derive(Debug)]
enum MyErrors
{
    InvalidSubreddit,
    InvalidArguments
}
//////json structures
#[derive(Debug,Deserialize)]
struct PostInfo
{
    title:String,
    created:f64,
    permalink:String
}
#[derive(Debug,Deserialize)]

struct Children
{
    data:PostInfo
}
#[derive(Debug,Deserialize)]
struct Info
{
    children:Vec<Children>
}
#[derive(Debug,Deserialize)]
struct Reddit
{
    data:Info
}

fn check_params(args:Vec<String>)->Result<String, MyErrors>
{
    let size=args.len();
    if size<2
       { return Err(MyErrors::InvalidArguments);}
    let path:String;
    let mut order=String::from("hot");
    let full_id=Regex::new(r#"^((https)|(http))(://www\.reddit\.com/r/)([a-zA-Z_]{3,21})$"#).unwrap();
    let only_name=Regex::new(r#"^([A-Za-z_]{3,21})$"#).unwrap();
    let sort_by=Regex::new(r#"^((hot)|(new)|(top))$"#).unwrap();
    if size==3
    {
        if sort_by.is_match(&args[2])
        {
            order=args[2].clone();
        }
    }
    if full_id.is_match(&args[1])
    {
        path=String::from(args[1].clone()+"/"+&order+".json");
    }
    else if only_name.is_match(&args[1]) 
    {
        path=String::from("https://www.reddit.com/r/".to_string()+&args[1].clone()+"/"+&order+".json");
    }
    else
    {
        return Err(MyErrors::InvalidSubreddit);
    }
    return Ok(path)
}
fn get_data(path:&String)
{
 let time=time::Duration::from_secs(10);
 let mut printed:HashMap<String, bool>= HashMap::new();

    loop {
        let data=ureq::get(path).call().expect("msg").into_string().expect("2");
        let data:Reddit=serde_json::from_str(&data).unwrap();
        print_data(&mut printed, data);
        sleep(time);
    }

}
fn print_data(printed:&mut HashMap<String, bool>, data:Reddit)
{
    for i in data.data.children
    {
        let token=i.data.permalink.clone()+&i.data.title+&i.data.created.to_string();
        printed.entry(token.clone()).or_insert(false);
        let p=printed.get_mut(&token).expect("Err la get");
        if *p==false
        {
            *p=true;
            let d= UNIX_EPOCH +Duration::from_secs(i.data.created as u64);
            let datetime= DateTime::<Utc>::from(d);
            let timestamp=datetime.format("%d.%m.%Y %H:%M:%S").to_string();
            print!("Titlu:{}\nLink: www.reddit.com{}\nTime: {}\n", i.data.title, i.data.permalink, timestamp);     
        }
    }
}
fn main() {
    let args: Vec<String>=env::args().collect();
    match check_params(args)
    {
        Err(MyErrors::InvalidSubreddit)=>println!("3<=Length(name)<=21 && name=[a-zA-Z_]"),
        Err(MyErrors::InvalidArguments) => println!("Format <subreddit> [<hot|new|top>]"),
        Ok(path)=>  get_data(&path)
    }
  
}
