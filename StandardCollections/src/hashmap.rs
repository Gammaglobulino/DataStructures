use std::collections::HashMap;

pub fn myhasmaptest(){

    let mut shapes=HashMap::new();
    shapes.insert(String::from("Square"),4);
    shapes.insert(String::from("Triangle"),3);
    assert_eq!(shapes.len(),2);
    println!("{:?}",shapes);

    println!("a Square has {} sides",shapes["Square".into()]);

    for (k,v) in &shapes{
        println!("{}:{}",k,v);
    }

    shapes.insert("Square".into(),5);
    println!("{:?}",shapes);

    shapes.entry("Circle".into()).or_insert(1);
    {
        let actual=shapes
            .entry("Circle".into())
            .or_insert(2);
        *actual-=1;
    }
    println!("{:?}",shapes);


}