fn main() {
    let x = gen_nth_fib_num(5);
    println!("{x}");

    let temp_c = 100.0;
    let temp_c_2_f = temp_converter(temp_c, 1);
    println!("{temp_c_2_f}");

    let temp_f = 32.0;
    let temp_f_2_c = temp_converter(temp_f, 2);
    println!("{temp_f_2_c}");

}

fn temp_converter(temp:f32, option:i8) -> f32{
    // Option 1: convert from celsius to fahrenheit
    // Option 2: convert from fahrenheit to celsius
    
    let mut new_temp: f32 = 0.0;

    if option == 1 {
        new_temp = (temp * 9.0 / 5.0) + 32.0;
    } else if option == 2 {
        new_temp = (temp - 32.0) * 5.0 / 9.0;
    } else {
        println!("Not a valid option !!");
    }

    return new_temp;
}

fn gen_nth_fib_num(n:i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c: i32;
    if n <= 1 {
        return n
    }

    else {
        // Recursive

        // return gen_nth_fib_num(n-1) + gen_nth_fib_num(n-2)

        // Iterative

        for _ in 2..n+1 {
            c = a + b;
            a = b;
            b = c;
        }
        return b;
    }
}