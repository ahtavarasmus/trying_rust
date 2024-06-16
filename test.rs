fn testfunction(x: i32) -> i32 {
    x+10 
}

fn main() {
    let mut x: i32 = 23;
    println!("{}", testfunction(x));
    x = 12;
    println!("{}", testfunction(x));

    let s: &str = "hello, world!";
    println!("{}", s);

    let p: String = "hellow, world!".to_string();
    println!("{}", p);

    let l: [i32; 5] = [1,2,3,4,5];
    for x in l.iter() {
        print!("{}, ", x);
    }
    println!("");
    let mut v: Vec<i32> = vec![5,4,3,2,1];
    for x in &mut v {
        *x *= 4;
    }
    println!("{:?}", v);

}
