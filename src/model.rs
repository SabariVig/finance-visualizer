use chrono::{Local, NaiveTime};
use ledger_parser::{Amount, Commodity, Ledger, LedgerItem, Posting, PostingAmount, Price};
use ledger_utils::join_ledgers::join_ledgers;
use rust_decimal::{Decimal, RoundingStrategy};
use std::{env, error::Error, fs, path::Path};

pub struct Model {
    pub ledger: Ledger,
    pub prices: Option<Price>,
    path: String,
    modified: NaiveTime,
}

impl Model {
    pub fn new(path: impl AsRef<Path> + Copy) -> Result<Self, Box<dyn Error>> {
        let ledger_string = fs::read_to_string(path)?;
        let ledger = ledger_parser::parse(&ledger_string)?;
        let path = path
            .as_ref()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        Ok(Self {
            ledger,
            modified: Local::now().time(),
            path: path,
            prices: None,
        })
    }

    pub fn open(&self, path: impl AsRef<Path>) -> Result<Ledger, Box<dyn Error>> {
        let ledger_string = fs::read_to_string(path)?;
        let ledger = ledger_parser::parse(&ledger_string)?;
        let mut ledger_vec: Vec<Ledger> = Vec::new();
        for item in &ledger.items {
            if let LedgerItem::Include(include_path) = item {
                let new_path = format!("{}/{}", &self.path, include_path);
                let recursive_ledger = self.open(new_path)?;
                ledger_vec.push(recursive_ledger);
            }
        }
        ledger_vec.push(ledger);
        let final_ledger = join_ledgers(ledger_vec);
        Ok(final_ledger)
    }

    pub fn print(&self) {
        println!("{}", self.ledger);
    }
}

impl Default for Model {
    fn default() -> Self {
        // TODO: Change env to LEDGER_FILE
        let path = env::var("LEDGER_FILE_DEV").unwrap();
        Self::new(&path).unwrap()
    }
}

trait LedgerHelper {
    fn convert_to_currency(&mut self, name: &str) -> Result<(), Box<dyn Error>>;
}

impl LedgerHelper for Ledger {
    fn convert_to_currency(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        for item in &mut self.items {
            if let LedgerItem::Transaction(transaction) = item {
                for postings in transaction.postings.iter_mut() {
                    let posting_amount = postings
                        .amount
                        .as_ref()
                        .expect("Unable to get Posting Amount");
                    if posting_amount.amount.commodity.name != name {
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
                                        name: name.to_string(),
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
}
