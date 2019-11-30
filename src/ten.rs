
pub fn fib_series(terms:u64){

    let mut first; 
    let mut second;
    let mut next;

    first = 0;
    second = 1;
    
    println!("{}",first);
    println!("{}",second);

    for x in 1..terms + 1 -2 {
        
        next = first + second;
        println!("{}",next);

        first = second;
        second = next;

    }


}