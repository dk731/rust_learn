#[derive(Debug)]
enum QuarterTypes {
    Small,
    Medium,
    Big,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(QuarterTypes),
}

impl Coin {
    fn value(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                let base_value: f64 = 25.0;

                (base_value
                    * match state {
                        QuarterTypes::Small => 0.5,
                        QuarterTypes::Medium => 1.0,
                        QuarterTypes::Big => 2.0,
                    }) as u8
            }
        }
    }
}

fn main() {
    let coin = Coin::Quarter(QuarterTypes::Big);

    println!("My Coin: {:?} ({})", coin, coin.value());
}

fn add_one(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i + 1,
    }
}
