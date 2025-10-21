use std::io;

fn add_calc(x: i32, y: i32) -> i32
{
    x + y
}

fn sub_calc(x: i32, y: i32) -> i32
{
    x - y
}

fn mult_calc(x: i32, y: i32) -> i32
{
    x * y
}

fn div_calc(x: f64, y: f64) -> f64
{
    x / y
}

fn calc_menu()
{
    println!("What do you wanna do?\n[1]Add, [2]Subtract, [3]Multiply,[4]Divide");
    let mut option = String::new();

    let user_input = io::stdin();

    user_input
        .read_line(&mut option)
        .expect("ERROR FOUND");

    let option: i32 = option
        .trim()
        .parse()
        .expect("ERROR FOUND");

        if option == 1
        {

            let mut val_one = String::new();
            let mut val_two = String::new();

            println!("Input two integers to be added");

            user_input
                .read_line(&mut val_one)
                .expect("ERROR FOUND");

            let val_one: i32 = val_one
                .trim()
                .parse()
                .expect("ERROR FOUND");
            
                user_input
                .read_line(&mut val_two)
                .expect("ERROR FOUND");

            let val_two: i32 = val_two
                .trim()
                .parse()
                .expect("ERROR FOUND");

            println!("The sum is: {}", add_calc(val_one,val_two));
        }
        
        else if option == 2
        {
            let mut val_one = String::new();
            let mut val_two = String::new();

            println!("Input two integers to be subtracted");

            user_input
                .read_line(&mut val_one)
                .expect("ERROR FOUND");

            let val_one: i32 = val_one
                .trim()
                .parse()
                .expect("ERROR FOUND");
            
                user_input
                .read_line(&mut val_two)
                .expect("ERROR FOUND");

            let val_two: i32 = val_two
                .trim()
                .parse()
                .expect("ERROR FOUND");

            println!("The difference is: {}", sub_calc(val_one,val_two));
        }
        else if option == 3
        {
            println!("{}", mult_calc(5,5));
        }
        else if option == 4
        {
            println!("{}", div_calc(15.0, 2.0));
        }
}

fn main()
{   
    calc_menu();
}