// require crate
use std::io;

// const declaration
const GLMR_PRICE: f64 = 0.23;

// first function executed
fn main() {
    // loop is making the menu consistant
    loop {
        // Display Modes
        println!("Choose a mode:");
        println!("1. USD TO GLMR");
        println!("2. GLMR TO USD");
        println!("3. APY CALCULATOR");
        println!("4. Exit");

        // Ask the user for a mode
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse the input as an integer and check if it matches
        match input.trim().parse::<u32>() {
            Ok(mode) => {
                match mode {
                    1 => {
                        mode_a();
                        continue;
                    }
                    2 => {
                        mode_b();
                        continue;
                    }
                    3 => {
                        mode_c();
                        continue;
                    }
                    4 => {
                        println!("Exiting...");
                        break;
                    }
                    _ => println!("Invalid mode choice!"),
                }
            }
            Err(_) => println!("Invalid input! Please enter a valid number."),
        }
    }
}

fn read_input() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Invalid input! Please enter a valid number.")
}


fn mode_a() {
    println!("How many dollars do you have?");

    let guess = read_input();
    let guess = dollars_to_glmr(guess);
    println!("You could also have {guess} GLMR");
}

fn mode_b() {
    println!("How many GLMR do you have?");

    let guess = read_input();
    let guess = glmr_to_dollar(guess);
    println!("You could also have {guess} USD");
}

fn mode_c() {
    println!("Welcome to the Weekly APY Calculator!");

    println!("Enter the initial amount:");
    let initial_amount: f64 = read_input();

    // Read the annual APY as a percentage from the user (number is needed)
    println!("Enter the annual APY:");
    let annual_apy: f64 = read_input();

  // Calculate future amount and earned amount after different time frames
  // First tuple definition in rust!
  let (future_amount_daily, earned_amount_daily) = calculate_future_and_earned_amount(initial_amount, annual_apy, 365);
  let (future_amount_weekly, earned_amount_weekly) = calculate_future_and_earned_amount(initial_amount, annual_apy, 52);
  let (future_amount_monthly, earned_amount_monthly) = calculate_future_and_earned_amount(initial_amount, annual_apy, 12);
  let (future_amount_yearly, earned_amount_yearly) = calculate_future_and_earned_amount(initial_amount, annual_apy, 1);

  // Display the results
  println!("Future Amount after one day: {:.2}", future_amount_daily);
  println!("Earned Amount after one day: {:.2}", earned_amount_daily);

  println!("Future Amount after one week: {:.2}", future_amount_weekly);
  println!("Earned Amount after one week: {:.2}", earned_amount_weekly);

  println!("Future Amount after one month: {:.2}", future_amount_monthly);
  println!("Earned Amount after one month: {:.2}", earned_amount_monthly);

  println!("Future Amount after one year: {:.2}", future_amount_yearly);
  println!("Earned Amount after one year: {:.2}", earned_amount_yearly);
}

fn dollars_to_glmr(x:f64) -> f64 {
    x / GLMR_PRICE
}

fn glmr_to_dollar(x:f64) -> f64 {
    GLMR_PRICE * x
}

// Take 3 parameters, return a tuple of two f64 numbers
fn calculate_future_and_earned_amount(initial_amount: f64, annual_apy: f64, time_frames: u32) -> (f64, f64) {
    // Convert annual APY to the appropriate time frame
    let apy_adjusted = (annual_apy / 100.0) / (time_frames as f64);

    // Calculate the future amount
    let future_amount = initial_amount * (1.0 + apy_adjusted);

    // Calculate the earned amount (interest earned)
    let earned_amount = future_amount - initial_amount;

    (future_amount, earned_amount)
}