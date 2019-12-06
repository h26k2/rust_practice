
pub fn concept(){

    /***********************************************
     * Shadowing, Code Blocks and reference concept
    ************************************************/

    //This is called shadowing.. a same variable name is declared so many times without 
    //having any error 

    let x = "My name is Hasnain Karim !";
    println!("value of x is : {}",x);

    let x = x.len();
    println!("value of x is : {}",x);

    let x  = "Something another";
    println!("value of x is : {}",x);

    /********************************************************************
     * Code blocks are isolated blocks of code their scope lies only
     * in between of their braces
    *********************************************************************/

    let general_statement = "Rust is an interesting language!";
    let x = 5; 

    {
        println!("general statement is : {}",general_statement);
        let x = x + 1;
        println!("value of x is : {}",x);
        let something = "123"; //This is not accessible outside of this block
        println!("hello i am accessible within the code block : {}",something);
    }

    println!("vlaue of x is : {}",x);
    //println!("value of something is : {}",something); remove comment to check

    /***************************************************************************************
     * Referencing is like accessing value of a variable directly from a memory reference
     * of that variable
    *****************************************************************************************/

    let mut num = 5;
    
    println!("value of num is : {}",num);

    //Now changing its value not directly but by using referencing

    {
        let ref_num = &mut num;
        *ref_num += 5;
        println!("value of num after incrementing 5 in its value : {}",num);
    }

    //println!("ref num : {}",ref_num); not accessible because of code blocking
    println!("original value of outside of the code block is : {}",num);


}