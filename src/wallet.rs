use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Wallet {
    id: String,
    balance: u64,
}
trait WalletInfo {
    fn info(&self) -> String;
}

impl Wallet {
    pub fn create(balance: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let mut hasher = DefaultHasher::new();
        timestamp.hash(&mut hasher);
        let id = format!("wallet_{:x}", hasher.finish());

        Self { id, balance }
    }
    pub fn balance(&self) -> u64 {
        self.balance
    }
    pub fn send(&mut self, amount: u64) -> Result<(), String> {
        if self.balance < amount {
            return Err("Insufficient balance".into());
        }
        self.balance -= amount;
        Ok(())
    }
    pub fn transfer(&mut self, target: &mut Self, amount: u64) -> Result<(), String> {
        self.send(amount)?;
        target.balance += amount;
        Ok(())
    }
    pub fn total_balance(wallets: &[Self]) -> u64 {
        wallets.iter().map(|w| w.balance()).sum()
    }
}

impl WalletInfo for Wallet {
    fn info(&self) -> String {
        format!("Wallet [{}] Balance: {} ETH", self.id, self.balance)
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.info())
    }
}

pub fn wallet_example() {
    println!("✨ Wallet System\n");

    let mut w1 = Wallet::create(100);
    let mut w2 = Wallet::create(50);
    let w3 = Wallet::create(75);

    println!("Created wallets:");
    println!("  {}", w1);
    println!("  {}", w2);
    println!("  {}", w3);
    println!("\n✅ Checking balances:");
    println!("  w1 balance: {} ETH", w1.balance());
    println!("  w2 balance: {} ETH", w2.balance());

    println!("\n💸 Sending 30 ETH from w1...");
    match w1.send(30) {
        Ok(_) => println!("  Sent successfully."),
        Err(e) => println!("  Error: {}", e),
    }
    println!("  New w1 balance: {}", w1.balance());

    println!("\n🔁 Transferring 20 ETH from w1 to w2...");
    match w1.transfer(&mut w2, 20) {
        Ok(_) => {
            println!("  Transfer success!");
            println!("  w1: {} ETH, w2: {} ETH", w1.balance(), w2.balance());
        },
        Err(e) => println!("  Transfer failed: {}", e),
    }

    println!("\n📊 Calculating total...");
    let all_wallets = vec![w1.clone(), w2.clone(), w3.clone()];
    println!("  Total balance: {} ETH", Wallet::total_balance(&all_wallets));

    println!("\n🧬 Cloning wallet w1...");
    let backup = w1.clone();
    println!("  Backup: {}", backup);

    println!("\n📦 Ownership transfer of w3...");
    let w3_owner = w3; 
    println!("  New owner: {}", w3_owner);
}