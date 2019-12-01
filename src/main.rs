
mod stars;
mod formatting;

fn main(){

    //First Star
    
    formatting::print_hero_stars("FIRST STAR");
    stars::right_angle_triangle();

    //Second Star

    formatting::print_hero_stars("SECOND STAR");
    stars::right_angle_trianle_reverse();

    //Third Star

    formatting::print_hero_stars("THIRD STAR");
    stars::left_angle_triangle();

    //Fourth Star

    formatting::print_hero_stars("FOURTH STAR");
    stars::left_angle_triangle_reverse();

    //Fifth Star

    formatting::print_hero_stars("FIFTH STAR");
    stars::triangle();

}


