fn main (){
    let A:i32 = 10;
    let B:i32 = 20;
    
    println!("Value of A:{}",A);
    println!("Value of B{}",B);

    let mut res = A>B;
    println!("A greater then B:{}",res);

    res = A<B;
    println!("A lesser then B:{}",res);

    res = A>=B;
    println!("A greater then or equal to B:{}",res);

    res = A<=B;
    println!("A lesser then or equal to B:{}",res);

    res = A==B;
    println!("A is equal to B:{}",res);

    res = A !=B;
    println!("A is not eual to B:{}",res);
  
}