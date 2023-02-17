

 struct archive
 {
     magic:u32,
     description: String,
     count :u32,
     files: Vec<archive_file>

 }

 struct archive_file
 {
     file_name:String,
     buffer:Vec<buffer>
 }
 struct buffer
 {
    length:u32,
    data:vec![],
 }
struct string_{
    length:u32,
    data  :String
}

trait ArchiveSerializer {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()>;
}


macro_rules! println_hex {
    ($num:expr) => {
        println!("{:x}", $num);
    };
}

fn main()
{
    println!("Hello, world!");
}
