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
}

fn hello(){
    println!("Hello");
}
