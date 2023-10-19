fn main() {
    let mut v = String::from("");
    let mut t:String=String::new();
    let mut i=0;
    while i < 26 
    {
        let c = (i as u8 + 'a' as u8) as char;
        v = add_chars_n(v, c, 26 - i);
        p2(&mut t,c,26-i);

        i += 1;
    }
    print!("{}\n", v);
    print!("{}\n",t);
    let s:&mut String=&mut String::new();
    add_space(s, 40);
    add_str(s,  "I 💚");
    add_str(s,  "\n");
    add_space(s, 40);
    add_str(s,  "RUST.");
    add_str(s,  "\n");
    add_space(s, 4);
    add_str(s,  "Most");
    add_space(s, 12);
    add_str(s,  "crate");
    add_space(s, 6);
    add_integer(s, 306437986);
    add_space(s, 11);
    add_str(s,  "and");
    add_space(s, 5);
    add_str(s,  "lastest");
    add_space(s, 9);
    add_str(s,  "is");
    add_str(s,  "\n");
    add_space(s, 9);
    add_str(s,  "downloaded");
    add_space(s, 8);
    add_str(s,  "has");
    add_space(s, 13);
    add_str(s,  "downloads");
    add_space(s, 5);
    add_str(s,  "the");
    add_space(s, 9);
    add_str(s,  "version");
    add_space(s, 4);
    add_float(s, 2.038);
    add_str(s, ".");
    print!("{}",s);
}

fn add_chars_n(mut s:String,c:char,n:u32) -> String
{
    let mut ls:String=String::new();
    for _ in 0..n
    {
        ls.push(c);
    }
    s.push_str(&ls);
    return s;
}

fn p2(s:&mut String,c:char,n:u32)
{
    let mut ls:String=String::new();
    for _ in 0..n
    {
        ls.push(c);
    }
    s.push_str(&ls);
}

fn add_space(s:&mut String, n:u32) 
{
    for _ in 0..n
    {
        s.push(' ');
    }
}

fn add_str( s:&mut String, to_concate:&str)
{
    s.push_str(&to_concate);
}

fn add_integer(s:&mut String,n:u32)
{
    let mut aux:u32;
    aux=n;
    let mut p:u32;
    let mut c:char;
    let mut nr_cifre:u8=0;
    p=1;
    while aux>9
    {
        p*=10;
        aux/=10;
        nr_cifre=nr_cifre+1;
    }
    aux=n;
    let mut ok:u8=0;
    if nr_cifre%3==2
    {
        ok=0;
    }
    if nr_cifre%3==1
    {
        ok=2;
    }
    if nr_cifre%3==0
    {
        ok=1;
    }
    for i in 0..nr_cifre+1
    {
        c=(n/p%10 as u32+ '0' as u32)as u8 as char;
        if i%3==ok&&i!=0
        {
            s.push('_');
        }
        s.push(c);
        aux/=10;
        p/=10;
    }
}
fn add_float(s: &mut String,n:f32)
{
    s.push_str(&n.to_string());
}