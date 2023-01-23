use near_sdk::{ext_contract};

pub const TGAS: u64 = 1_000_000_000_000;
pub const MIN_DEPOSIT: u128 = 1;

// Validator interface, for cross-contract calls
#[ext_contract(ext_contract_lottery)]
pub trait Lottery {
    fn new (&mut self) -> String;
}

#[ext_contract(ext_self)]
pub trait NftContract {
    fn my_callback(&self) -> String;
    }



    #[ext_contract(hello_near)]
trait HelloNear {
  fn get_greeting(&self) -> String;
  fn set_greeting(&self, greeting: String);
}