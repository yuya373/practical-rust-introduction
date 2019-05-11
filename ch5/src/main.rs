use std::boxed::Box;

fn main() {
    println!("Hello, world!");
    let t1 = (3, "birds".to_string());
    println!("{:?}", &t1);
    let mut b1 = Box::new(t1);
    (*b1).0 += 1;
    assert_eq!(*b1, (4, "birds".to_string()));
    println!("{:?}", b1);
    // println!("{:?}", &t1);
    println!("Size: {}", std::mem::size_of::<()>());
    println!("Size: {}", std::mem::size_of::<Box<()>>());
    println!("Size: {}", std::mem::size_of::<char>());
    println!("Size: {}", std::mem::size_of::<Box<char>>());
    println!("Size: {}", std::mem::size_of::<u32>());
    println!("Size: {}", std::mem::size_of::<Box<u32>>());
    println!("Size: {}", std::mem::size_of::<Vec<String>>());
    println!("Size: {}", std::mem::size_of::<Box<Vec<String>>>());

    enum List<T> {
        Nil,
        Cons(T, Box<List<T>>),
    }

    let v3 = vec![0; 100];
    assert_eq!(v3.len(), 100);

    let mut v6 = vec!['a', 'b', 'c'];
    v6.push('d');
    v6.push('e');
    assert_eq!(v6, ['a', 'b', 'c', 'd', 'e']);
    assert_eq!(v6.pop(), Some('e'));
    v6.insert(1, 'f');
    assert_eq!(v6.remove(2), 'b');
    assert_eq!(v6, ['a', 'f', 'c', 'd']);

    let mut v7 = vec!['g', 'h'];
    v6.append(&mut v7);
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h']);
    assert_eq!(v7, []);

    let a8 = ['i', 'j'];
    v6.extend_from_slice(&a8);
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h', 'i', 'j']);
    assert_eq!(a8, ['i', 'j']);

    let mut v: Vec<char> = Vec::with_capacity(1000);
    println!("length: {}, capacity: {}", v.len(), v.capacity());
    v.shrink_to_fit();
    println!("length: {}, capacity: {}", v.len(), v.capacity());

    let mut v1 = vec![0, 1, 2, 3];
    v1.push(4);
    println!("v1 len: {}, capacity: {}", v1.len(), v1.capacity());
    let s1 = v1.into_boxed_slice();
    let v2 = s1.into_vec();
    println!("v2 len: {}, capacity: {}", v2.len(), v2.capacity());

    use std::collections::HashMap;

    let mut m1 = HashMap::new();
    m1.insert("a", 1);
    m1.insert("b", 3);
    assert_eq!(m1.len(), 2);
    assert_eq!(m1.get("a"), Some(&1));
    assert_eq!(m1.get("c"), None);

    let d = m1.entry("d").or_insert(0);
    *d += 7;
    assert_eq!(m1.get("d"), Some(&7));

    let m2 = vec![("a", 1), ("b", 3)]
        .into_iter()
        .collect::<HashMap<_, _>>();
    println!("m2: {:?}", m2);

    let mut s1 = "ラズベリー".to_string();
    let mut s2 = String::from("ブラックベリー");

    s1.push_str("タルト");
    assert_eq!(s1, "ラズベリータルト");
    s2.push('と');
    s2.push_str(&s1);
    assert_eq!(s2, "ブラックベリーとラズベリータルト");

    let i = 42;
    assert_eq!(i.to_string(), "42");

    let f = 4.3 + 0.1;
    assert_eq!(f.to_string(), "4.3999999999999995");
    assert_eq!(format!("{:.2}", f), "4.40");

    let t = (1, "ABC");
    assert_eq!(format!("{:?}", t), r#"(1, "ABC")"#);
    let s1 = "42";
    assert_eq!(s1.parse::<i32>(), Ok(42));
    let s2 = "abc";
    let r2: Result<f64, _> = s2.parse();
    assert!(r2.is_err());
    println!("{:?}", r2);

    let utf16: Vec<u16> = vec![0x61, 0x62, 0x6f22, 0x5b57];
    if let Ok(s) = String::from_utf16(&utf16) {
        assert_eq!(s, "ab漢字");
    } else {
        unreachable!();
    };

    let bs1 = b"abc\xe3\x81\x82";
    assert_eq!(bs1, &[b'a', b'b', b'c', 0xe3, 0x81, 0x82]);

    let bs2 = br#"ab\ncd"#;
    assert_eq!(bs2, &[b'a', b'b', b'\\', b'n', b'c', b'd']);

    let a = ['a', 'b', 'c', 'd', 'e'];
    assert_eq!(a[..], ['a', 'b', 'c', 'd', 'e']);
    assert_eq!(a[..3], ['a', 'b', 'c']);
    assert_eq!(a[..=3], ['a', 'b', 'c', 'd']);
    assert_eq!(a[1..], ['b', 'c', 'd', 'e']);
    assert_eq!(a[1..3], ['b', 'c']);
    assert_eq!(a[1..=3], ['b', 'c', 'd']);

    assert_eq!(.., std::ops::RangeFull);
    assert_eq!(..3, std::ops::RangeTo { end: 3 });
    assert_eq!(..=3, std::ops::RangeToInclusive { end: 3 });
    assert_eq!(1.., std::ops::RangeFrom { start: 1 });
    assert_eq!(1..3, std::ops::Range { start: 1, end: 3 });
    assert_eq!(1..=3, std::ops::RangeInclusive::new(1, 3));

    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a'));
    assert_eq!(a1.get(4), None);

    let mut o1 = Some(10);
    match o1 {
        Some(s) => assert_eq!(s, 10),
        None => unreachable!(),
    }

    o1 = Some(20);
    if let Some(s) = o1 {
        assert_eq!(s, 20);
    };

    let mut o2 = Some(String::from("Hello"));
    assert_eq!(o2.unwrap(), "Hello");

    o2 = None;
    // o2.unwrap();
    // thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:347:21
    assert_eq!(
        o2.unwrap_or_else(|| String::from("o2 is none")),
        "o2 is none"
    );

    let mut o3 = Some(25);
    assert_eq!(o3.map(|n| n * 10), Some(250));

    o3 = None;
    assert_eq!(o3.map(|n| n * 10), None);

    o3 = Some(10);
    assert_eq!(
        o3.map(|n| n * 10)
            .and_then(|n| if n >= 200 { Some(n) } else { None }),
        None
    );

    fn add_elms(s: &[i32]) -> Option<i32> {
        // if let Some(s0) = s.get(0) {
        //     if let Some(s3) = s.get(3) {
        //         Some(s0 + s3)
        //     } else {
        //         None
        //     }
        // } else {
        //     None
        // }

        let s0 = s.get(0)?;
        let s3 = s.get(3)?;
        Some(s0 + s3)
    }

    assert_eq!(add_elms(&[3, 7, 31, 127]), Some(3 + 127));
    assert_eq!(add_elms(&[7, 11]), None);

    assert_eq!("10".parse::<i32>(), Ok(10));
    let res0 = "a".parse::<i32>();
    assert!(res0.is_err());
    println!("{:?}", res0);

    fn add0(s0: &str, s1: &str) -> Result<i32, std::num::ParseIntError> {
        let s0 = s0.parse::<i32>()?;
        let s1 = s1.parse::<i32>()?;

        Ok(s0 + s1)
    }

    assert_eq!(add0("3", "127"), Ok(130));
    assert!(add0("3", "abc").is_err());

    fn add1(s0: &str, s1: &str) -> Result<i32, String> {
        let s0 = s0.parse::<i32>().map_err(|_| "s0 is not integer")?;
        let s1 = s1.parse::<i32>().map_err(|_| "s1 is not integer")?;

        Ok(s0 + s1)
    }

    assert_eq!(add1("3", "abc"), Err("s1 is not integer".to_string()));

    enum MyError {
        ParseError(std::num::ParseIntError),
    }

    fn f() -> Result<(), MyError> {
        "abc".parse::<i32>()?;
        Ok(())
    }

    impl std::convert::From<std::num::ParseIntError> for MyError {
        fn from(source: std::num::ParseIntError) -> Self {
            MyError::ParseError(source)
        }
    }

    let r: Result<(), String> = None.ok_or_else(|| "Err".to_string());

    type UserName = String;
    type Id = i64;
    type Timestamp = i64;
    type User = (Id, UserName, Timestamp);

    fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
        (id, name, created)
    }

    let id = 400;
    let now = 4567890123;
    let user = new_user(String::from("mika"), id, now);

    let bad_user = new_user(String::from("bad user"), now, id);

    use std::cell::RefCell;
    use std::rc::Rc;

    pub type SharedMap<K, V> = Rc<RefCell<HashMap<K, V>>>;

    struct Polygon {
        vertexes: Vec<(i32, i32)>,
        stroke_width: u8,
        fill: (u8, u8, u8),
    }
    struct Vertex(i32, i32);
    struct Triangle(Vertex, Vertex, Vertex);
    struct UniqueValue;
    // struct UniqueValue {};
    // struct UniqueValue ():

    fn new_polygon(vertexes: Vec<(i32, i32)>) -> Polygon {
        let stroke_width = 1;
        let fill = (0, 0, 0);
        Polygon {
            vertexes,
            stroke_width,
            fill,
        }
    }

    let quadrangle = new_polygon(vec![(5, 2), (4, 7), (10, 6), (8, 1)]);
    let triangle = new_polygon(vec![(0, 0), (3, 0), (2, 2)]);

    assert_eq!(triangle.vertexes[0], (0, 0));
    assert_eq!(triangle.vertexes.len(), 3);
    assert_eq!(triangle.fill, (0, 0, 0));

    let Polygon {
        vertexes: quad_vx, ..
    } = quadrangle;
    assert_eq!(4, quad_vx.len());

    let Polygon { fill, .. } = quadrangle;
    assert_eq!((0, 0, 0), fill);

    let mut polygon = new_polygon(vec![(-1, -5), (-4, 0)]);
    assert_eq!(polygon.vertexes.len(), 2);
    polygon.vertexes.push((2, 8));
    assert_eq!(polygon.vertexes.len(), 3);

    let triangle1 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 5,
    };

    let triangle2 = Polygon {
        vertexes: vec![(0, 0), (-3, 0), (-2, 2)],
        ..triangle1
    };

    let polygon1: Polygon = Default::default();
    let polygon2 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        ..Default::default()
    };

    impl Default for Polygon {
        fn default() -> Self {
            Self {
                stroke_width: 1,
                vertexes: Default::default(),
                fill: Default::default(),
            }
        }
    }

    let p: Polygon = Default::default();
    assert_eq!(p.stroke_width, 1);

    let vx0 = Vertex(0, 0);
    let vx1 = Vertex(3, 0);
    let triangle = Triangle(vx0, vx1, Vertex(2, 2));
    assert_eq!((triangle.1).0, 3);

    {
        struct UserName(String);
        struct Id(u64);
        struct Timestamp(u64);

        type User = (Id, UserName, Timestamp);

        fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
            (id, name, created)
        }

        let id = Id(400);
        let now = Timestamp(4567890123);
        // let bad_user = new_user(UserName(String::from("bad user")), now, id);
    }

    {
        #[derive(Debug, PartialEq)]
        struct UniqueValue;

        let uv1 = UniqueValue;
        let uv2 = UniqueValue;
        assert_eq!(uv1, uv2);
    }

    {
        // enum
        #[derive(Debug, PartialEq)]
        enum Weekday {
            // variant
            Monday,
            Tuesday,
            Wednesday,
            Thursday,
            Friday,
        }

        fn say_something(weekday: Weekday) {
            if weekday == Weekday::Friday {
                println!("Thanks God, it's Friday")
            } else {
                println!("まだ{:?}か", weekday)
            }
        }
        say_something(Weekday::Friday);

        enum Month {
            January = 1,
            February = 2,
            March = 3,
            December = 12,
        }
        assert_eq!(3, Month::March as isize);

        type UserName = String;
        #[derive(Debug)]
        enum Task {
            Open,
            AssignedTo(UserName),
            Working {
                assignee: UserName,
                remaining_hours: u16,
            },
            Done,
        }

        use Task::*;

        let tasks = vec![
            AssignedTo(String::from("junko")),
            Working {
                assignee: String::from("hiro"),
                remaining_hours: 18,
            },
            Done,
        ];

        for (i, task) in tasks.iter().enumerate() {
            let i = i + 1;

            match task {
                AssignedTo(assignee) => println!(
                    "タスク{}は{}さんにアサインされています",
                    i, assignee
                ),
                Working {
                    assignee,
                    remaining_hours,
                } => println!(
                    "タスク{}は{}さんが作業中です。残り{}時間の見込み",
                    i, assignee, remaining_hours
                ),
                _ => println!(
                    "タスク{}はその他のステータス（{:?}）です",
                    i, task
                ),
            }
        }

        struct StrRefs<'a> {
            s1: &'a str,
            s2: &'a str,
        }

        #[derive(Default)]
        struct A {
            f0: u8,
            f1: u32,
            f2: u8,
            f3: u16,
        }

        let a: A = Default::default();
        println!(
            "struct A ({} bytes)\n f0: {:p}\n f1: {:p}\n f2: {:p}\n f3: {:p}\n",
            std::mem::size_of::<A>(),
            &a.f0,
            &a.f1,
            &a.f2,
            &a.f3
        );
    }

    let i1 = 42;
    let f1 = i1 as f64;

    println!("{:?}", f1);

    let c1 = 'a';
    assert_eq!(97, c1 as u32);

    let i2 = 300;
    let u1 = i2 as u8;
    assert_eq!(44, u1); // 300 = (256 * 1) + 44

    let t1 = ('a', 42);
    // let t2 = t1 as (u32, u8);

    let v1 = vec![b'h', b'e', b'l', b'l', b'o'];
    // let v2 = v1 as Vec<u16>;

    let t3 = (t1.0 as u32, t1.0 as u8);
    let v3 = v1.iter().map(|&n| n as u16).collect::<Vec<u16>>();

    let v4: Vec<u8> = From::from("hello");
    assert_eq!(v1, v4);

    let p1: Box<i32> = Box::new(10);
    // let p2 = p1 as *mut i32;
    let p3: *mut i32 = unsafe { std::mem::transmute(p1) };

    let f1 = 5.6789e+3_f32;
    let i1 = f1 as i32;
    println!("{} as {}", f1, i1);

    let i2: i32 = unsafe { std::mem::transmute(f1) };
    println!("{} as {}", f1, i2);
}
