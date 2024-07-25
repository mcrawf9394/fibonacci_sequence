use std::io;

fn main() {
    println!("Enter Number");
    let mut num: String = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let num: i32 = num.trim().parse().expect("please enter a number");
    let mut current_number: i32 = 2;
    let mut previous_number: i32 = 1;
    if num < 3 {
        println!("1")
    } else if num == 3 {
        println!("2")
    } else {
        for _i in 4..=num {
            let new_num: i32 = previous_number + current_number;
            previous_number = current_number;
            current_number = new_num;
        }
        println!("{current_number} is the current number of the sequence");
        println!("{} is an approximation of Phi using the last two numbers of the sequence", current_number as f32 / previous_number as f32)
    }
}