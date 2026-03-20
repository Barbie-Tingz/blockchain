mod account_info;
mod epoch_info;
mod recent_performance_samples;
mod blockchain_info;
mod transaction; 
mod signatures_for_address; 

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); 

    let wallet = dotenv::var("WALLET").expect("Cannot find WALLET"); // This reads you .env file 
    let client = reqwest::Client::new(); // creates the shared client 
    
    match blockchain_info::account_info(&client, &wallet).await {
        Ok(response) => println!("{:#?}", response), // matching on response
        Err(e) => eprintln!("Error: {}", e), // eprintln! prints to standard error output 
    }

    match blockchain_info::epoch_info(&client).await { 
        Ok(response) => println!("{:#?}", response), 
        Err(e) => eprintln!("Error: {}", e),
    }

    match blockchain_info::recent_performance_samples(&client, Some(3)).await { // Some(3) gives 3 samples 
        Ok(response) => println!("{:#?}", response), 
        Err(e) => eprintln!("Error: {}", e),
    }

    match blockchain_info::signatures_for_address(&client, &wallet).await {
        Ok(response) => {
            if let Some(first) = response.result.first() { //.first grabs the first item; Returns Option<&Signature> because it be empty
                // Some(first) checks if the block is not empty and then unwraps it. If empty it skips the block. 
                match blockchain_info::transaction(&client, &first.signature).await {
                    Ok(tx) => println!("{:#?}", tx), 
                    Err(e) => eprintln!("Error fetching transaction: {}", e),
                }
            }
        }
        Err(e) => println!("Error: {}", e), 
    }
}