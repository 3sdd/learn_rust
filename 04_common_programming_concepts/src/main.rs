use std::io;

fn main() {
    chapter3_1();
    chapter3_2();
    chapter3_2_array();
}

fn chapter3_2_array(){
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn chapter3_2() {
    let sum = 5 + 10;
    println!("sum:{sum}");

    let difference = 95.5 - 4.3;
    println!("diff:{difference}");

    let product = 4 * 30;
    println!("product:{product}");

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("quotient:{quotient}");
    println!("truncated:{truncated}");

    let remainder = 53 % 5;
    println!("remainder:{remainder}");

    let t = true;
    let f: bool = false;
    println!("t:{t}");
    println!("f:{f}");

    // char   :  single literals
    let c = 'z';
    let z:char ='Z';
    let heart_eyed_cat ='😻';

    println!("{c},{z},{heart_eyed_cat}");

    // tuple
    let tup: (i32,f64,u8) = (500,6.4,1);
    let (x,y,z)=tup;
    println!("The value of y is {y}");

    let five_hundred=tup.0;
    let six_point_four=tup.1;
    let one=tup.2;

    println!("{five_hundred},{six_point_four},{one}");

    let a=[1,2,3,4,5];

    let months=["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let a:[i32;5]=[1,2,3,4,5];

    // let a = [3, 3, 3, 3, 3];と同じ
    let a= [ 3;5];

    let a=[1,2,3,4,5];
    let first=a[0];
    let second=a[1];
    println!("{first},{second}");
}


fn chapter3_1() {
    // let mut x = 5;
    // println!("The value of x is: {x}");

    // x = 6;
    // println!("The value of x is: {x}");

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing

    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // string type
    let spaces = "   ";
    // number type
    let spaces = spaces.len();

    // error
    // let mut spaces = "  ";
    // spaces = spaces.len();
}
