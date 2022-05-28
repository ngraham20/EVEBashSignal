mod error;
use error::Error;

mod zkill;
use zkill::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let kmsb = KillMailStreamBuilder::builder()
    .character_id(12345)?
    .corporation_id(67890)?
    .build();
    Ok(())
}