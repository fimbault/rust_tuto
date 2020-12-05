#[derive(Debug)]
struct Account {
    balance: u32,
}

impl Account {

    // earn money
    fn earn(&mut self, amount: u32) {
        self.balance += amount;
    }
    
}

fn transfer(source: &mut Account, target: &mut Account, amount: u32) {
    source.balance -= amount;
    target.balance += amount;
}

fn main() {
    let mut acct1 = Account { balance: 20 };
    let mut acct2 = Account { balance: 10 };

    transfer(&mut acct1, &mut acct2, 3);

    let mut acct3 = Account { balance : 0 };
    acct3.earn( 10);

    println!("Account 1: {:?}", acct1);
    println!("Account 2: {:?}", acct2);
    println!("Account 3: {:?}", acct3);
}
