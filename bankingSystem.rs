trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!(
                "Deposited ${:.2} into account {}. New balance: ${:.2}",
                amount, self.account_number, self.balance
            );
        } else {
            println!("Deposit amount must be positive.");
        }
    }

    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 {
            if self.balance >= amount {
                self.balance -= amount;
                println!(
                    "Withdrew ${:.2} from account {}. New balance: ${:.2}",
                    amount, self.account_number, self.balance
                );
            } else {
                println!(
                    "Insufficient funds. Account {} has a balance of ${:.2}",
                    self.account_number, self.balance
                );
            }
        } else {
            println!("Withdraw amount must be positive.");
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut account1 = BankAccount {
        account_number: 12345,
        holder_name: "Alice Smith".to_string(),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: 67890,
        holder_name: "Bob Johnson".to_string(),
        balance: 500.0,
    };

    // Deposit into account1
    account1.deposit(200.0);

    // Withdraw from account2
    account2.withdraw(150.0);

    // Display balances
    println!(
        "Account {} balance: ${:.2}",
        account1.account_number,
        account1.balance()
    );
    println!(
        "Account {} balance: ${:.2}",
        account2.account_number,
        account2.balance()
    );
}
