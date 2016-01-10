use std::cmp;

fn main() {
    println!("Hello, world!");

    // variable bindings
    let x: u32 = 123;
    let y: u32 = 321;
    let mut z = 123;
    z = z + 1;
    println!("x={} y={} z={}", x, y, z);

    // function pointers
    let fp1: fn(u32,u32)->u32 = add; // no type inference
    let fp2 = add;                   // using type inference
    println!("{}", fp1(1,2));
    println!("{}", fp2(3,4));
    println!("{:?}", 123); // debug format

    // types
    // i8-i64
    // u8-u64
    // isize/usize
    // f32/f64
    // bool

    // arrays
    let a1 = [1,2,3,4,5,6,7,8,9,10];
    let a2 = [69; 5];
    let a3 = ["hello", "world", "123"];

    let s1 = &a1[2..4]; // slice
    let s2 = &a1[..];

    println!("{:?}", a1);
    println!("{:?}", a2);
    println!("{:?}", a3.len());
    println!("{:?}", a3[1]);
    println!("{:?}", s1);
    println!("{:?}", s2);

    // strings
    let str1 = "hello world"; // & static str
    let str2: String = str1.to_string(); // String

    println!("{:?}", str1);
    println!("{:?}", str2);
    println!("{:?}", str1.len());

    // tuples
    let t1: (i32,i32) = (111,222);
    let mut t2: (i32,i32) = (333,444);
    t2 = t1;
    let t3: (f32, String) = ( 3.141, "PI".to_string());
    println!("{:?}", t1);
    println!("{:?}", t2);
    println!("{:?}", t3);
    println!("{:?}", t3.1);

    // if
    let x: i32 = 7;
    let y: i32 = if x==5 || x==6 { 111 } else { 222 };
    println!("{:?}", y);

    // loop
    let mut i: u32 = 5;
    loop {
        i -= 1;
        println!("i={}", i);
        if i == 0 { break }
    }
    i = 5;
    while i != 0 {
        println!("i={}", i);
        i -= 1;
    }
    for i in 0..5 {
        println!("i={}", i);
    }
    for (i,j) in (5..11).enumerate() {
        println!("i={} j={}", i, j);
    }
    'outer: for x in 0..4 {
        'inner: for y in 0..4 {
            if x % 2 == 0 { continue 'outer; } // continues the loop over x
            if y % 2 == 0 { continue 'inner; } // continues the loop over y
            println!("x: {}, y: {}", x, y);
        }
    }

    // lifetime
    //let x;                    // -+ x goes into scope
                              //  |
    {                         //  |
        let y = &5;           // ---+ y goes into scope
        let f = Foo { x: y }; // ---+ f goes into scope
        //x = &f.x;             //  | | error here
    }                         // ---+ f and y go out of scope
                              //  |
    //println!("{}", x);        //  |
                              // -+ x goes out of scope

    static FOO: i32 = 5;        // FOO stored in datasection of binary
    let x: &'static i32 = &FOO; // reference to data section of binary
    let str1: &'static str = "qwiojdqoiwjd";

}

struct Foo<'a> {
    x: &'a i32,
}

fn add(a:u32, b:u32) -> u32 {
    a+b
}

// diverging function, must not return
// RUST_BACKTRACE=1 for trace
//fn doPanic(a:u32) -> ! {
//    panic!("Something went wrong!");
//}
