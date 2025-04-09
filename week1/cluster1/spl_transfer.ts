import { Commitment, Connection, Keypair, PublicKey } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
import { BN } from "@coral-xyz/anchor";
import wallet from "../wallet.json";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("AddvSPsJSLN1wrXFT71TAEvnoV2SHzPVFZPwrev2kgu7");

const toWalletAddress = new PublicKey("FEZhZZCPu7xTBwupZhCFHzjmhaJ8mVVtdrrLc7n1DquJ");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        let fromWallet = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey,
            true,
            commitment
        );

        // Get the token account of the toWallet address, and if it does not exist, create it
        let toWallet = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            toWalletAddress,
            true,
            commitment
        );

        const amount = new BN(10); // amount to transfer

        // Transfer the token to the recipient's token account
        let transferTx = await transfer(connection, keypair, fromWallet.address, toWallet.address, keypair, amount);
        console.log("Transaction Signature: ", transferTx);

        console.log("Transfer successful!");
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();
