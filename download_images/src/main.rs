use std::fs;
use std::fs::File;
use std::io::copy;
use std::io::Write;


fn main() {
    let mut data=ureq::get("https://i.redd.it/gs2ww0c9zlz91.jpg");
    let mut out = File::create("ureq1.jpg").expect("failed to create file");
    std::io::copy(&mut data., &mut out);
    //let data=ureq::get(path).call().expect("msg").into_string().expect("2");

}