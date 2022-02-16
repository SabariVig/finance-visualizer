use ledger_parser::{Amount, Commodity, Ledger, LedgerItem, Posting, PostingAmount, Price};
use rust_decimal::{Decimal, RoundingStrategy};
use std::{env, error::Error, fs::File, io::Read, path::Path};

pub struct Model {
    pub file: File,
    pub ledger: Ledger,
}

impl Model {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut file_string = String::new();
        file.read_to_string(&mut file_string)?;
        let ledger = ledger_parser::parse(&file_string)?;
        // ledger.convert_to_currency("INR")?;
        Ok(Model { file, ledger })
    }

    pub fn print(&self) -> Result<(), Box<dyn Error>> {
        println!("{:?}", self.ledger);
        Ok(())
    }
}

impl Default for Model {
    fn default() -> Self {
        // TODO: Change env to LEDGER_FILE
        let path = env::var("LEDGER_FILE_DEV").unwrap();
        Self::new(path).unwrap()
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
