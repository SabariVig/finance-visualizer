use crate::handlers::LedgerResponse;
use chrono::NaiveDate;
use filetime::FileTime;
use ledger_parser::{Amount, Commodity, Ledger, LedgerItem, Posting, PostingAmount, Price};
use ledger_utils::{join_ledgers::join_ledgers, monthly_report::MonthlyReport};
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

    pub async fn monthly(&self, account: String) -> Vec<LedgerResponse> {
        let monthly_report = MonthlyReport::from(&self.ledger);
        let mut response_vec: Vec<LedgerResponse> = Vec::new();
        for reports in &monthly_report.monthly_balances {
            for (_name, amount) in reports
                .monthly_change
                .get_account_balance(&[&account])
                .amounts
                .iter()
            {
                response_vec.push(LedgerResponse {
                    date: NaiveDate::from_ymd(reports.year, reports.month, 1).to_string(),
                    amount: amount.quantity,
                    account: Some(account.to_string()),
                });
            }
        }
        response_vec
    }

    pub async fn cashflow(&self, account: String) -> Vec<LedgerResponse> {
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
                response_vec.push(LedgerResponse {
                    date: NaiveDate::from_ymd(reports.year, reports.month, 1).to_string(),
                    amount: sum + amount.quantity,
                    account: Some(account.to_string()),
                });
                sum = sum + amount.quantity;
            }
        }
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
                        && foreign_currency
                            .contains(&&posting_amount.amount.commodity.name.as_str())
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
}

impl Default for Model {
    fn default() -> Self {
        // TODO: Change env to LEDGER_FILE
        let path = env::var("LEDGER_FILE_DEV").unwrap();
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
