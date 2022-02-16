mod model;
mod helper;
use crate::model::Model;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = Model::new("/home/hawk/temp_ledger/ledger.complete")?;
    model.print()?;
    Ok(())
}
