// 设计一家银行

// 背景
// 银行提供多种金融服务，包括支票和储蓄账户、信用卡和贷款。客户通常必须在使用任何服务之前在银行开设账户。
// 客户可以存款或取款，甚至购买投资。

// 要求
// 一些可能要问的问题：
// 1.银行将提供哪些金融服务？
// 2.客户需要拥有账户吗？银行会管理它们吗？
// 3.银行有实体地点和银行柜员吗？
// 4.我们关心银行的物理安全吗？即银行有金库吗？

// 服务
// 1.客户可以开设账户和存款/取款
// 2.我们只关心在物理位置内发生的交易（即通过银行柜员）

// 出纳员(Tellers)
//      柜员可以代表客户进行交易
//          每笔交易都会被记录并与出纳员和客户相关联

// 总部
//      每个分行地点将在一天结束时将资金汇至中央地点（即银行总部）
//          我们无需担心交通细节

// 设计
// 顶层设计
// 1.我们将有一个基本 Transaction 类，该类将由 Deposit(存钱)、Withdrawal(取钱) 和 OpenAccount(开户) 类继承。
// 2.BankTeller 将简单地封装出纳员的唯一 ID。我们不需要客户的类，因为我们可以使用 BankAccount 类来封装他们的 ID 和余额。
// 3.总部银行将由多个 BankBranch(分行) 对象和一个 BankSystem 组成，该 BankSystem 将成为客户帐户和交易的中央存储。
// 4.请注意，客户可以与多个分行进行交易，因此我们需要将他们的信息存储在银行系统中。

use std::{cell::RefCell, rc::Rc};

use rand::Rng;

struct Transaction {
    // 用户ID
    customer_id: usize,
    // 柜员ID(银行开户是有柜员带用户开户)
    teller_id: usize,
}

impl Transaction {
    fn new(customer_id: usize, teller_id: usize) -> Self {
        Self {
            customer_id,
            teller_id,
        }
    }
}

pub trait TransactionDescription {
    fn get_transaction_description(&self) -> String;
}

/// Deposit 存款
pub struct Deposit {
    transaction: Transaction,
    amount: usize,
}

impl Deposit {
    pub fn new(customer_id: usize, teller_id: usize, amount: usize) -> Self {
        Self {
            transaction: Transaction::new(customer_id, teller_id),
            amount,
        }
    }
}

impl TransactionDescription for Deposit {
    fn get_transaction_description(&self) -> String {
        format!(
            "Teller {} deposited {} to account {}",
            self.transaction.teller_id, self.amount, self.transaction.customer_id
        )
    }
}

/// Withdrawal 取款
pub struct Withdrawal {
    transaction: Transaction,
    amount: usize,
}

impl Withdrawal {
    pub fn new(customer_id: usize, teller_id: usize, amount: usize) -> Self {
        Self {
            transaction: Transaction::new(customer_id, teller_id),
            amount,
        }
    }
}

impl TransactionDescription for Withdrawal {
    fn get_transaction_description(&self) -> String {
        format!(
            "Teller {} withdraw {} from account {}",
            self.transaction.teller_id, self.amount, self.transaction.customer_id
        )
    }
}

pub struct OpenAccount {
    transaction: Transaction,
}

impl OpenAccount {
    pub fn new(customer_id: usize, teller_id: usize) -> Self {
        Self {
            transaction: Transaction::new(customer_id, teller_id),
        }
    }
}

impl TransactionDescription for OpenAccount {
    fn get_transaction_description(&self) -> String {
        format!(
            "Teller {} opened account {} ",
            self.transaction.teller_id, self.transaction.customer_id
        )
    }
}

/// BankTeller 银行柜员
pub struct BankTeller {
    pub id: usize,
}

impl BankTeller {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}
/// BankAccount 银行账户
pub struct BankAccount {
    // 用户ID
    customer_id: usize,
    // 用户名
    name: String,
    // 余额
    balance: usize,
}

impl BankAccount {
    pub fn new(customer_id: usize, name: String, balance: usize) -> Self {
        Self {
            customer_id,
            name,
            balance,
        }
    }

    pub fn get_balance(&self) -> usize {
        self.balance
    }

    // 存钱
    pub fn deposit(&mut self, amount: usize) {
        // wrapping_add 避免相加溢出(溢出了会从0开始)
        self.balance = self.balance.wrapping_add(amount);
    }

    // 取钱
    pub fn withdraw(&mut self, amount: usize) {
        self.balance = self.balance.wrapping_sub(amount);
    }
}

pub struct BankSystem {
    accounts: Vec<BankAccount>,
    transactions: Vec<Box<dyn TransactionDescription>>,
}

impl BankSystem {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
            transactions: Vec::new(),
        }
    }

    pub fn get_accounts(&self) -> &Vec<BankAccount> {
        &self.accounts
    }

    pub fn get_account(&self, customer_id: usize) -> Option<&BankAccount> {
        self.accounts.iter().find(|a| a.customer_id == customer_id)
    }

    pub fn get_account_mut(&mut self, customer_id: usize) -> Option<&mut BankAccount> {
        self.accounts
            .iter_mut()
            .find(|a| a.customer_id == customer_id)
    }

    pub fn get_transactions(&self) -> &Vec<Box<dyn TransactionDescription>> {
        &self.transactions
    }

    pub fn open_account(&mut self, customer_name: String, teller_id: usize) -> usize {
        // Create account
        let customer_id = self.accounts.len() + 1; // id 为用户数+1
        let account = BankAccount::new(customer_id, customer_name, 0);
        self.accounts.push(account);

        // Log transaction
        let ts = OpenAccount::new(customer_id, teller_id);
        self.transactions.push(Box::new(ts));
        customer_id
    }

    pub fn deposit(&mut self, customer_id: usize, teller_id: usize, amount: usize) {
        let Some(account) = self.get_account_mut(customer_id) else {
            return
        };
        account.deposit(amount);

        let ts = Deposit::new(customer_id, teller_id, amount);
        self.transactions.push(Box::new(ts));
    }

    pub fn withdraw(&mut self, customer_id: usize, teller_id: usize, amount: usize) {
        let Some(account) = self.get_account_mut(customer_id) else {
            return
        };
        // 取出的钱比该用户的余额还多，不执行
        if amount > account.balance {
            return;
        }

        account.withdraw(amount);
        let ts = Withdrawal::new(customer_id, teller_id, amount);
        self.transactions.push(Box::new(ts));
    }
}

