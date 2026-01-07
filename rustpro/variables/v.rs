fn main(){
    let apple = 40; // variables are by default immutable
    let mut oranges = 50;
    let fruits = apple + oranges;
    println!("Tola fruits i have {} which omprises of {oranges} oranges and {apple} apple\n",fruits);

    oranges = 70;
    println!("oranges changes to {oranges}");

    //variable shadowing
    let oranges = oranges + 10;
    println!("after shadowing oranges changes to {oranges}");

    //just checking if uggestions are comming

    let strhello = "hello for this shit";
    let fnumber = 13.56;

    println!("this is string kine variable : {}",strhello);
    println!("this float type : {fnumber}");
}