fn main() {
    let s:&'static str="Hello I'm a string";
    for c in s.chars(){
        println!("{}",c);
    }
    // reverse
    for c in s.chars().rev(){
        println!("{}",c);
    }
    // take the first char

    if let Some(first_char)=s.chars().nth(0){
        println!("first letter is {}",first_char);
    }

    let mut heapString=String::from("Hello I'm a string allocated to heap");
    heapString=heapString+"!";
    println!("{}",heapString);

    // convert static string to heap
    let mut newstr=s;
    let newstr=newstr.replace("Hello","Hey");
    println!("{}",newstr);

    //formatting strings

    let name="Andrea";
    let greetings=format!("hi, {0} how're you today? {0} the great!",name);
    println!("{}",greetings);

    //repeat arguments

    let phrase=format!("I'm my name is {lastname}, {name} {lastname}",name="James",lastname="Bond");
    println!("{}",phrase);



}
