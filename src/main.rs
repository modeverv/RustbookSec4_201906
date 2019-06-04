fn main() {
    println!("Hello, world!");
    let n = 42;
    let c = 'R';
    let ret = hello();
    assert_eq!(ret,());
    assert_eq!(std::mem::size_of::<()>(),0);
    let b1 = true;
    let b2 = !b1;
    let xu128:u128 = 0x11;
    println!("{}",xu128);
    let pw2 = 2_u128.pow(127);
    println!("{} - {}", pw2,std::u128::MAX);
    println!("{}",'\u{1f600}');
    println!("{}","\u{1f468}\u{200D}\u{1f469}\u{200D}\u{1f467}");
    let n1 = 200u32;
    let n1_ptr = &n1;
    let mut n2 = 0;
    let n2_ptr_mut = &mut n2;
    let a1 = ['a'; 50];
    for ch in a1.iter() {
        print!("{} ", *ch);
    }
    let t1 = (3, "bur".to_string());
    let mut b1 = Box::new(t1);
    (*b1).0 += 1;
    assert_eq!(*b1, (4,"bur".to_string()));
    let aa1 = ['a','b'];
    assert_eq!(aa1.get(0), Some(&'a'));
    let mut o1 = Some(10);
    match o1 {
        Some(s) => assert_eq!(s, 10),
        None => unreachable!(),
    }
    if let Some(s) = o1 {
        assert_eq!(s, 10);
    }
    // 5-2-7から
    let triangle = Polygon{
        vertexes: vec![(0,0),(3,0),(2,2)],
        fill: (255,255,255),
        stroke_width: 1,
    };
    let Polygon {vertexes: vx, ..}  = triangle;
    assert_eq!(3, vx.len());
    // 6から
}

struct Polygon {
    vertexes:  Vec<(i32,i32)>,
    stroke_width: u8,
    fill: (u8,u8,u8)
}

fn hello(){
    println!("Hello");
}
