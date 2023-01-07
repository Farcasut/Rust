

use std::{fs::File, io::{Write, stdout, Read}, default, ptr::read, convert::Infallible};

use subprocess::{Popen, PopenConfig, Redirection, Exec, PopenError};



fn main()
{ 
    let path= "D:\\steamcmd\\steamcmd.exe";
    let command = "quit";
    let mut file = File::create("INPUT.txt").unwrap();
    file.write(command.as_bytes());
    let mut child = Popen::create(&[path], PopenConfig{stdin:Redirection::File(file), stdout:Redirection::Pipe, ..Default::default()}).unwrap();
    let stdout =  child.stdout.as_mut().unwrap();
    let mut line:String=String::new();
    stdout.read_to_string(&mut line).unwrap();
    println!("{}",line);


    // let opt :Option<&str>= Some("quit");
    // let mut p= Popen::create(&[path], PopenConfig { stdin:Redirection::Pipe, ..Default::default() }).unwrap();
    // let mut stdin = p.stdin.as_mut().expect("ERROR");
    // stdin.write_all("quit".as_bytes()).unwrap();
}