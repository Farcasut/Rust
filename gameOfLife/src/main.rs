#![allow(non_snake_case)]
use core::time;
use std::{fs, thread::sleep};
const ROWS:i8=30;
const COLS:i8=50;

#[derive(Debug, Clone, Copy)]
struct Canvas
{
    suprafata:[[char;COLS as usize];ROWS as usize]
}

fn newCanvas()-> Canvas
{
    return Canvas{suprafata:[[' ' ;COLS  as usize];ROWS  as usize]};
}
fn initialState(state:&mut Canvas)
{ 
    let file=fs::read_to_string("initial.txt").expect("Eroare la deschidere");
    for (i,v) in file.split("\n").enumerate()
    {
        for (j, c) in v.chars().enumerate()
        {
            if c=='x'
            {
                state.suprafata[i][j]=c;
            }
        }
    }
}
fn print(currentState:&Canvas)
{
    for i in currentState.suprafata
    {
        for j in i
        {
            print!("{}", j);
        }
        println!();
    }
}
fn alive(state: &Canvas, i:i8, j:i8)->i8
{
    let mut ali:i8 = 0;
    let positions:[[i8;2];8]=[[-1, -1],[-1, 0], [-1,1],[0, -1], [0, 1], [1, -1], [1,0], [1, 1]];
    for pair in positions
    {
        let newLine:i8=i+pair[0];
        let newCol:i8=j+pair[1];
        if newCol<0 || newLine<0  || newCol==COLS || newLine==ROWS
            {
                continue;
            }
        if state.suprafata[newLine as usize][newCol as usize]=='x'
        {
            ali+=1;
        }
    }
   ali
}
fn startGameOfLife()
{
    let mut state1 =  newCanvas();
    let mut state2=newCanvas();
    let time=time::Duration::from_secs(1);
    initialState(&mut state1);
    #[allow(while_true)]
    while true
    {
    sleep(time);
    print(&state1);
    for i in 0..ROWS
    {
        for j in 0..COLS
        {
            let ali =  alive(&state1, i, j);
            if state1.suprafata[i as usize][j as usize]=='x' && (ali==2 || ali==3)
            {
                state2.suprafata[i as usize][j as usize]='x';
                continue;
            }
            if state1.suprafata[i as usize][j as usize]==' ' && ali==3
            {
                state2.suprafata[i as usize][j as usize]='x';
                continue;
            }
            state2.suprafata[i as usize][j as usize]=' ';
        }
    }
    state1=state2;
    print!("\x1B[2J\x1B[1;1H");
    }
}

fn main() {
    startGameOfLife();
}
