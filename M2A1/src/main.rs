fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {

    let mut cur = low;
    
    // Support traversing in positive and negative steps, since step size is signed
    // When negative steps, "high" should be the smaller integer
    while (step > 0 && cur <= high) || (step < 0 && cur >= high) {
        *total += cur;
        cur += step;
    }
}

fn main() {
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
}
