use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::time::{Duration, Instant};
use num::complex::Complex;
use regex::Regex;
use std::io::prelude::*;

fn main() {
    println!("Listing 2.4");
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);

    // example 1 : compare float
    // let result: f32 = 0.1 + 0.2;
    // let desired: f32 = 0.2;
    // let absolute_difference = (desired - result).abs();
    // assert!(absolute_difference <= f32::EPSILON);

    // almost all operations interacting with `NAN` return `NAN`
    // by definition, `NAN` values are never equal
    // this example will always crash
    // let x = (-42.0_f32).sqrt();
    // assert_eq!(x, x);

    // To program defensively, make use of the is_nan() and is_finite() methods.
    // Inducing a crash, rather than silently proceeding with a mathematical error,
    // allows you to debug close to what has caused the problem.

    // example 2 : `.is_finite()`
    // let x: f32 = 1.0 / 0.0;
    // assert!(x.is_finite());

    // example 3 : complex number
    let a = Complex { re: 2.1, im: -1.2};                   // literal syntax
    let b = Complex::new(11.1, 22.2);  // static method
    let result = a + b;
    println!("{} + {}i", result.re, result.im);

    // If you need to modify each item during the loop,
    // you can use a mutable reference by including the mut keyword:
    //
    // for item in &mut collection {
    //     // ...
    // }

    // for loop (compare)
    // 1. (Ownership) for item in collection
    // - for item in `IntoIterator::into_iter(collection)`
    // 2. (Read-Only) for item in &collection
    // - for item in `collection.iter()`
    // 3. (Read-Write) for item in &mut collection
    // - for item in `collection.iter_mut()`

    // inclusive syntax : n..m
    // exclusive syntax : n..=m

    println!("Listing 2.7 : Testing how fast your computer can increment a counter");
    let mut count = 0;
    let time_limit = Duration::new(1,0);
    let start = Instant::now();
    while (Instant::now() - start) < time_limit{
        count += 1;
    }
    println!("{count}");


    // example : break from nested loops
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 1000 {
                    break 'outer
                }
            }
        }
    }
    println!("break nested loops");

    // example : break with value
    let n = loop {
        break 123;
    };
    println!("{}", n);

    // a reference is a value that stands in place for another value

    println!("Listing 2.10 Creating a reference to a large array");
    let a = 42;
    let r = &a;
    let b = a + *r;
    println!("a + a = {}", b);

    // references are created with the reference operator `&` and
    // dereferencing occurs with the dereference operator `*`

    println!("Listing 2.11 Searching for an integer in an array of integers");
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 15, 42, 132, 203, 877, 4140, 21147];

    for item in &haystack {
        println!("{:p}", item);
        if *item == needle {
            println!("{}", item);
        }
    }

    // fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    //     *i + *j
    // }
    //
    // fn add_with_lifetimes(...) -> i32
    // should be familiar to you already.
    // From this we can infer that add_with_lifetimes() is a function that returns an i32 value.
    //
    // <'a, 'b> declares two lifetime variables, 'a and 'b, within the scope of add_with_lifetimes().
    // These are normally spoken as lifetime a and lifetime b.
    //
    // i: &'a i32 binds lifetime variable 'a to the lifetime of i.
    // The syntax reads as “parameter i is a reference to an i32 with lifetime a.”
    //
    // j: &'b i32 binds the lifetime variable 'b to the lifetime of j.
    // The syntax reads as “parameter j is a reference to an i32 with lifetime b.”

    // omitting lifetime annotations is formally referred to as lifetime elision

    // all of Rust's operations are defined with traits
    // all of Rust's operators are syntactic sugar for a trait's method
    // `a + b` is converted to `a.add(b)`

    // a few principles that should assist you when reading Rust code
    // - terms in lowercase (i, j) denote variables
    // - single uppercase letters (T) denote generic type variables
    // - Terms beginning with uppercase (Add) are either traits or concrete types, such as String or Duration
    // - Labels ('a) denote lifetime parameters

    // `str` is a high-performance, relatively feature-poor type.
    // Once created, `str` values cannot expand or shrink

    // String uses dynamic memory allocation to store the text that it represents.
    // Creating &str values avoids a memory allocation.
    //
    // `String` is an owned type : read-write
    // `&str` is a borrowed type : read-only

    // - char - A single character encoded as 4 bytes
    // - [u8] - A slice of raw bytes, usually found when dealing with streams of binary data
    // - Vec<u8> - A vector of raw bytes, usually created when consuming [u8] data.
    //             String is to Vec<u8> as str is to [u8]

    // creating arrays
    // - [1,2,3]
    // - [0;100] : means repeat 0 for 100 times

    // In practice, most interaction with arrays occurs via another type called a slice `[T]`
    // the slice is  itself interacted with by reference `&[T]`

    // slices are dynamically sized array-like objects
    // the term `dynamically` means that their size is not known at compile time
    //
    // the use of the word `dynamic` in dynamically sized is closer in meaning to dynamic typing rather than movement
    //
    // slices are important because it's easier to implement traits for slices than arrays
    // creating a slice from an array is easy and cheap because it doesn't need to be tied to any specific size

    // Vectors (Vec<T>) are growable lists of `T`

    // Vec<T> performs best when you can provide it with a size hint via `Vec::with_capacity()`

    // cargo add regex@1
    // cargo build
    // cargo doc
    // cargo doc --open
    // rustup doc

    // Reading from files
    // the general pattern is to open a `File` object, then wrap that in a `BufReader`
    // `BufReader` takes care of providing buffered I/O, which can reduce system calls to the OS if the hard disk is congested

    println!("Listing 2.14 Type signature of a function with lifetime explicit annotations");
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b);
    println!("{}", res);

    println!("Listing 2.17 A generic function with a type variable and trait bounds");
    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    let durations = add(
        Duration::new(5,0),
        Duration::new(10,0),
    );
    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);

    println!("Listing 2.18 Searching for a simple pattern within lines of a string");
    let search_term = "picture";
    let quote = "\
 Every face, every shop, bedroom window, public-house, and
 dark square is a picture feverishly turned--in search of what?
 It is the same with books.
 What do we seek through millions of pages?";
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }

    println!("Listing 2.21 Defining arrays and iterating over their elements");
    let one = [1,2,3];
    let two: [u8; 3] = [1,2,3];
    let blank1  = [0;3];
    let blank2: [u8;3] = [0;3];

    let arrays = [one,  two, blank1, blank2];
    for a in &arrays {
        print!("{:?}", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n , n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
           sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }

    println!("Listing 2.22 Enabling context lines to be printed out with a Vec<Vec<T>>");
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
 Every face, every shop,
 bedroom window, public-house, and
 dark square is a picture
 feverishly turned--in search of what?
 It is the same with books.
 What do we seek
 through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(
        usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2*ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound =
                tag.saturating_sub(ctx_lines);
            let upper_bound =
                tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }

    println!("Listing 2.24 Searching for patterns with regular expressions");
    let re =  Regex::new("picture").unwrap();
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        let contains_substring = re.find(line);
        match contains_substring {

            Some(_) => println!("{}", line),
            None => (),
        }
    }

    println!("Listing 2.27 Reading a file manually line by line");
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len ==  0 {
            break
        }
        println!("{} ({} bytes long)", line, len);
        line.truncate(0);
    }

    println!("Listing 2.28 Reading a file line by  line via BufReader::lines()");
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
       let line = line.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}

fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}
