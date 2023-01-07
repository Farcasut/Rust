/*
TODO
    ADD option for saving
    ADD support for multiple subreddits
    ADD support for custom path
✔  refactor the saving logic
*/

use chrono::prelude::DateTime;
use chrono::Utc;
use core::time;
use regex::Regex;
use serde_derive::Deserialize;
use clap::Parser;
use std::{
    collections::HashSet,
    fs::{create_dir_all, File},
    thread::sleep,
    time::{Duration, UNIX_EPOCH}
};
#[derive(Debug)]

enum MyErrorsInit
{
    InvalidSubreddit,
    InvalidArguments,
    RegexError(regex::Error),  
}

impl From<regex::Error> for MyErrorsInit
{
    fn from(error:regex::Error) ->Self
    {
        MyErrorsInit::RegexError(error)
    }
}

enum MyErrors {
   
    Req(reqwest::Error),
    IoError(std::io::Error),
    UreqError(ureq::Error), 
    SerdeJsonError(serde_json::Error)
}

impl From<serde_json::Error> for MyErrors
{
    fn from(error: serde_json::Error)->Self
    {
        MyErrors::SerdeJsonError(error)
    }
}

impl From<ureq::Error> for MyErrors
{
    fn from(error: ureq::Error)->Self
    {
        MyErrors::UreqError(error)
    }
}

impl From<reqwest::Error> for MyErrors {
    fn from(error: reqwest::Error) -> Self {
        MyErrors::Req(error)
    }
}

impl From<std::io::Error> for MyErrors {
    fn from(error: std::io::Error)-> Self
    {
        MyErrors::IoError(error)
    }
}



//json structures
#[derive(Debug, Deserialize, Clone)]
struct RedditVideo {
    fallback_url: String,
}
#[derive(Debug, Deserialize, Clone)]
struct Media {
    reddit_video: Option<RedditVideo>,
}
#[derive(Debug, Deserialize, Clone)]

struct RedditVideoPreview {
    fallback_url: String,
}
#[derive(Debug, Deserialize, Clone)]
struct Preview {
    reddit_video_preview: Option<RedditVideoPreview>,
}
#[derive(Debug, Deserialize, Clone)]
struct PostInfo {
    title: String,
    created: f64,
    permalink: String,
    url: String,
    is_video: bool,
    author: String,
    media: Option<Media>,
    preview: Option<Preview>,
}
#[derive(Debug, Deserialize, Clone)]

struct Children {
    data: PostInfo,
}
#[derive(Debug, Deserialize, Clone)]
struct Info {
    children: Vec<Children>,
}
#[derive(Debug, Deserialize, Clone)]
struct Reddit {
    data: Info,
}

fn check_params(args: &Args) -> Result<String, MyErrorsInit> {
  
    let path: String;
    let full_id =
    Regex::new(r#"^((https)|(http))(://www\.reddit\.com/r/)([a-zA-Z_]{3,21})$"#)?;
    let only_name = Regex::new(r#"^([A-Za-z_0-9]{3,21})$"#)?;
    let sort_by = Regex::new(r#"^((hot)|(new)|(top))$"#)?;
    if ! sort_by.is_match(&args.order)
    {
        return Err(MyErrorsInit::InvalidArguments)
    }
    if full_id.is_match(&args.reddit) {
        path = String::from(args.reddit.clone() + "/" + &args.order + ".json");
    } else if only_name.is_match(&args.reddit) {
        path = format!("https://www.reddit.com/r/{}/{}.json", args.reddit, args.order);
    } else {
        return Err(MyErrorsInit::InvalidSubreddit);
    }
    return Ok(path);
    
}
fn get_data(path: &String, download: u8)-> Result<(), MyErrors> {
    let time = time::Duration::from_secs(10);
    let mut printed: HashSet<String> = HashSet::new();
    let mut count: u64 = 0;
    loop {
        let data = ureq::get(path).call()?.into_string()?;
        let reddit: Reddit = serde_json::from_str(&data)?;
        print_data(&mut printed, reddit, &mut count, download)?;
        sleep(time);
        }
}
fn print_data(  printed: &mut HashSet<String>,   data: Reddit,   count: &mut u64,download:u8 ) -> Result<(), MyErrors> 
{
    let is_gif = Regex::new(r#"(gif)"#).unwrap();
    for i in data.data.children {
        let token = i.data.permalink.clone();
        if printed.insert(token) {
            let d = UNIX_EPOCH + Duration::from_secs(i.data.created as u64);
            let datetime = DateTime::<Utc>::from(d);
            let timestamp = datetime.format("%d.%m.%Y %H:%M:%S").to_string();
            print!(
                "Titlu:{}\nLink: www.reddit.com{}\nTime: {}\n",
                i.data.title, i.data.permalink, timestamp
            );
            //make directory
            let directory = "D:image\\";
            create_dir_all(&directory)?;
            //maybe make a function for this
        if download != 0
          { 
             if !is_gif.is_match(&i.data.url)
            //daca nu are gif in nume inseamna ca fie e imagine fie e video
            {
                if !i.data.is_video
                //flag video
                {
                    save_data(&i.clone(), "png", count, directory, &i.data.url)?;
                } else {
                    save_data(
                        &i.clone(),
                        "mp4",
                        count,
                        directory,
                        &i.data.media.unwrap().reddit_video.unwrap().fallback_url,
                    )?;
                }
            } else {
                if i.data
                    .preview
                    .as_ref()
                    .unwrap()
                    .reddit_video_preview
                    .is_some()
                {
                    save_data(
                        &i.clone(),
                        "mp4",
                        count,
                        directory,
                        &i.data
                            .preview
                            .unwrap()
                            .reddit_video_preview.unwrap()
                            .fallback_url,
                    )?;
             
                } else {
                    save_data(&i, "gif", count, directory, &i.data.url)?;
                }
            }
            }
        }
    }
    return Ok(());
}


fn save_data(
    current: &Children,
    format: &str,
    count: &mut u64,
    directory: &str,
    link: &String
) -> Result<(), MyErrors> {
    let filename = format!(
        "{}\\{}-{}.{}",
        directory, current.data.author, count, format
    );
    let mut file = File::create(filename)?;
    let _file_len = reqwest::blocking::get(link)?.copy_to(&mut file)?;
    *count += 1;
    return Ok(());
}

#[derive(Parser)]
#[command(version, about = "Arguments")]
struct Args {
   ///Name of the person to greet
   #[arg(short, long)]
   reddit: String,
   ///Number of times to greet
   #[arg(short, default_value = "hot")]
   order: String,
   #[arg(short, default_value_t = 0)]
   download: u8
}

fn main() {
    //let args: Vec<String> = env::args().collect();
    let args = Args::parse();
    match check_params(&args)
    { 
        Err(MyErrorsInit::InvalidSubreddit)=>println!("The name has to be between 3 and 21 charachters long and the only special character allowed is the underscore\n"),
        Err(MyErrorsInit::InvalidArguments) => println!("The arguments should be -r|--reddit subreddit [-o <new|hot|top>] [-d <0|1>]"),
        Err(MyErrorsInit::RegexError(x))=>{eprintln!("{:?}", x);}
        Ok(path)=> match get_data(&path, args.download)
        {
            Err(MyErrors::Req(x)) => {eprintln!("{:?}", x);},
            Err(MyErrors::IoError(x))=>{eprintln!("{:?}", x);},
            Err(MyErrors::UreqError(x))=>{eprintln!("{:?}", x);} ,
            Err(MyErrors::SerdeJsonError(x))=>{eprintln!("{:?}", x);} ,
            Ok(()) => println!("program finished🏁")
        }
    }
}
