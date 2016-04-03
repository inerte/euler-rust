mod p1;
mod p2;

fn main() {
    println!("p1 {}", p1::run(1000));
    println!("p2 {}", p2::run(4000000));
}
