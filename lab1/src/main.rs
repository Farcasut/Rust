fn primes(x :i32)->bool
{
    if x==1{return false;}
    if x==2 || x==3
        {return true;}
   for i in 2..(x/2)+1
   {
        if x%i==0 {
            return false;
        }
   }

    return true;
}


fn coprime(x:i32, y:i32)->bool
{
    let maxim:i32;
    if x>y
    {maxim=x}
    else
    {
        maxim=y;
    }
    for i in 2..maxim+1
    {
        if x%i==0 && y%i==0
        {
             return false;
        }
    }

    return true;
}
fn bottles()
{
    for i in (1..100).rev()
    {
        println!("{} bottles of beer on the wall,\n{} bottles of beer.", i,i);
        println!("Take one down, pass it aroung,");
        if i!=1
        {println!("{} bottles of beer on the wall", i-1);}
    }

        println!("No bottles of beer on the wall.");
        println!("No bottles of beer.\nGo to the store, buy some more,\n99 bottles of beer on the wall.\n");
}

fn main() {
    for i in 1..101
    {
        if primes(i)==true
        {
        println!("{i} este prim ");
        }
    }
    for i in 1..101
    {
        for j in 1..101
            {
                if coprime(i, j)
                {
                    println!("{} si {} sunt coprime", i,j);
                }
            }
    }
    bottles();
}
