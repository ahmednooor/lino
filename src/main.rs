mod lino;
use lino::Lino;
use std;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut lino: Lino;
    
    if args.len() < 2 {
        lino = Lino::new();
    } else {
        lino = Lino::from_file(&args[1].to_string());
    }
    
    lino.run().unwrap();
}
