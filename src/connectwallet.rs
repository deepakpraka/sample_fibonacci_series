#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u32,
    pub order_type: OrderType,
    pub amount: f64,
    pub price: f64,
}

#[derive(Debug)]
pub struct OrderBook {
    buy_orders: Vec<Order>,
    sell_orders: Vec<Order>,
    next_id: u32,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_order(&mut self, order_type: OrderType, amount: f64, price: f64) {
        let order = Order {
            id: self.next_id,
            order_type: order_type.clone(),
            amount,
            price,
        };

        match order_type {
            OrderType::Buy => self.buy_orders.push(order),
            OrderType::Sell => self.sell_orders.push(order),
        }

        self.next_id += 1;
    }
    pub fn show_order_book(&self) {
        println!("\n=== ORDER BOOK ===");

        println!("\n--- Buy Orders ---");
        if self.buy_orders.is_empty() {
            println!("No buy orders");
        } else {
            for order in &self.buy_orders {
                println!("ID: {} | Amount: {:.2} | Price: ${:.2}", order.id, order.amount, order.price);
            }
        }

        println!("\n--- Sell Orders ---");
        if self.sell_orders.is_empty() {
            println!("No sell orders");
        } else {
            for order in &self.sell_orders {
                println!("ID: {} | Amount: {:.2} | Price: ${:.2}", order.id, order.amount, order.price);
            }
        }

        println!("==================\n");
    }
    pub fn total_orders(&self) -> usize {
        self.buy_orders.len() + self.sell_orders.len()
    }
    pub fn get_orders_by_type(&self, order_type: &OrderType) -> &Vec<Order> {
        match order_type {
            OrderType::Buy => &self.buy_orders,
            OrderType::Sell => &self.sell_orders,
        }
    }
    pub fn find_order_by_id(&self, id: u32) -> Option<&Order> {
        self.buy_orders.iter().chain(&self.sell_orders).find(|o| o.id == id)
    }
    pub fn get_total_value_by_type(&self, order_type: &OrderType) -> f64 {
        self.get_orders_by_type(order_type)
            .iter()
            .map(|o| o.amount * o.price)
            .sum()
    }
}

pub fn order_book_demo() {
    println!("🚀 Order Book System Demo\n");

    let mut book = OrderBook::new();
    book.add_order(OrderType::Buy, 100.0, 50.25);
    book.add_order(OrderType::Buy, 200.0, 49.80);
    book.add_order(OrderType::Buy, 150.0, 51.00);
    book.add_order(OrderType::Sell, 75.0, 52.50);
    book.add_order(OrderType::Sell, 300.0, 53.20);
    book.add_order(OrderType::Sell, 125.0, 51.75);
    book.show_order_book();
    println!("📊 Order Book Stats");
    println!("Total Orders: {}", book.total_orders());
    println!("Buy Orders: {}", book.get_orders_by_type(&OrderType::Buy).len());
    println!("Sell Orders: {}", book.get_orders_by_type(&OrderType::Sell).len());
    println!("\n🔎 Searching for order ID 3...");
    if let Some(order) = book.find_order_by_id(3) {
        println!("Found: {:?} | Amount: {} | Price: ${:.2}", order.order_type, order.amount, order.price);
    } else {
        println!("Order not found.");
    }
    println!("\n💰 Total Order Values");
    println!("Buy Total:  ${:.2}", book.get_total_value_by_type(&OrderType::Buy));
    println!("Sell Total: ${:.2}", book.get_total_value_by_type(&OrderType::Sell));
    let buy_ref = book.get_orders_by_type(&OrderType::Buy);
    println!("\n📁 Buy Orders (reference): {} orders", buy_ref.len());
    println!("\n🧩 Matching Order Types:");
    for order_type in &[OrderType::Buy, OrderType::Sell] {
        match order_type {
            OrderType::Buy => println!("Buy:  {} orders", book.get_orders_by_type(order_type).len()),
            OrderType::Sell => println!("Sell: {} orders", book.get_orders_by_type(order_type).len()),
        }
    }

    println!("\n✅ Demo completed.");
}