
use std::env;
use std::fmt::format;
use std::fs::File;
use std::io::{Read, Write};
use clap::Parser;
enum MyError
{
    EmptyArgs,
    IO(std::io::Error)
}
impl From<std::io::Error> for MyError
{
    fn from(error:std::io::Error)->Self
    {
        MyError::IO(error)
    }
}
#[derive(Parser)]
#[command(version, about = "arguments for converting an image from binary to hex")]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   infile: String,

   /// Number of times to greet
   #[arg(short, long, default_value = "out.txt")]
   outfile: String,
   #[arg(short, long, default_value_t = 0)]
   decode: u8
}

fn ex1(path_in:&str, path_out:&str)-> Result<(), std::io::Error> {
    print!("{}", path_in);
    let mut file_in = File::open(path_in)?;
    let mut file_out = File::create(path_out)?;
    let mut buffer = [0; 4096];
    loop {
        let read = file_in.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        for i in 0..read
        {
            file_out.write_fmt(format_args!("{:02x}", buffer[i]))?;
        }
    }

    Ok(())
}
fn ex2(args:Vec<String>) -> Result<(), MyError> 
{
    if args.len()<2
    {
        return Err(MyError::EmptyArgs);
    }
    ex1(&args[0], &args[1])?;
    Ok(())
}
fn ex3(args:Args)-> Result<(), MyError> 
{
    ex1(&args.infile, &args.outfile)?;
    Ok(())
}
fn binary(c:u8)->u8
{
    match c
    {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a',
        _ => 0
    }
}
fn decode(out:&str)-> Result<(), MyError> 
{

    
    let mut file_in = File::open(out)?;
    let dec=format!("{}.deocde", out);
    let mut file_out = File::create(dec)?;
    let mut buffer = [0; 4096];
    loop {
        let read = file_in.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        for i in 0..read
        {
            file_out.write_fmt(format_args!("{:x}", buffer[i]))?;

        }
    }
    Ok(())
}
fn ex4(args:Args)-> Result<(), MyError> 
{
    ex1(&args.infile, &args.outfile)?;
    if args.decode==1
    {
        decode(&args.outfile);
    }
    Ok(())
}
fn main()  {
    //ex1("in.png", "out.txt");
    //let args: Vec<String>=env::args().collect();
    //match ex2(args) 
    //{
    //    Err(MyError::EmptyArgs) =>println!("Comanda trebuie sa aiba forma: cargo run -- in.png, out.txt"),
    //    Err(MyError::IO(x))=>eprint!("{:?}", x),
    //    Ok(()) => println!("Totul a fost executat cu succes")
    //    
    //}
    match ex3(Args::parse()) 
    {
        Err(MyError::EmptyArgs) =>println!("Comanda trebuie sa aiba forma: cargo run -- in.png, out.txt"),
        Err(MyError::IO(x))=>eprint!("{:?}", x),
        Ok(()) => println!("Totul a fost executat cu succes")

    }
}