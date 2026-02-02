import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  getAccount,
} from "@solana/spl-token";
import { expect } from "chai";
import { Anchortokenstarter } from "../target/types/anchortokenstarter";

describe("AnchorTokenStarter", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .Anchortokenstarter as Program<Anchortokenstarter>;
  const payer = provider.wallet as anchor.Wallet;

  // Derive mint PDA
  const [mintPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  );

  // Derive counter PDA
  const [counterPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("counter")],
    program.programId
  );

  describe("Basic Tests", () => {
    it("Initializes the program", async () => {
      const tx = await program.methods.initialize().rpc();
      console.log("Initialize transaction signature", tx);
    });

    it("Creates and increments a counter", async () => {
      // Increment the counter
      await program.methods
        .increment()
        .accounts({
          counter: counterPda,
          payer: payer.publicKey,
        })
        .rpc();

      // Fetch the counter account
      const counterAccount = await program.account.counter.fetch(counterPda);

      // Verify the count is 1
      expect(counterAccount.count.toString()).to.equal("1");

      // Increment again
      await program.methods
        .increment()
        .accounts({
          counter: counterPda,
          payer: payer.publicKey,
        })
        .rpc();

      // Fetch and verify count is 2
      const counterAccount2 = await program.account.counter.fetch(counterPda);
      expect(counterAccount2.count.toString()).to.equal("2");
    });
  });

  describe("SPL Token Tests", () => {
    const decimals = 9;

    it("Creates a new token mint", async () => {
      // Check if mint already exists
      const mintAccount = await provider.connection.getAccountInfo(mintPda);
      if (mintAccount) {
        console.log("Mint already exists, skipping creation");
        return;
      }

      const tx = await program.methods
        .createMint(decimals)
        .accounts({
          payer: payer.publicKey,
          mint: mintPda,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();

      console.log("Create mint transaction:", tx);
    });

    it("Mints tokens to an associated token account", async () => {
      const mintAmount = new anchor.BN(1_000_000_000); // 1 token with 9 decimals

      // Derive the associated token account
      const associatedTokenAccount = await getAssociatedTokenAddress(
        mintPda,
        payer.publicKey
      );

      const tx = await program.methods
        .mintTokens(mintAmount)
        .accounts({
          signer: payer.publicKey,
          mint: mintPda,
          tokenAccount: associatedTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("Mint tokens transaction:", tx);

      // Verify balance
      const tokenAccount = await getAccount(
        provider.connection,
        associatedTokenAccount
      );

      expect(Number(tokenAccount.amount)).to.equal(mintAmount.toNumber());
    });

    it("Transfers tokens between accounts", async () => {
      // Create a recipient keypair
      const recipient = Keypair.generate();

      // Airdrop SOL to recipient for rent
      const airdropTx = await provider.connection.requestAirdrop(
        recipient.publicKey,
        2 * LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(airdropTx);

      // Get ATAs
      const senderATA = await getAssociatedTokenAddress(
        mintPda,
        payer.publicKey
      );

      const recipientATA = await getAssociatedTokenAddress(
        mintPda,
        recipient.publicKey
      );

      // Create the recipient's ATA first
      const createAtaTx = await program.methods
        .mintTokens(new anchor.BN(0))
        .accounts({
          signer: recipient.publicKey,
          mint: mintPda,
          tokenAccount: recipientATA,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([recipient])
        .rpc();

      // Get initial balances
      const initialSenderBalance = (
        await getAccount(provider.connection, senderATA)
      ).amount;

      const transferAmount = new anchor.BN(100_000_000); // 0.1 tokens

      const tx = await program.methods
        .transferTokens(transferAmount)
        .accounts({
          signer: payer.publicKey,
          mint: mintPda,
          senderTokenAccount: senderATA,
          recipientTokenAccount: recipientATA,
          recipient: recipient.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();

      console.log("Transfer transaction:", tx);

      // Verify balances
      const finalSenderBalance = (
        await getAccount(provider.connection, senderATA)
      ).amount;

      const finalRecipientBalance = (
        await getAccount(provider.connection, recipientATA)
      ).amount;

      expect(Number(finalSenderBalance)).to.equal(
        Number(initialSenderBalance) - transferAmount.toNumber()
      );
      expect(Number(finalRecipientBalance)).to.equal(transferAmount.toNumber());
    });
  });
});
