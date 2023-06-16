fn main() {
    let mut x = 1;
    println!("Print x: {x}");
    x = 2;
    println!("Update x: {x}");
    let y = 3;
    println!("Print y: {y}");
    {
        let y = 4;
        println!("Print y in block: {y}");
    }
}
