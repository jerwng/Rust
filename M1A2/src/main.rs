fn is_even(n: i32) -> bool {
    return n % 2 == 0;
}

fn div_by_3(n: i32) -> bool {
    return n % 3 == 0;
}

fn div_by_5(n: i32) -> bool {
    return n % 5 == 0;
}

fn main() {
    let nums: [i32; 10]= [23, 456, 58, 2495, 120, 6, 3499, 345, 84, 100];

    /*
    For loop to do FizzBuzz on the array
    */
    for num in nums.iter() {
        println!("Current Number: {}", *num);

        let even_odd_string;

        if is_even(*num) {
            even_odd_string = "even"
        } else {
            even_odd_string = "odd"
        }

        println!("{} is {}", *num, even_odd_string);
        
        if div_by_5(*num) && div_by_3(*num) {
            println!("FizzBuzz")
        } else if div_by_5(*num) {
            println!("Buzz")
        } else if div_by_3(*num) {
            println!("Fizz")
        }

        println!(" ");  
    }

    /*
    While loop to find sum of the array
    */
    let mut nums_sum = 0;
    let mut i = 0;

    while i < nums.len() {
        nums_sum += nums[i];
        i += 1;
    }

    println!("Sum of the array is: {}", nums_sum);
    println!(" ");  

    /*
    Loop to find largest number in the array
    */
    let mut j = 0;
    let mut cur_largest = i32::MIN;
   
    let max_number = loop {
    	if j >= nums.len() {
    		break cur_largest;
    	}

        if nums[j] > cur_largest {
            cur_largest = nums[j];
        }

        j += 1;
    };

    println!("Largest number in the array is: {}", max_number);
}
