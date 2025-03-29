#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Self {id, balance: 0, holder}
    }

    // use mutable references if the value it reference is going to be modified
    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    // use immutable references when the value is going to be used
    // but it's neither going to be modified nor its ownership is going to be taken 
    fn summary(&self) -> String {
        format!("Account: {} Holder: {} Balance:{}", self.id, self.holder, self.balance)
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>
}

impl Bank {
    fn new() -> Self {
        Self { accounts: vec![] }
    }

    // if the value is going to be taken, pass the value directly
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
        .iter()
        .map(|account| account.summary())
        .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("me"));

    account.deposit(500);
    account.withdraw(250);

    bank.add_account(account);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}
