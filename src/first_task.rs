
mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
mod seven;
mod eight;
mod nine;
mod ten;
mod formatting;


pub fn task1(){

    formatting::print_hero_stars("Program by : h26k2");

    //Task 1
    formatting::print_hero_stars("Output of Task-1");
    one::string_operation();

    //Task 2
    formatting::print_hero_stars("Output of Task-2");
    two::integer_init();

    //Task 3
    formatting::print_hero_stars("Output of Task-3");
    three::float_init();

    //Task 4
    formatting::print_hero_stars("Output of Task-4");
    four::arithmetic_operations();

    //Task 5
    formatting::print_hero_stars("Output of Task-5");
    five::array_task();

    //Task 6
    formatting::print_hero_stars("Output of Task-6");
    six::tuple_init();

    //Task 7
    formatting::print_hero_stars("Output of Task-7");
    println!("value of pie is : {}",seven::return_pie());

    // Task 8
    formatting::print_hero_stars("Output of Task-8");
    eight::leap_year(2020);

    //Task 9
    formatting::print_hero_stars("Output of Task-9");
    nine::even_num(1,20);

    //Task 10
    formatting::print_hero_stars("Output of Task-10");
    ten::fib_series(5);
}