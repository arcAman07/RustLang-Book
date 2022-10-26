pub fn create_vector() {
    let mut v1:Vec<i32> = Vec::new();
    let v2 = vec![1,2,3];
    v1.push(5);
    v1.push(10);
    println!("{:?}", &v1);
    let third = &v2[2];
    println!("The third element is {}", third);
    let second :Option<&i32> = v1.get(1);
    match second  {
        Some(s) => println!("Second: {}", s),
        None => ()
    }
}
