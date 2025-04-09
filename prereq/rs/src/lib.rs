const RPC_URL: &str = "https://api.devnet.solana.com"; 

mod programs;

#[cfg(test)]
pub mod test {

    use solana_sdk;
    use solana_sdk::signature::{Keypair, Signer};
    use solana_client::rpc_client::RpcClient; 
    use solana_sdk::signature::read_keypair_file;
    use solana_program::system_instruction::transfer;
    use solana_sdk::transaction::Transaction;
    use crate::RPC_URL; 
    use solana_program::hash::hash;
    use solana_sdk::message::Message;

    use crate::programs::Turbin3_prereq::{TurbinePrereqProgram, CompleteArgs}; 

    use solana_program::system_program;
        
    #[test]
    fn keygen() {
        let keypair = Keypair::new();
        println!("You've generated a new Solana wallet: {}", keypair.pubkey().to_string());
        println!("{:?}", keypair.to_bytes());
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL); 

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) { 
            Ok(s) => { 
                println!("Success! Check out your TX here:"); 
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            }, 
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()) 
        };             
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let pubkey = keypair.pubkey(); 
        let message_bytes = b"I verify my solana Keypair!"; 
        let sig = keypair.sign_message(message_bytes); 
        let sig_hashed = hash(sig.as_ref()); 

        match sig.verify(&pubkey.to_bytes(), message_bytes) {
            true => println!("Signature verified"), 
            false => println!("Verification failed"),
        }

        let to_keypair = read_keypair_file("turbin3-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = to_keypair.pubkey();

        let rpc_client = RpcClient::new(RPC_URL); 

        let recent_blockhash = rpc_client .get_latest_blockhash() .expect("Failed to get recent blockhash"); 

        let transaction = Transaction::new_signed_with_payer( &[transfer( 
            &keypair.pubkey(), &to_pubkey, 1_000_000 
            )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash 
        );            

        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");

        println!( 
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature 
        ); 

        let balance = rpc_client 
        .get_balance(&keypair.pubkey()) 
        .expect("Failed to get balance"); 

        let message = Message::new_with_blockhash( 
            &[transfer( &keypair.pubkey(), &to_pubkey, balance, 
            )], Some(&keypair.pubkey()), &recent_blockhash 
        ); 

        let fee = rpc_client 
            .get_fee_for_message(&message) .expect("Failed to get fee calculator"); 
        
        let transaction = Transaction::new_signed_with_payer( 
            &[transfer( &keypair.pubkey(), &to_pubkey, balance - fee,
            )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash
        ); 

        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file"); 

        let prereq = TurbinePrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]); 
                
        let signature = rpc_client .send_and_confirm_transaction(&transaction) .expect("Failed to send transaction"); 

        println!("Success! Check out your TX here:https://explorer.solana.com/tx/{}/?cluster=devnet", signature); 

        let args = CompleteArgs {
            github: b"alizeeshan1234".to_vec() 
        }; 
            
        let blockhash = rpc_client .get_latest_blockhash() .expect("Failed to get recent blockhash"); 

        let transaction = TurbinePrereqProgram::complete( 
        &[&signer.pubkey(), &prereq, &system_program::id()], &args, Some(&signer.pubkey()), &[&signer], 
        blockhash ); 
        
        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction"); 

        println!("Success! Check out your TX here:https://explorer.solana.com/tx/{}/?cluster=devnet", signature); 

    }

}
