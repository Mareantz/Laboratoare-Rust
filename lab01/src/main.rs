fn main() {
    p3();
}

fn prim(nr:u32)
{
    if nr==1||nr==0
    {
        println!("Numarul {nr} nu este prim");
        return;
    }
    if nr%2==0&&nr!=2
    {
        println!("Numarul {nr} nu este prim");
        return;
    }
    let mut i:u32=3;
    while i*i<=nr
    {
        if nr%i==0
        {
            println!("Numarul {nr} nu este prim");
            return;
        }
        i+=2;
    }
    println!("Numarul {nr} este prim");
    return;
}

fn p1()
{
    let mut i=0;
    while i<=100
    {
        prim(i);
        i+=1;
    }
}

fn coprime(mut a:u32,mut b:u32)
{
    let mut c=a%b;
    let nr1=a;
    let nr2=b;
    while c!=0
    {
        a=b;
        b=c;
        c=a%b;
    }
    if c==1
    {
        println!("Numerele {nr1} si {nr2} sunt coprime");
        return;
    }
    else 
    {
        println!("Numerele {nr1} si {nr2} nu sunt coprime");
        return;
    }
}

fn p2()
{
    let mut i=0;
    let mut j;
    while i<=100
    {
        j=i+1;
        while j<=100
        {
            coprime(i,j);
            j+=1;
        }
        i+=1;
    }
    return;
}

fn p3()
{
    let mut beer=99;
    while beer>1
    {
        println!("{beer} bottles of beer on the wall,");
        println!("{beer} bottles of beer.");
        println!("Take one down, pass it around,");
        println!("{} bottles of beer on the wall.",beer-1);
        println!("");
        beer-=1;
    }
    println!("{beer} bottles of beer on the wall,");
    println!("{beer} bottles of beer.");
    println!("Take one down, pass it around,");
    println!("No bottles of beer on the wall.");
    println!("");

    println!("No bottles of beer on the wall,");
    println!("No bottles of beer.");
    println!("No bottles of beer,");
    println!("99 bottles of beer on the wall.");
    return;
}