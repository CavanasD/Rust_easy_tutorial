fn main(){
    //creation
    let a = 10;
    let a1: i16 = 1;
    //mutability
    let mut b = 10;
    b = 100;

    //shadowing
    let c = 10;
    let c = 20;

    println!("{c}");

    //scope
    let d = 30;
    {
        let d = 40;
        println!("{d}");

    }
    {
        let e = 100;
    }
    println!("{e}");
    println!("{d}");



}