use std::{fs::File, io::Read};



struct archive_file
{
    file_name:String,
    buffer:Vec<u8>
}
fn main()-> std::io::Result<()>  
{

    let mut file=File::open("1.archive")?;
    let mut magic_bytes:[u8;4]=[0;4];
    let mut descriptionLength:[u8;4]=[0;4];
    let mut bytes_count:[u8;4]=[0;4];
    file.read_exact(&mut magic_bytes)?;//MAGIC field
    file.read_exact(&mut descriptionLength)?;//Description length
    let mut description_utf8=vec![0 as u8; u32::from_ne_bytes(descriptionLength) as usize];
    file.read_exact(& mut description_utf8)?;
    let description = String::from_utf8(description_utf8).unwrap();//Description
    file.read_exact(&mut bytes_count)?;
    let count =  u32::from_ne_bytes(bytes_count);//Number of archive_file
    let mut list:Vec<archive_file>= Vec::new();
    file.read_exact(&mut bytes_count)?;//the length of the title
   
    let mut title=vec![0 as u8; u32::from_ne_bytes(bytes_count) as usize];
    file.read_exact(& mut title)?;
    let title_ = String::from_utf8(title).unwrap();//the title

    let mut bytes_count:[u8;4]=[0;4];//length of the buffer
    let mut bufferData=vec![0 as u8; u32::from_ne_bytes(bytes_count) as usize];


 



    return Ok(());

}
