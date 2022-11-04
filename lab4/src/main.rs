use std::{fs, path};
#[derive(Debug)]
enum MyErr {
    isNotAnAsciiOrIsNumeric,
}

fn P1(path: String) -> (String, String) {
    let mut file = fs::read_to_string(path).expect("Path Gresit");
    let mut bytes: String = String::new();
    let mut longest: String = String::new();
    for v in file.split("\n") {
        if (bytes.len() < v.len()) {
            bytes = v.to_string();
        }
        if (longest.chars().count() < v.chars().count()) {
            longest = v.to_string();
        }
    }

    return (longest, bytes);
}
fn P2(data: String) -> Result<String, MyErr> {
    let mut result: String = String::new();
e
 
    for i in data.chars() {
        if i.is_ascii() {
            if i.is_uppercase() {
                result.push(((i as u8 - 'A' as u8 + 13) % 26 + 'A' as u8) as char);
            } else if i.is_lowercase() {
                result.push(((i as u8 - 'a' as u8 + 13) % 26 + 'a' as u8) as char);
            } else {
                result.push(i);
            }
        } else {
            return Err(MyErr::isNotAnAsciiOrIsNumeric);
        }
    }

    Ok(result)
}
fn P3(path: String)
{   
    let prescurtari = [("pt", "pentru"),("dl","domunl"),("ptr", "pentru"),("dna","doamna")];
    let mut file = fs::read_to_string(&path).expect("Path Gresit");
    for i in prescurtari
    {
        file=file.replace(i.0, i.1);
    }
        fs::write(path, &file).expect("Erroare la write");
}
fn P4()
{
    let file =  fs::read_to_string("/etc/hosts").expect("Erroare la open /etc/hosts");
    for line in file.lines() {
      if line.starts_with("#")
      {
        continue;
      }
      let mut result:String=String::new();
        for i in line.split_whitespace()
        {
            result.push_str(i);

        }
      

    }
}

fn main() {
     let a= P1("fisier".to_string());
    println!("{} {}\n",a.0, a.1);
    let b = P2("hello ".to_string()).expect("Err rot 13");
    println!("{b}");
    P3("prescurtari".to_string());

}
