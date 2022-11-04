use core::{panic, num::{dec2flt::number, self}};
use std::result;

//enum Value
//{
//    Integer(i32),
//    Float(f32),
//    String(String),
//    nul
//}
//
//fn f() -> Option<u32>
//{
//    Some(5)
//}
//
//fn add()->Option<u32>
//{
//    let x = f()?;
//    let y = f()?;
//    Some(x+y)
//
//
//}
//enum Color
//{
//    Red, Blue
//}
//
//#[derive(Debug)]
//enum MyError
//{
//    DivByZro
//}
//
//fn div(x:u32, y:u32)->Result<u32, MyError>
//{
//    if y==0{
//        Err(MyError::DivByZro)
//    }else {
//        Ok(x/y)
//    }
//
//}
//   In main
//let v = Value::Integer(10);
//let x=Value::Float(10.0);
//
//match v
//{
//    Value::Integer(x)=>print!("1"),
//    Value::Float(x)=>print!("1"),
//    _ => print!("2")
//}
//
//fn next_prime(x: u16) -> Option<u16>
//{
//    let max = 65535;
//    if x>max{
//            None
//        }
//    else {
//        
//
//        for n in x+1..max
//        {
//            let mut div:u32=0;
//            for i in 2..n/2
//            {
//                if n%i==0
//                {
//                    div=div+1;
//                }
//            } 
//            if div==0
//            {
//                return Some(n);
//            }
//        }
//
//        None
//        }
//  
//
//}
//
//fn adition(x:u32, y:u32)-> u32
//{   
//    let result:u64 = x as u64 + y as u64;
//
//    if result  > (std::u32::MAX) as u64
//    {
//        panic!("The number is too big\n");
//    }
//
//    return result as u32;
//}
//fn multmultiplication(x:u32, y:u32) ->u32
//{
//    let result:u64 =  (x as u64*y as u64);
//    if result > std::u32::MAX as u64{
//        panic!("The number is too big\n");
//    }
//
//    return result as u32;
//}
//
//
//
//
fn next_prime(x: u16) -> Option<u16>
{
    let max = 65535;
    if x>max{
            None
        }
    else {
        

        for n in x+1..max
        {
            let mut div:u32=0;
            for i in 2..n/2
            {
                if n%i==0
                {
                    div=div+1;
                }
            } 
            if div==0
            {
                return Some(n);
            }
        }

        None
        }
  

}
#[derive(Debug)]
enum MyErr
{
    TheNumberIsTooBig,
    CharIsNotAscii,
    CharIsNotDigit,
    CharIsNotBase16,
    CharIsNotLetter,
    CharIsNotPrintable
}
fn adition(x:u32, y:u32)-> Result<u32, MyErr>
{   
    let result:u64 = x as u64 + y as u64;

    if result  > (std::u32::MAX) as u64
    {
       return Err(MyErr::TheNumberIsTooBig);
    }

    return  Ok((result as u32));
}
fn multmultiplication(x:u32, y:u32) ->Result<u32, MyErr>
{
    let result:u64 =  (x as u64*y as u64);
    if result > std::u32::MAX as u64{
       return  Err(MyErr::TheNumberIsTooBig);
    }

    return Ok(result as u32);
}


fn to_uppercase(x:char) -> Result<char, MyErr>
{   
    let asciiValue:u8=x as u8;
    if 65<=asciiValue && asciiValue<=90 || 97<=asciiValue && asciiValue<=122
    {
        if 65<=asciiValue && asciiValue<=90
        {
            return Ok(asciiValue as char);
        }
        return Ok((asciiValue-32) as char);
    }
    else {
        return Err(MyErr::CharIsNotLetter);
        
    }
}

fn to_lower(x:char) -> Result<char, MyErr>
{   
    let asciiValue:u8=x as u8;
    if 65<=asciiValue && asciiValue<=90 || 97<=asciiValue && asciiValue<=122
    {
        if 65<=asciiValue && asciiValue<=90
        {
            return Ok((asciiValue+32) as char);
            
        }
        return Ok(asciiValue as char);
    }
    else {
        return Err(MyErr::CharIsNotLetter);
        
    }
}

fn main() 
{
//    
//for i in 0..65535
//{
//   let mut x= next_prime(i);
//   print!("Urmatorul prim este {:?}\n", x);
//}
let mut number:u16=0;
while let Some(x)=next_prime(number) {
    print!("{}",x);
    number=x;
}
 
let x=to_lower('A');
   match x{
    Ok(v)=>print!("{}", v),
    Err(v)=>print!("{:?}",v),

   }
   
}
