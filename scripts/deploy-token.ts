#!/usr/bin/env ts-node
/**
 * Quick deploy script - similar to Foundry's forge script
 *
 * Usage:
 *   npx ts-node scripts/deploy-token.ts
 *
 * Environment variables (like Foundry's .env):
 *   ANCHOR_PROVIDER_URL  - RPC URL
 *   ANCHOR_WALLET        - Wallet keypair file
 */

import { createMint, createAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { getOrCreateKeypair, loadKeypair } from "./utils";

const DEPLOYER_KEYPAIR_PATH = process.env.ANCHOR_WALLET || "~/.config/solana/id.json";

async function main() {
  // Load deployer keypair (like private key in Foundry)
  const deployer = loadKeypair(DEPLOYER_KEYPAIR_PATH);
  const connection = new Connection(process.env.ANCHOR_PROVIDER_URL || "http://localhost:8899");

  console.log("🚀 Deploying new SPL Token...");
  console.log("Deployer:", deployer.publicKey.toBase58());

  // ============================================================
  // CONFIG: Change these for your token
  // ============================================================
  const TOKEN_CONFIG = {
    name: "My Token",
    symbol: "MTK",
    decimals: 9,
    initialSupply: 1_000_000, // 1 million tokens
    mintable: true,           // Can mint more later?
    freezable: false,         // Can freeze accounts?
  };

  // Create the mint (like deploying ERC20 contract)
  const mintKeypair = Keypair.generate();
  console.log("📝 Mint Address:", mintKeypair.publicKey.toBase58());

  // Authorities
  const mintAuthority = TOKEN_CONFIG.mintable ? deployer.publicKey : null;
  const freezeAuthority = TOKEN_CONFIG.freezable ? deployer.publicKey : null;

  const mint = await createMint(
    connection,
    deployer,        // Payer (like gas)
    mintAuthority,   // Who can mint
    freezeAuthority, // Who can freeze
    TOKEN_CONFIG.decimals
  );

  console.log("✅ Token created!");

  // Mint initial supply to deployer
  const deployerTokenAccount = await createAssociatedTokenAccount(
    connection,
    deployer,
    mint,
    deployer.publicKey
  );

  const amount = TOKEN_CONFIG.initialSupply * 10**TOKEN_CONFIG.decimals;
  await mintTo(
    connection,
    deployer,
    mint,
    deployerTokenAccount,
    deployer,
    amount
  );

  console.log("💰 Initial supply minted:", TOKEN_CONFIG.initialSupply);

  // ============================================================
  // OUTPUT: Save these addresses!
  // ============================================================
  console.log("\n=== DEPLOYMENT COMPLETE ===");
  console.log("Token Mint Address:", mint.toBase58());
  console.log("Deployer Address:", deployer.publicKey.toBase58());
  console.log("Deployer Token Account:", deployerTokenAccount.toBase58());
  console.log("\n📋 Save this info:");
  console.log(JSON.stringify({
    mintAddress: mint.toBase58(),
    deployer: deployer.publicKey.toBase58(),
    tokenAccount: deployerTokenAccount.toBase58(),
    decimals: TOKEN_CONFIG.decimals,
    initialSupply: TOKEN_CONFIG.initialSupply,
  }, null, 2));
}

main().catch(console.error);
