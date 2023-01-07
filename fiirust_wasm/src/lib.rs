
use wasm_bindgen::prelude::*;
const alfabet:&[u8]=b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[wasm_bindgen]
pub fn base64_encode(test: &[u8]) -> String {

    let mut answer= String::new();
    let mut encode:u32;
    let bitmask:u32=0b111111;
    let size = test.len()%3;
    for i in (0..test.len()-size).step_by(3)
    {
        let a:u32=test[i] as u32;
        let b:u32=test[i+1] as u32;
        let c:u32=test[i+2] as u32;
        encode= (a<<16) | (b<<8) | (c<<0);
        
        let c1:usize=(encode  & bitmask) as usize;
        let c2:usize=((encode>>6)  & bitmask) as usize;
        let c3:usize=((encode>>12) & bitmask) as usize;
        let c4:usize=((encode>>18) & bitmask) as usize;
        answer.push(alfabet[c4] as char);
        answer.push(alfabet[c3] as char);
        answer.push(alfabet[c2] as char);
        answer.push(alfabet[c1] as char);
    }
    if size==2
    {
        let a:u32=test[test.len()-2] as u32;
        let b:u32=test[test.len()-1] as u32;
        encode= (a<<16) | (b<<8);
        let c2:usize=((encode>>6)  & bitmask) as usize;
        let c3:usize=((encode>>12) & bitmask) as usize;
        let c4:usize=((encode>>18) & bitmask) as usize;
        answer.push(alfabet[c4] as char);
        answer.push(alfabet[c3] as char);
        answer.push(alfabet[c2] as char);
        answer.push('=');
    }
    if size==1
    {
        let a:u32=test[test.len()-1] as u32;
        encode= (a<<16);
        let c3:usize=((encode>>12) & bitmask) as usize;
        let c4:usize=((encode>>18) & bitmask) as usize;
        answer.push(alfabet[c4] as char);
        answer.push(alfabet[c3] as char);
        answer.push('=');
        answer.push('=');
    }



    answer.to_string()
}
