mod programs;
mod prereqs;
mod cluster1;

#[cfg(test)]
mod tests {
    

    use solana_client::{rpc_client::RpcClient};
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{message::Message, signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction, system_program};
    use bs58;
    use std::io::{self, BufRead};
    use std::str::FromStr;
    use std::env;
    use dotenv::dotenv; 

    #[test]
    fn test_turbin3_prereq() {
        use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs, UpdateArgs};

        // Load environment variables from '.env'
        dotenv().ok();

        // Get RPC_URL from .env
        let rpc_url = env::var("RPC_URL").expect("RPC_URL not set in .env");

        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(rpc_url);

        // Let's define our accoutns
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

        // Create PDA for our prereq account
        let prereq = Turbin3PrereqProgram::derive_program_address(
            &[
                b"prereq",
                &signer.pubkey().to_bytes().as_ref()
             ]
        );

        // Define our instruction data
        let args = CompleteArgs{
            github:b"nathandanielanderson".to_vec()
        };

        // Get recent blockhash
        let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

        // Invoke "complete" function
        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::ID], // Instruction
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash
        );

        // Send the transaction
        let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

         // Print our transaction out
         println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);

    }

    fn keygen() {

        // Create a new keypair
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); 
        println!(""); 
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());

    } 
    
    #[test]
    fn base58_to_wallet() {

        println!("Input your private key as base58:");
        let stdin = io::stdin();

        let base58 = stdin.lock().lines().next().unwrap().unwrap();

        println!("Yuour wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);

    }

    #[test]
    fn wallet_to_base58() {

        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();

        let wallet = 
        stdin.lock().lines().next().unwrap().unwrap()
        .trim_start_matches('[').trim_end_matches(']').split(',') 
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);

    }

    #[test] 
    fn airdrop() {
        // Load environment variables from '.env'
        dotenv().ok();

        // Get RPC_URL from .env
        let rpc_url = env::var("RPC_URL").expect("RPC_URL not set in .env");

        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file"); 

        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(rpc_url);

        // Claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {

            Ok(s) => {

                println!("Success! Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());

            },

            Err(e) => { println!("Oops, something went wrong: {}", e.to_string()); }
        }

    } 
    
    #[test] 
    fn transfer_sol() {

        // Load environment variables from '.env'
        dotenv().ok();

        // Get RPC_URL from .env
        let rpc_url = env::var("RPC_URL").expect("RPC_URL not set in .env");

        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(rpc_url);

        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        
        // Define our Turbin3 public key
        let to_pubkey = Pubkey::from_str("B7XdPS3HfCFt5RS3RH1t8xeH6XsaFL8Hj8wdWhNSLkWe").unwrap();

        // Get recent blockhash
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)], // Instructions
            Some(&keypair.pubkey()), // Payer
            &vec![&keypair], // Signers
            recent_blockhash // Recent blockhash
            ); 

        // Send the transaction
        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
        
        // Print our transaction out
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
        
    }

    #[test]
    fn dump() {

        // Load environment variables from '.env'
        dotenv().ok();

        // Get RPC_URL from .env
        let rpc_url = env::var("RPC_URL").expect("RPC_URL not set in .env");

        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // Define our Turbin3 public key
        let to_pubkey = Pubkey::from_str("B7XdPS3HfCFt5RS3RH1t8xeH6XsaFL8Hj8wdWhNSLkWe").unwrap();

        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(rpc_url);

        // Get recent blockhash
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");


        // Get balance of dev wallet
        let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");

        // Create a test transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)], // Instructions
            Some(&keypair.pubkey()), // Payer
            &recent_blockhash // Recent blockhash
        );

        // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
        let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");

        // Deduct fee from lamports amount and create a TX with correct balance
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)], // Instructions
            Some(&keypair.pubkey()), // Payer
            &vec![&keypair], // Signers
            recent_blockhash // Recent blockhash
        );
        
        // Send the transaction
        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
        
        // Print our transaction out
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);

    }
}
