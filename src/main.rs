

fn main() {
    let name: &str = "Goku";


    let a: i32 = 8_9999;
    let b: i32 = 1;


    let result: i32 = add_numbers(a, b);

    println!("Hello, rust world! My Name is {}!", name);
    println!("My power level is over {} !!!", result);

    let mut message: String = String::from("Hi there!");
    message.push_str(" How are you?");
    println!("{}", message); // Output: Hi there! How are you?

    let last_number: i32 = looper();
    println!("{}", last_number); // 12

    vector_practice()

}


fn add_numbers(number_1: i32, number_2: i32) -> i32{
    let result: i32 = number_1 + number_2;
    result
}

fn looper() -> i32{
    let mut last: i32 = 0;
    for item in 1..100 {
        if item >= 100 {
         break;
        };
    last = item;}

    assert_eq!(last, 99);
    last

}

fn vector_practice(){
    let mut nums: Vec<i32> = vec![1, 2, 3]; // mut nums: Vec<i32>

    nums.push(4);

    println!("The length of nums is now {}", nums.len()); // Prints 4

}