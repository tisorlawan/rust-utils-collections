fn get_ref<T: std::fmt::Display>(s: &mut T) -> &mut T {
    s
}

fn main() {
    let mut i: i32 = 10;
    let j = get_ref(&mut i);
    *j += 1;
    println!("{}", i);

    let mut r = [0; 10];

    let i = match r.get_mut(0) {
        Some(i) => i,
        None => panic!("Wew"),
    };

    *i += 25;
    println!("{}", r[0]);
}
