use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arg{
    #[arg(short, long)]
    count: i32,
}

fn main(){
    let arg: Arg = Arg::parse();
    let i: i32 = factorial(arg.count);

    println!("{}", i);
}

fn factorial(n: i32) -> i32 {
    if n > 0 {
        n * factorial(n - 1)
    } else {
        1
    }
}
