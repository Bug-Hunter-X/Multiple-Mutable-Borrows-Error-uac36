fn main() {
    let mut x = 5;
    { //Using a block to limit scope of the mutable reference 
        let y = &mut x;
        *y += 1;
    }
    { //Using another block for the second mutable reference
        let z = &mut x;
        *z += 2;
    }
    println!("{}", x);
}
//Alternative solution using cloning
fn main(){
    let mut x = 5;
    let mut y = x.clone();
    let mut z = x.clone();
    y += 1;
    z += 2;
    x = y + z;
    println!("{}", x);
}