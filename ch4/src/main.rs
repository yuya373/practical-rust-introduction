use std::collections::HashMap;

fn main() {
    assert_eq!(std::mem::size_of::<()>(), 0);
    let mut h: HashMap<&str, ()> = HashMap::new();
    h.insert("size of", ());
    h.insert("value", ());
    h.insert("is", ());
    h.insert("zero.", ());
    println!("{:?}", h);

    // overflow!
    let n1 = std::u8::MAX;
    let n2 = 1u8;
    // panic! in debug mode
    // let n3 = n1 + n2;
    // println!("{}", n3);

    let n1 = 200u8;
    let n2 = 3u8;

    assert_eq!(std::u8::MAX, 255);
    assert_eq!(n1.checked_mul(n2), None);
    assert_eq!(n1.saturating_mul(n2), std::u8::MAX);
    assert_eq!(n1.wrapping_mul(n2), 88);
    assert_eq!(n1.overflowing_mul(n2), (88, true));

    let c1 = 'A';
    let c2 = 'a';
    assert!(c1 < c2);
    assert!(c1.is_uppercase());

    let c3 = '0';
    assert!(c3.is_digit(10));

    assert_eq!(std::mem::size_of::<char>(), 4);

    fn f1(mut n: u32) {
        n = 1;
        println!("f1: n = {}", n);
    }

    fn f2(n_ptr: &mut u32) {
        println!("f2: n_ptr = {:p}", n_ptr);

        *n_ptr = 2;
        println!("f2: *n_ptr = {}", *n_ptr);
    }

    let mut n = 0;
    println!("main: n = {}", n);
    f1(n);
    println!("main: n = {}", n);
    f2(&mut n);
    println!("main: n = {}", n);

    let c1 = 'A';
    let c1_ptr = &c1;
    assert_eq!(*c1_ptr, 'A');

    let mut n1 = 0;
    let n1_ptr = &mut n1;
    assert_eq!(*n1_ptr, 0);

    *n1_ptr = 1_000;
    assert_eq!(*n1_ptr, 1000);

    let c1 = 'A';
    let c1_ptr: *const char = &c1;
    assert_eq!(unsafe { *c1_ptr }, 'A');

    let mut n1 = 0;
    let n1_ptr: *mut i32 = &mut n1;
    assert_eq!(unsafe { *n1_ptr }, 0);

    unsafe {
        *n1_ptr = 1_000;
        assert_eq!(*n1_ptr, 1_000);
    }

    fn double(n: i32) -> i32 {
        n + n
    }

    fn abs(n: i32) -> i32 {
        if n >= 0 {
            n
        } else {
            -n
        }
    }

    let mut f: fn(i32) -> i32 = double;
    assert_eq!(f(-42), -84);

    f = abs;
    assert_eq!(f(-42), 42);

    let mut f: fn(i32) -> i32 = double;
    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());

    let mut f_bad = double;
    // f_bad = abs;
    // error[E0308]: mismatched types
    //    --> src/main.rs:103:13
    //     |
    // 103 |     f_bad = abs;
    //     |             ^^^ expected fn item, found a different fn item
    //     |
    //     = note: expected type `fn(i32) -> i32 {main::double}`
    //                found type `fn(i32) -> i32 {main::abs}`
    assert_eq!(std::mem::size_of_val(&f_bad), 0);

    let x = 4;
    let adder = |n| n + x;
    assert_eq!(adder(2), 4 + 2);
    // struct adder_env {
    //     x_ref: &i32,
    // }

    let mut state = false;
    let mut flipflop = || {
        state = !state;
        state
    };
    assert!(flipflop());
    assert!(!flipflop());
    assert!(flipflop());
    assert!(state);

    let mut f: fn(i32) -> i32 = |n| n * 3;
    assert_eq!(f(-42), -126);

    let x = 4;
    // f = |n| n * x;

    let v = vec!["I", "love", "Rust!"]
        .into_iter()
        .map(|s| s.len())
        .collect::<Vec<_>>();
    assert_eq!(v, vec![1, 4, 5]);

    let t1 = (88, true);
    assert_eq!(t1.0, 88);
    assert_eq!(t1.1, true);

    let mut t1 = (88, true);
    t1.0 += 100;
    assert_eq!(t1.0, 188);

    let (n1, b1) = (88, true);
    assert_eq!(n1, 88);
    assert_eq!(b1, true);

    let ((x1, y1), (x2, y2)) = ((0, 5), (10, -1));
    assert_eq!(x1, 0);
    assert_eq!(y1, 5);
    assert_eq!(x2, 10);
    assert_eq!(y2, -1);

    let mut t1 = ((0, 5), (10, -1));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t1;
    *x1_ptr += 3;
    *y1_ptr *= -1;
    assert_eq!(t1, ((3, -5), (10, -1)));

    let a = [0, 1];
    // compile error
    // error: index out of bounds: the len is 2 but the index is 3
    // a[3];
    // runtime error
    // thread 'main' panicked at 'index out of bounds: the len is 2 but the index is 2', src/main.rs:172:5
    // let i = 2;
    // a[i];

    assert_eq!(a.get(1), Some(&1));
    assert_eq!(a.get(2), None);

    let a = ['a'; 50];
    for ch in a.iter() {
        print!("{}, ", ch);
    }
    println!();

    let mut a = [1; 50];
    for n in a.iter_mut() {
        *n *= 2;
    }
    for n in a.iter() {
        print!("{}, ", n)
    }
    println!();

    fn print_info(name: &str, sl: &[char]) {
        println!(
            "{:9} - {}, {:?}, {:?}, {:?}",
            name,
            sl.len(),
            sl.first(),
            sl[1],
            sl.last()
        );
    }

    let a = ['a', 'b', 'c', 'd'];
    println!("a: {:?}", a);
    print_info("&a[..]", &a[..]);
    print_info("&a", &a);
    print_info("&a[1..3]", &a[1..3]);

    let v = vec!['e', 'f', 'g', 'h'];
    println!("v: {:?}", v);
    print_info("&v[..]", &v[..]);
    print_info("&v", &v);
    print_info("&v[1..3]", &v[1..3]);

    let mut a = [5, 4, 3, 2];
    let s = &mut a[1..3];
    s[0] = 6;
    s[1] *= 10;
    s.swap(0, 1);
    assert_eq!(s, [30, 6]);

    let s1: &'static str = "abc1";
    let s2: &'static str = "abc2";
    assert!(s1 < s2);
    assert!(s1 != s2);
    let s3 = "aaa
    aaa";
    assert_eq!(s3, "aaa\n    aaa");
    let s4 = "aaa\
              aaa";
    assert_eq!(s4, "aaaaaa");

    let s5 = "aaa\\bbb";
    let s6 = r#"aaa\bbb"#;
    assert_eq!(s5, s6);

    let s7 = r###"aaa ## bbb"###;
    assert_eq!(s7, "aaa ## bbb");

    let s1 = "a";
    let s2 = "あ";
    let s3 = ":bowtie:";
    let s4 = ":flag_td:";
    assert_eq!(s1.len(), 1);
    assert_eq!(s2.len(), 3);
    assert_eq!(s3.len(), 8);
    assert_eq!(s4.len(), 9);

    let s = "abcあいう";
    assert_eq!(s.get(0..1), Some("a"));
    assert_eq!(s.get(3..6), Some("あ"));
    assert_eq!(s.get(3..4), None);

    let s = "かか\u{3099}く";
    println!("{}", s);

    for ch in s.chars() {
        println!("{}", ch);
    }

    let utf8: [u8; 4] = [0x61, 0xe3, 0x81, 0x82];
    assert_eq!(std::str::from_utf8(&utf8), Ok("aあ"));

    let bad_utf8: [u8; 2] = [0x81, 0x33];
    let result2 = std::str::from_utf8(&bad_utf8);
    assert!(result2.is_err());
    println!("{:?}", result2);

    let mut s1: String = "abcあいう".to_string();
    let s2: &mut str = s1.as_mut_str();

    s2.make_ascii_uppercase();
    assert_eq!(s2, "ABCあいう");

    let b = unsafe { s2.as_bytes_mut() };
    b[3] = b'*';
    b[4] = b'a';
    b[5] = b'*';
    assert_eq!(s1, "ABC*a*いう");
}
