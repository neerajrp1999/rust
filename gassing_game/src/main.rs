use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let scr=rand::thread_rng().gen_range(0,100);
    loop{
        let mut guess=String::new();
        println!("Ennter guess no:");
        io::stdin().read_line(&mut guess);
        //let guess:u32=guess.trim().parse().expect("hgghg");
        let guess:u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        match guess.cmp(&scr){
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {println!("equal");break;},
        }
    }
}