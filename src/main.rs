mod lino;
use lino::Lino;
use std;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("My path is {}.", args[0]);
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    println!("{:?}", std::env::current_dir());
    println!("{:?}", std::env::current_dir().unwrap().to_str().unwrap().to_string());

    let mut lino: Lino;
    
    if args.len() < 2 {
        lino = Lino::new();
    } else {
        lino = Lino::from_file(&args[1].to_string());
    }
    
    lino.run().unwrap();
}
