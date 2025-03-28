
#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Account {
        Account {id, balance: 0, holder}
    }

    fn holder(&self) -> String {
        self.holder.clone()
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>
}

impl Bank {
    fn new() -> Bank {
        Bank {
            accounts: vec![]
        }
    }
}

// use a reference to account
// if the function takes ownership of the variable
// it is deleted after it goes out of scope (after println! is executed)
fn print_account(account: &Account) {
    println!("{:#?}", account);
}

// this function needs to be called with a mutable reference
fn update_account(account: &mut Account) {
    account.balance = 10;
}


fn main() {
    let bank = Bank::new();
    let mut account = Account::new(1, String::from("me"));

    // some data types in rust (numbers, chars, arrays and tuples of base data types) are
    // copied instead of moved
    // here, the value of 'account.id' is copied to the variable 'id' instead of moved
    // and 'account' keeps being valid during the execution of the rest of the programm 
    let id = account.id;
    println!("{}", id);

    // multiple immutable references can be available at the same time
    let first_account_ref = &account;
    let second_account_ref = &account;

    // ERROR -> mutable references cannot be created while immutable references still exist
    // and only ONE mutable reference can exist at a time
    // let mutable_ref = &mut account;

    // ERROR -> bank cannot be moved to other_account as long as references to bank exist
    // let other_account = account;

    print_account(first_account_ref);
    print_account(second_account_ref);

    update_account(&mut account);
    print_account(&account);

    // other_bank now owns the value that was previously store in bank
    // meaning that tha value was moved to other_bank
    let other_bank = bank;
    println!("{:#?}", &other_bank);

    // ERROR -> bank doesn't own the value anymore
    // println!("{:#?}", bank);

}
