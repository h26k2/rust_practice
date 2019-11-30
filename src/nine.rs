
pub fn even_num(start_range:u64 , end_range:u64){
    
    for i in start_range..end_range + 1 {
        if (i%2) == 0 {
            println!("{}",i);
        }
    }
    
}