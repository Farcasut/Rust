
use std::{collections::{HashSet},thread::sleep,env, time::{UNIX_EPOCH, Duration}, fs::{File, create_dir_all}};
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
//json structures
#[derive(Debug,Deserialize)]
struct RedditVideo
{
    fallback_url: String
}
#[derive(Debug,Deserialize)]
struct Media{
    reddit_video:Option<RedditVideo>
}
#[derive(Debug,Deserialize)]

struct RedditVideoPreview
{
    fallback_url:String
}
#[derive(Debug,Deserialize)]
struct Preview
{
    reddit_video_preview:Option<RedditVideoPreview>
}
#[derive(Debug,Deserialize)]
struct PostInfo
{
    title:String,
    created:f64,
    permalink:String,
    url:String,
    is_video:bool,
    author:String,
    media:Option<Media>,
    preview:Option<Preview>

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
    let only_name=Regex::new(r#"^([A-Za-z_0-9]{3,21})$"#).unwrap();
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
        path=format!("https://www.reddit.com/r/{}/{}.json", args[1], order);
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
 let mut printed:HashSet<String>= HashSet::new();
 let mut count:u64=0;
    loop {
        let data=ureq::get(path).call().unwrap().into_string().unwrap();
        let reddit:Reddit=serde_json::from_str(&data).unwrap();
        match print_data(&mut printed, reddit,&mut count)
        {
            Err(e) =>println!("{:?}",e),
            Ok(())=> sleep(time)
        }
    }

}
fn print_data(printed:&mut HashSet<String>, data:Reddit,count:&mut u64)->Result<(), reqwest::Error>
{
    let is_gif=Regex::new(r#"(gif)"#).unwrap();
    for i in data.data.children
    {
        let token=i.data.permalink.clone();
        if printed.insert(token)
        {
            let d= UNIX_EPOCH +Duration::from_secs(i.data.created as u64);
            let datetime= DateTime::<Utc>::from(d);
            let timestamp=datetime.format("%d.%m.%Y %H:%M:%S").to_string();
            print!("Titlu:{}\nLink: www.reddit.com{}\nTime: {}\n", i.data.title, i.data.permalink, timestamp);
            //make directory
            //let directory=format!("D:image\\{}", i.data.author); #if you want every user to have a folder
             let directory="D:image\\";
             create_dir_all(&directory).unwrap();
            //maybe make a function for this
            if !is_gif.is_match(&i.data.url)//daca nu are gif in nume inseamna ca fie e imagine fie e video
            {
            if !i.data.is_video //flag video
            {   
                let filename=format!("{}\\{}-{}.png", directory, i.data.author,count);
                let mut file = File::create(filename).unwrap();
                let _file_len = reqwest::blocking::get(&i.data.url)?
                .copy_to(&mut file)?;
                *count+=1;
            }
            else 
            {
                let filename=format!("{}\\{}-{}.mp4", directory,i.data.author, count);
                let mut file = File::create(filename).unwrap();
                let _file_len = reqwest::blocking::get(&i.data.media.unwrap().reddit_video.unwrap().fallback_url)?
                .copy_to(&mut file)?;
                *count+=1;
            }
            }
            else 
            {
                if i.data.preview.as_ref().unwrap().reddit_video_preview.is_some()
                {
                let filename=format!("{}\\{}.mp4", directory, count);
                let mut file = File::create(filename).unwrap();
                let _file_len = reqwest::blocking::get(&i.data.preview.unwrap().reddit_video_preview.unwrap().fallback_url)?
                .copy_to(&mut file)?;
                *count+=1;
                }
                else 
                {
                let filename=format!("{}\\{}.gif", directory, count);
                let mut file = File::create(filename).unwrap();
                let _file_len = reqwest::blocking::get(&i.data.url)?
                .copy_to(&mut file)?;
                *count+=1;
                }
            }
        }
    }
    return Ok(());
}
fn main() {
    let args: Vec<String>=env::args().collect();
    match check_params(args)
    {
        Err(MyErrors::InvalidSubreddit)=>println!("The name has to be between 3 and 21 charachters long and the only special character allowed is the underscore\n"),
        Err(MyErrors::InvalidArguments) => println!("Format <subreddit> [<hot|new|top>]"),
        Ok(path)=>  get_data(&path)
    }

}
