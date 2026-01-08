fn main() {
    let num = [1,2,3,4,5,6];
    println!("aray methods : {}",num.len());

    let num: [i32;5] = [1,2,3,4,5];
    println!("array size() : {}",num.len());

    let num = ["abc","cbr","somethingweired"];
    println!("Array of string : {}",num.len());

    #[allow(unused_variables)]
     let stringarry = ["alla","this","bullshit"];

    // stringarry.rotate_left(2);
    println!("i m checking the rust analyser feedback how this is working : {:? }", stringarry);


}
 