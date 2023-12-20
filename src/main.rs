/*
    money counter
    ask the user how many of each note / coin they have
    give them back their total
*/
use std::io;
use std::io::Write;

#[derive(Debug)]
struct Coin {
    // cash value (p)
    value: i32,     
    // number of coins
    coin_count: i32,
}

impl Coin {
    // &self - an instance method
    fn total(&self) -> i32 {
        self.value * self.coin_count
    }
}

fn get_input() -> i32 {
    let mut input_text = String::new();
    
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read input");
    let num: i32 = input_text.trim().parse().ok().expect("That's no number!");

    num
}

fn print_line(text: &str) {
    print!("{}", text);
    io::stdout()
        .flush()
        .unwrap();
}

fn p_to_pounds(pence: i32) -> String {
    format!("£{:.2}", pence as f32 / 100f32)
}

fn get_total(coins: Vec<Coin>) -> String {
    let mut total: i32 = 0;

    for coin in coins {
        total = total + coin.total();
    }

    p_to_pounds(total)
}

fn main() {
    let mut coins: Vec<Coin> = Vec::new();

    /*
        create a Coin
        add to vector
        iterate
    */
    println!("Enter number of coins: ");
    print_line("£2: ");
    coins.push(Coin {
        value: 200,
        coin_count: get_input(),
    });

    print_line("£1: ");
    coins.push(Coin {
        value: 100,
        coin_count: get_input(),
    });

    print_line("50p: ");
    coins.push(Coin {
        value: 50,
        coin_count: get_input(),
    });

    print_line("20p: ");
    coins.push(Coin {
        value: 20,
        coin_count: get_input(),
    });

    print_line("10p: ");
    coins.push(Coin {
        value: 10,
        coin_count: get_input(),
    });

    print_line("5p: ");
    coins.push(Coin {
        value: 5,
        coin_count: get_input(),
    });

    println!("\nTotal: {}", get_total(coins));
}

// one £2,
// three £1,
// five 50p coins,
// two 20p coins,
// one 10p coin
// and fifteen 5p coins.
#[test]
fn test_coins() {
    let mut coins: Vec<Coin> = Vec::new();

    coins.push(Coin {
        value: 200,
        coin_count: 1,
    });
    coins.push(Coin {
        value: 100,
        coin_count: 3,
    });
    coins.push(Coin {
        value: 50,
        coin_count: 5,
    });
    coins.push(Coin {
        value: 20,
        coin_count: 2,
    });
    coins.push(Coin {
        value: 10,
        coin_count: 1,
    });
    coins.push(Coin {
        value: 5,
        coin_count: 15,
    });

    assert!(get_total(coins) == "£8.75");
}
