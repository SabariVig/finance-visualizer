use crate::handlers::LedgerResponse;
use crate::utils::get_month_last_date;
use chrono::NaiveDate;
use filetime::FileTime;
use ledger_parser::{
    Amount, Commodity, Ledger, LedgerItem, Posting, PostingAmount, Price, Transaction,
};
use ledger_utils::{
    balance::Balance, join_ledgers::join_ledgers, monthly_report::MonthlyReport,
    tree_balance::TreeBalanceNode,
};
use rust_decimal::{Decimal, RoundingStrategy};
use std::{env, error::Error, fs, path::Path};

pub struct Model {
    pub ledger: Ledger,
    pub prices: Option<Price>,
    path: String,
    modified: FileTime,
}

impl Model {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let mut this = Self::default();
        let metadata = path.as_ref().metadata()?;
        let modified = FileTime::from_last_modification_time(&metadata);
        let ledger = this.open(&path)?;
        this = Self {
            ledger,
            modified,
            ..this
        };
        Ok(this)
    }

    pub fn open(&self, path: &impl AsRef<Path>) -> Result<Ledger, Box<dyn Error>> {
        let ledger_string = fs::read_to_string(path)?;
        let ledger = ledger_parser::parse(&ledger_string)?;
        let mut ledger_vec: Vec<Ledger> = Vec::new();
        let directory = &path
            .as_ref()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        for item in &ledger.items {
            if let LedgerItem::Include(include_path) = item {
                let new_path = format!("{}/{}", directory, include_path);
                ledger_vec.push(self.open(&new_path)?);
            }
        }
        ledger_vec.push(ledger);
        let final_ledger = join_ledgers(ledger_vec);
        Ok(final_ledger)
    }

    pub fn reload_file(&mut self) -> Result<(), Box<dyn Error>> {
        let path = Path::new(&self.path);
        let metadata = path.metadata()?;
        let modified = FileTime::from_last_modification_time(&metadata);

        if modified > self.modified {
            let model = Self::new(path)?;
            self.ledger = model.ledger;
            self.modified = model.modified;
        }
        Ok(())
    }

    pub async fn monthly(
        &mut self,
        account: String,
        convert_commodity: bool,
    ) -> Vec<LedgerResponse> {
        if convert_commodity {
            self.convert_to_currency("INR", vec!["*"]).unwrap(); // TODO Handel error later
        }
        self.sort_by_date();
        let monthly_report = MonthlyReport::from(&self.ledger);
        let mut response_vec: Vec<LedgerResponse> = Vec::new();
        for reports in &monthly_report.monthly_balances {
            for (_name, amount) in reports
                .monthly_change
                .get_account_balance(&[&account])
                .amounts
                .iter()
            {
                let date = get_month_last_date(reports.month, reports.year);
                response_vec.push(LedgerResponse {
                    date: Some(NaiveDate::from_ymd(reports.year, reports.month, date).to_string()),
                    amount: amount.quantity,
                    account: Some(account.to_string()),
                });
            }
        }
        response_vec
    }

    pub async fn cashflow(
        &mut self,
        account: String,
        convert_commodity: bool,
    ) -> Vec<LedgerResponse> {
        if convert_commodity {
            self.convert_to_currency("INR", vec!["*"]).unwrap(); // TODO Handel error later
        }
        self.sort_by_date();
        let monthly_report = MonthlyReport::from(&self.ledger);
        let mut response_vec: Vec<LedgerResponse> = Vec::new();
        let mut sum = Decimal::new(0, 0);
        for reports in &monthly_report.monthly_balances {
            for (_name, amount) in reports
                .monthly_change
                .get_account_balance(&[&account])
                .amounts
                .iter()
            {
                let date = get_month_last_date(reports.month, reports.year);
                response_vec.push(LedgerResponse {
                    date: Some(NaiveDate::from_ymd(reports.year, reports.month, date).to_string()),
                    amount: sum + amount.quantity,
                    account: Some(account.to_string()),
                });
                sum = sum + amount.quantity;
            }
        }
        response_vec
    }

    pub async fn balance(&mut self, account: String, convert_commodity: bool) -> LedgerResponse {
        if convert_commodity {
            self.convert_to_currency("INR", vec!["*"]).unwrap(); // TODO Handel error later
        }
        let balance = Balance::from(&self.ledger);
        let account_balance = balance.get_account_balance(&[&account]);
        LedgerResponse {
            date: None,
            amount: account_balance.amounts.get("INR").unwrap().quantity,
            account: Some(account),
        }
    }

    pub async fn split(&self, account_path: String) -> Vec<LedgerResponse> {
        let mut response_vec: Vec<LedgerResponse> = Vec::new();
        let balance = Balance::from(&self.ledger);
        let tree_balance = &TreeBalanceNode::from(balance);
        let split_tree = &tree_balance.children[&account_path];
        for (name, children) in &split_tree.children {
            response_vec.push(LedgerResponse {
                amount: children.balance.amounts["INR"].quantity,
                account: Some(name.to_string()),
                date: None,
            })
        }
        response_vec.sort_by(|a, b| b.amount.cmp(&a.amount));
        response_vec
    }

    pub fn convert_to_currency(
        &mut self,
        native_currency: &str,
        foreign_currency: Vec<&str>,
    ) -> Result<(), Box<dyn Error>> {
        for item in &mut self.ledger.items {
            if let LedgerItem::Transaction(transaction) = item {
                for postings in transaction.postings.iter_mut() {
                    let posting_amount = postings
                        .amount
                        .as_ref()
                        .expect("Unable to get Posting Amount");
                    if posting_amount.amount.commodity.name != native_currency
                        && (foreign_currency
                            .contains(&&posting_amount.amount.commodity.name.as_str())
                            || foreign_currency.contains(&"*"))
                    {
                        let commodity_prices = posting_amount
                            .price
                            .as_ref()
                            .expect("Unable to get Posting Price");
                        let mut quantity = Decimal::new(0, 0);
                        if let Price::Unit(commodity) = commodity_prices {
                            quantity = posting_amount.amount.quantity * commodity.quantity;
                        }
                        if let Price::Total(commodity) = commodity_prices {
                            quantity = commodity.quantity;
                        }
                        *postings = Posting {
                            amount: Some(PostingAmount {
                                amount: Amount {
                                    quantity: quantity.round_dp_with_strategy(
                                        2,
                                        RoundingStrategy::MidpointAwayFromZero,
                                    ),
                                    commodity: Commodity {
                                        name: native_currency.to_string(),
                                        position: posting_amount.amount.commodity.position,
                                    },
                                },
                                lot_price: None,
                                price: None,
                            }),
                            ..postings.clone()
                        };
                    }
                }
            }
        }
        Ok(())
    }

    pub fn print(&self) {
        println!("{}", self.ledger);
    }

    pub fn sort_by_date(&mut self) {
        let mut transactions = Vec::<Transaction>::new();
        for item in &self.ledger.items {
            match item {
                LedgerItem::Transaction(transaction) => {
                    transactions.push(transaction.clone());
                }
                _ => {}
            }
        }
        let _ = &self.ledger.items.sort_by(|a, b| {
            // HACK:
            let mut a_trans = Transaction {
                comment: None,
                date: NaiveDate::from_yo(2021, 01),
                effective_date: None,
                status: None,
                code: None,
                description: "Hello".to_string(),
                postings: vec![Posting {
                    account: "ABC".to_string(),
                    amount: None,
                    balance: None,
                    reality: ledger_parser::Reality::Real,
                    status: None,
                    comment: None,
                }],
            };
            let mut b_trans = a_trans.clone();
            if let LedgerItem::Transaction(trans) = a {
                a_trans = trans.clone();
            }
            if let LedgerItem::Transaction(trans) = b {
                b_trans = trans.clone();
            };
            a_trans.date.cmp(&b_trans.date)
        });
    }
}

impl Default for Model {
    fn default() -> Self {
        // TODO: Change env to LEDGER_FILE
        let path = env::var("LEDGER_FILE_DEV").unwrap();
        println!("{}", path);
        let metadata = Path::new(&path).metadata().unwrap();
        let modified = FileTime::from_last_modification_time(&metadata);

        Self {
            ledger: ledger_parser::parse("").unwrap(),
            modified,
            prices: None,
            path,
        }
    }
}

