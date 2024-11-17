fn main() {
    let x = 5; //if we delcare variable default variable is immutable
    println!("x value = {x}");
    /*
    x = 6;
    println!("x value = {x}");
    */
    let mut y = 6; //if we delcare variable that can be chagne the value we use mut (is mutable) at infont of the name of variable
    println!("x value = {y}");
    y = 8;
    println!("y value = {y}");

    /*if we want to create a constant variable the name of the variable need to be the capital letters and we need to delcare the data type of it*/
    const THREE_HOUR_IN_SECONDS: u32 = 60 * 60 * 3; 
    println!("3 hour in sec is: {THREE_HOUR_IN_SECONDS}");

    //global variable vs local variable and shadowing
    let z = 5;

    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }

    println!("The value of z is: {z}");

    //difference between mut and shadowing(let) is the shadowing can change the type of the variable

    //shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("value of space: {spaces}");

    //mut
    //let mut spaces = "   "; 
    //spaces = spaces.len(); error if we use mut we can't change the variable
}
