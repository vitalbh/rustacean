
fn returns_closure_sum() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| {
        println!("Calling function returns_closure_sum {x}");
        x + 1
    })
}

fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    println!("Calling function apply_with_log on {input}");
    //func(input);
    func(input)
}

fn apply_with_log_double(func: impl Fn(i32) -> i32, input: i32) -> i32 {
    println!("Calling function apply_with_log_double on {input}");
    func(func(input))
}

fn main() {
    let add_3 = |x| x + 3;
    let mul_5 = |x| x * 5;
    let more_one = returns_closure_sum();
    println!("closure returned");
    println!("sum +1: {}", more_one(10));
    println!("add_3: {}", apply_with_log(add_3, 10));
    println!("mul_5: {}", apply_with_log(mul_5, 20));

    println!("add_3: {}", apply_with_log_double(add_3, 10));
    println!("mul_5: {}", apply_with_log_double(mul_5, 20));
}