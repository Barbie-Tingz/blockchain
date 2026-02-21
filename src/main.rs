#[macro_use]
extern crate serde_derive; 

mod blockchain_info; 
mod epoch_info; 
mod account_info; 
mod recent_performance_samples; 

#[tokio::main]
async fn main() {
blockchain_info::account_info_request(&str);
}
