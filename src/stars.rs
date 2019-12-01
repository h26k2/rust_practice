
pub fn right_angle_triangle(){

    for i in 1..6{
        for j in 1..i+1{
            print!("*");
        }
        println!("");
    }

}

pub fn right_angle_trianle_reverse(){

    for i in (1..6).rev(){
        for j in 1..i+1{
            print!("*");
        }
        println!("");
    }

}

pub fn left_angle_triangle_reverse(){

    for i in (1..6).rev(){
        
        for spaces in 1..6-i{
            print!(" ");
        }

        for j in 1..i{
            print!("*");
        }

        println!("");

    }

}

pub fn left_angle_triangle(){

    for i in (1..6).rev(){
        
        let mut count = 0;

        for spaces in 1..i+1{
            print!(" ");
            count = count + 1;
        }

        for j in 1..6-count{
            print!("*");
        }

        println!("");

    }

}

pub fn triangle(){

    let mut strs = 1;

    for i in (1..6).rev(){

        for spaces in 1..i{
           print!(" ");
        }

        for j in 1..strs + 1{
           print!("*");
        }

        strs = strs + 2;
        println!("");        

    }

}