/// BankBranch 分行
pub struct BankBranch {
    // 分行地址
    address: String,
    // 分行持有的现金
    cash_on_hand: usize,
    bank_system: Rc<RefCell<BankSystem>>,
    // 分行柜员
    tellers: Vec<BankTeller>,
}

impl BankBranch {
    pub fn new(address: String, cash_on_hand: usize, bank_system: Rc<RefCell<BankSystem>>) -> Self {
        Self {
            address,
            cash_on_hand,
            bank_system,
            tellers: Vec::new(),
        }
    }

    // 添加柜员
    pub fn add_teller(&mut self, teller: BankTeller) {
        self.tellers.push(teller);
    }

    // 开户
    pub fn open_account(&mut self, customer_name: String) -> usize {
        if self.tellers.is_empty() {
            return 0;
        }

        let teller = self.get_available_teller();
        self.bank_system
            .borrow_mut()
            .open_account(customer_name, teller.id)
    }

    // 存钱
    pub fn deposit(&mut self, customer_id: usize, amount: usize) {
        if self.tellers.is_empty() {
            return;
        }

        let teller = self.get_available_teller();
        self.bank_system
            .borrow_mut()
            .deposit(customer_id, teller.id, amount);
    }

    // 取钱
    pub fn withdraw(&mut self, customer_id: usize, amount: usize) {
        // 查看该分行现金是否足够
        if amount > self.cash_on_hand {
            return;
        }
        if self.tellers.is_empty() {
            return;
        }
        self.cash_on_hand = self.cash_on_hand.wrapping_sub(amount);
        let teller = self.get_available_teller();
        self.bank_system
            .borrow_mut()
            .withdraw(customer_id, teller.id, amount);
    }

    // 总行抽走分行现金(按百分比)
    pub fn collect_cash(&mut self, ratio: f64) -> usize {
        let cash_to_collect = (self.cash_on_hand as f64 * ratio).round() as usize;
        self.cash_on_hand = self.cash_on_hand.wrapping_div(cash_to_collect);
        cash_to_collect
    }

    // 随机找一个柜员
    fn get_available_teller(&self) -> &BankTeller {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.tellers.len());
        &self.tellers[idx]
    }
}

/// Bank 银行
pub struct Bank {
    // 记录分行
    branches: Vec<Rc<RefCell<BankBranch>>>,

    bank_system: Rc<RefCell<BankSystem>>,

    // 银行总现金
    total_cash: usize,
}

impl Bank {
    pub fn new(total_cash: usize) -> Self {
        Self {
            branches: Vec::new(),
            bank_system: Rc::new(RefCell::new(BankSystem::new())),
            total_cash,
        }
    }

    // 添加分行, initial_funds 初始化基金
    pub fn add_branch(&mut self, address: String, initial_funds: usize) -> Rc<RefCell<BankBranch>> {
        let branch = BankBranch::new(address, initial_funds, Rc::clone(&self.bank_system));
        let branch = Rc::new(RefCell::new(branch));
        self.branches.push(Rc::clone(&branch));
        branch
    }

    // 收集各个分行的存款
    pub fn collect_cash(&mut self, ratio: f64) {
        for branch in &self.branches {
            let cash_collected = branch.borrow_mut().collect_cash(ratio);
            self.total_cash = self.total_cash.wrapping_add(cash_collected);
        }
    }

    pub fn print_transactions(&self) {
        for transaction in self.bank_system.borrow().get_transactions() {
            println!("{}", transaction.get_transaction_description());
        }
    }
}

fn main() {
    let mut bank = Bank::new(10000);
    let branch1 = bank.add_branch("123 Main St".to_string(), 1000);
    let branch2 = bank.add_branch("456 Elm St".to_string(), 1000);

    branch1.borrow_mut().add_teller(BankTeller::new(1));
    branch1.borrow_mut().add_teller(BankTeller::new(2));
    branch2.borrow_mut().add_teller(BankTeller::new(3));
    branch2.borrow_mut().add_teller(BankTeller::new(4));

    let customer_id1 = branch1.borrow_mut().open_account("John Doe".to_string());
    let customer_id2 = branch1.borrow_mut().open_account("Bob Smith".to_string());
    let customer_id3 = branch1.borrow_mut().open_account("Jane Doe".to_string());

    branch1.borrow_mut().deposit(customer_id1, 100);
    branch1.borrow_mut().deposit(customer_id2, 200);
    branch2.borrow_mut().deposit(customer_id3, 300);
    branch1.borrow_mut().withdraw(customer_id1, 50);
    bank.print_transactions();
    // Possible Output:
    // Teller 1 opened account 1
    // Teller 2 opened account 2
    // Teller 2 opened account 3
    // Teller 2 deposited 100 to account 1
    // Teller 2 deposited 200 to account 2
    // Teller 4 deposited 300 to account 3
    // Teller 1 withdraw 50 from account 1
    bank.collect_cash(0.5);
}
