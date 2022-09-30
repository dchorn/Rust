pub fn say_hello() {
    println!("Hello, world!");
}

//pub fn print() {
//    let numbers = [1,2,3,4,5];
//    for number in numbers.iter() {
//        println!("{}",number);
//    }
//}

pub fn print() {
    let numbers = vec![1,2,3,4,5];
    for number in numbers {
        println!("{}",number);
    }
}
