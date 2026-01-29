fn main() {
    for i in 1..101 {
        let mut output: String = String::new();
        match (i % 3, i % 5) {
            (0, 0) => output.push_str("FizzBuzz"),
            (0, _) => output.push_str("Fizz"),
            (_, 0) => output.push_str("Buzz"),
            (_, _) => output.push_str(""),
        }
        if output.is_empty() {
            println!("{}", i)
        } else {
            println!("{}", output)
        }
    }
}
