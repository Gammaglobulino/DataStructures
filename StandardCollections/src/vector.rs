
//experimenting with Vec

pub fn vector(){
    let mut v=Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    assert_eq!(v.len(),3);

    println!("v={:?}",v);
    let idx:usize=0;
    println!("v[0]={}",v[idx]);

    match v.get(2){
        Some(x) => println!("a[3]={}",x),
        None => println!("error, no such element")
    }

    for x in &v{println!("{}",x)};

    assert_eq!(v.pop(),Some(3));
    assert_eq!(v.len(),2);
    let _ = v.pop();
    let _ = v.pop();
    assert_eq!(v.pop(),None);

    v.push(1);
    v.push(2);
    v.push(3);

    while let Some(x)=v.pop(){
        println!("{}",x);
    }

}