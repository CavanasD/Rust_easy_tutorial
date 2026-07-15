fn main(){
    // unsigned integers
    let a: u8 = 1;
    let a1: u16 = 1;
    let a2: u32 = 1;
    let a3: u64 = 1;
    let a4: u128 = 1;
    // signed integers
    let b1: u8 = 1;
    let b2: u16 = 1;
    let b3: u32 = 1;
    let b4: u64 = 1;
    let b5: u128 = 1;
    // float numbers
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;
    // platform specific integers
    let d1: usize = 1; //The pointer-sized unsigned integer type
    let d2: isize = 1;
    // char &str String
    let c1: char = 'a';
    let s2: &str = "Hello"; //String slices
    let s3: String = String::from("World");
    // Scalar Data Types↑
    // Compound Data Types ↓
    let a1: [_; 5] = [10, 20, 30, 40, 50];
    let a1 = [1, 2, 3];
    let a2: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    let i1 = a2[4];
    // tuples (元组)
    let t1 = (2, 2.0, "helo");
    let s1 = t1.2;
    let (i1, f1, s1) = t1;
    // 这个有点难理解，看一下下面的例子
    fn divide(a: i32, b: i32) -> (i32, i32) {
        let quotient = a / b;
        let remainder = a % b;

        (quotient, remainder)
    }
    fn miaowu() {
        let result = divide(10, 3);

        println!("商：{}", result.0);
        println!("余数：{}", result.1);
    }

    //empty tuple
    let unit = ();

    // Type aliasing
    type age = u8;
    let a1:age = 1;
    
}