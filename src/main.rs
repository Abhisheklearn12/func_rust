// Define a function that applies another function to each element of a list
fn apply_to_list<F>(list: &Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    list.iter().map(|&x| f(x)).collect()
}

// Define a function that multiplies a number by 2
fn multiply_by_two(x: i32) -> i32 {
    x * 2
}

// Define a closure that adds 10 to a number
fn add_ten_closure() -> impl Fn(i32) -> i32 {
    |x| x + 10
}

// Function to filter only even numbers from the list
fn filter_evens(list: &Vec<i32>) -> Vec<i32> {
    list.iter().filter(|&&x| x % 2 == 0).cloned().collect()
}

// Function to fold the list into a sum of its elements
fn sum_list(list: &Vec<i32>) -> i32 {
    list.iter().fold(0, |acc, &x| acc + x)
}

// Function to demonstrate pattern matching with a custom type
#[derive(Debug)]
enum Status {
    Success(String),
    Error(String),
}

// Function to check if a number is positive and return a Status
fn check_number_status(x: i32) -> Status {
    match x {
        x if x > 0 => Status::Success(format!("{} is positive", x)),
        x if x < 0 => Status::Error(format!("{} is negative", x)),
        _ => Status::Error(String::from("Zero is neither positive nor negative")),
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, -1, 0];

    // Apply multiply_by_two to the list using functional programming style
    let doubled = apply_to_list(&numbers, multiply_by_two);
    println!("Doubled: {:?}", doubled);

    // Apply the closure add_ten to the list
    let add_ten = add_ten_closure();
    let increased_by_ten = apply_to_list(&numbers, add_ten);
    println!("Increased by 10: {:?}", increased_by_ten);

    // Filter the list to only even numbers
    let evens = filter_evens(&numbers);
    println!("Even numbers: {:?}", evens);

    // Sum all numbers in the list
    let total_sum = sum_list(&numbers);
    println!("Sum of all numbers: {}", total_sum);

    // Check the status of each number using pattern matching
    for &num in &numbers {
        let status = check_number_status(num);
        match status {
            Status::Success(msg) => println!("Success: {}", msg),
            Status::Error(msg) => println!("Error: {}", msg),
        }
    }
}
//yeh func_rust ki file hain