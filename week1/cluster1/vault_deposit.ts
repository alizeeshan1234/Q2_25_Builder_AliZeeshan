import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  Commitment,
} from "@solana/web3.js";
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
  BN,
} from "@coral-xyz/anchor";
import { WbaVault, IDL } from "./programs/wba_vault";
import wallet from "../wallet.json";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Commitment
const commitment: Commitment = "finalized";

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment,
});

// Create our program
const program = new Program<WbaVault>(IDL,  "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address, provider);

// Create a random keypair
const vaultState = Keypair.generate();
console.log(vaultState.publicKey);
// Create the PDA for our enrollment account
// const vaultAuth = ???

let vaultAuth: PublicKey;

[vaultAuth] = PublicKey.findProgramAddressSync(
  [Buffer.from("auth"), vaultState.publicKey.toBuffer()],
  program.programId,
);

console.log(vaultAuth);

// Create the vault key
// const vault = ???
let vault: PublicKey;

[vault] = PublicKey.findProgramAddressSync(
  [Buffer.from("vault"), vaultAuth.toBuffer()],
  program.programId
);

console.log(vault);

// Execute our enrollment transaction
(async () => {
  try {
    
    const depositAmount = new BN(100);

    const initialize = await program.methods.initialize().accounts({
      owner: keypair.publicKey,
      vaultState: vaultState.publicKey,
      vaultAuth,
      vault,
      systemProgram: SystemProgram.programId,
    }).signers([keypair]).rpc();

    console.log(`Transction Signature: ${initialize}`);

    const signature = await program.methods
    .deposit(depositAmount)
    .accounts({
      owner: keypair.publicKey,
      vaultState: vaultState.publicKey,
      vaultAuth: vaultAuth,
      vault: vault.toBase58(),
      systemProgram: SystemProgram.programId,
    }).signers([keypair, vaultState]).rpc();
    console.log(`Deposit success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
