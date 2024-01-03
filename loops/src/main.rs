fn main() {
    let mut count = 0;

    'counting_up : loop {
        println!("count: {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;

        }

        count += 1;
    }
    println!("End Count: {count}");

    let mut number = 3;

    while number != 0{
        println!("{number}");
        number -= 1;
    }
    println!("LIFT OFF !!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("{element}");
    }

    let b = (1..4);

    for number in b.rev() {
        println!("{number}");
    }
    println!("LIFT OFF !!");
}