use chrono::NaiveDate;
use ledger_parser::{Ledger, LedgerItem, Posting, Transaction};
use tokio::signal;

pub fn get_month_last_date(m: u32, year: i32) -> u32 {
    if m == 12 {
        NaiveDate::from_ymd(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd(year, m + 1, 1)
    }
    .signed_duration_since(NaiveDate::from_ymd(year, m, 1))
    .num_days()
    .try_into()
    .unwrap()
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}

pub fn new_transaction() -> Transaction {
    Transaction {
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
    }
}

