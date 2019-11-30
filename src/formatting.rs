
pub fn print_hero_stars(text:&str){
    
    println!("\n");

    for rows in 1..31 {
        print!("*");
    }

    println!("\n{}",text);

    for rows in 1..31{
        print!("*");
    }

    println!("\n");

}