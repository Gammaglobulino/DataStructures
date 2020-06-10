use std::collections::HashSet;

pub fn myhashset(){
    let mut greeks=HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}",greeks); //sorted elements {"delta","gamma"}
    greeks.insert("delta"); // will not be inserted as already there
    assert_eq!(greeks.len(),2);
    println!("{:?}",greeks);
    if greeks.insert("vega"){
        println!("we addedd <<vega>> yeah!");
    }
    println!("{:?}",greeks);

    if greeks.contains("delta"){
        println!("yes- delta is there");
    }
    if greeks.remove("delta"){
        println!("delta removed");
        assert_eq!(greeks.len(),2);
    }

    let from1to5:HashSet<_>=(1..=5).collect();
    let from1to10:HashSet<i32>=(1..=10).collect();
    let from7to10:HashSet<i32>=(7..=10).collect();

    println!("is {:?} a subset of {:?}? {}",from1to5,from1to10,from1to5.is_subset(&from1to10));
    println!("{:?} is disjoint from {:?} {}",from1to5,from7to10,from1to5.is_disjoint(&from7to10));

}