fn testfunction(x: i32) -> i32 {
    x+10 
}

#[warn(dead_code)]
#[warn(unused_variables)]
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
    let slice: &mut [i32] = &mut v[1..3];

    let x: (i32,&str,f32) = (5,"moi", 3.14);

    //let (a,b,c) = x;
    println!("slice: {:?} and x[0]: {}, x[1]: {}", slice, x.0, x.1);

    struct P {
        x: i32,
        y: i32,
    }
    let p1: P = P {x: 5, y: 6};
    println!("p1.x: {}, p1.y: {}", p1.x, p1.y);

    struct P2 (String, i32);

    let _p2: P2 = P2("moi".to_string(), 5);

    enum E {
        Left, Right, Up, Down
    }

    enum Optional<T> {
        yesval(T),
        noval,
    }

    struct Foo<T> {
        bar: T,
    }

    impl<T> Foo<T> {
        fn borrow_without_mutate(&self) -> &T {
            &self.bar
        }
        fn borrow_with_mutate(&mut self) -> &mut T {
            &mut self.bar
        }
        fn just_take_ownership(self) -> T {
            self.bar
        }
    }

    let mut foo: Foo<i32> = Foo {bar: 5};
    println!("foo: {}", foo.borrow_without_mutate());
    *foo.borrow_with_mutate() = 6;

    fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 1,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    println!("fibonacci(10): {}", fibonacci(10));

    type POINTER = fn(u32) -> u32;
    let fib: POINTER = fibonacci;

    println!("fib(10): {}", fib(10));

    let array = [1,2,3,4,5];
    for i in array {
        println!("{}", i);
    }

    for i in 0..5 {
        println!("{}", array[i]);
    }

    let val = if true {
        5
    } else {
        6
    };
    println!("val: {}", val);

    // `while` loop
    while 1 == 1 {
        println!("The universe is operating normally.");
        // break statement gets out of the while loop.
        //  It avoids useless iterations.
        break
    }

    // Infinite loop
    loop {
        println!("Hello!");
        // break statement gets out of the loop
        break
    }

    let mut mine: Box<i32> = Box::new(5);
    *mine = 6;
    println!("mine: {}", mine);
    let mut now_its_mine = mine;
    *now_its_mine = 7;
    println!("now_its_mine: {}", now_its_mine);
    //println!("{}", mine);

    let mut var = 4;
    var = 5;
    let ref_val: &i32 = &var;
    println!("var: {}, ref_val: {}", var, ref_val);
   
    let mut var2 = 5;
    let ref_val2: &mut i32 = &mut var2;
    *ref_val2 = 6;
    println!("ref_val2: {}", ref_val2);
}
