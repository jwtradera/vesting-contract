import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TOKEN_PROGRAM_ID, getOrCreateAssociatedTokenAccount, getAssociatedTokenAddress, createMint, mintTo, mintToChecked, NATIVE_MINT, getAccount } from "@solana/spl-token";
import {
  SYSVAR_RENT_PUBKEY
} from "@solana/web3.js";
import { BN, min } from 'bn.js';
import { assert } from 'chai';

import { VestingContract } from "../target/types/vesting_contract";

const sleep = async (seconds) => {
  await new Promise(f => setTimeout(f, 1000 * seconds));
}


describe("vesting-contract", () => {

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.VestingContract as Program<VestingContract>;

  // Initial mint amount
  const MINT_A_AMOUNT = 1_000;

  // Create test keypairs
  const admin = anchor.web3.Keypair.generate();
  const payer = anchor.web3.Keypair.generate();
  const user1 = anchor.web3.Keypair.generate();
  const user2 = anchor.web3.Keypair.generate();
  const mintAuthority = anchor.web3.Keypair.generate();

  // Declare PDAs
  let pdaGlobalAccount, pdaRewardVault, pdaUser1Account, pdaUser2Account = null;

  // Declare nft mints
  let mint = null;

  const claim = async (user) => {

    // Get stake PDA
    const [pdaStakeAccount,] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("VESTING_LOCK"), user.publicKey.toBytes()], program.programId);

    // Get user's token associated account
    const userATA = await getOrCreateAssociatedTokenAccount(provider.connection, payer, mint, user.publicKey);

    const tx = await provider.connection.confirmTransaction(
      await program.rpc.claim(
        {
          accounts: {
            globalState: pdaGlobalAccount,
            stakeState: pdaStakeAccount,
            rewardVault: pdaRewardVault,
            userVault: userATA.address,
            authority: user.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY
          },
          signers: [user]
        })
    );

  }

  it('Initialize test accounts', async () => {
    // Airdrop sol to the test users
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(admin.publicKey, anchor.web3.LAMPORTS_PER_SOL),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, anchor.web3.LAMPORTS_PER_SOL),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user1.publicKey, anchor.web3.LAMPORTS_PER_SOL),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user2.publicKey, anchor.web3.LAMPORTS_PER_SOL),
      "confirmed"
    );
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(mintAuthority.publicKey, anchor.web3.LAMPORTS_PER_SOL),
      "confirmed"
    );

    // Create mint token with decimal 0
    mint = await createMint(provider.connection, payer, mintAuthority.publicKey, null, 0);

  });

  it('Initialize global account', async () => {

    [pdaGlobalAccount,] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("VESTING_GLOBAL")], program.programId);
    [pdaRewardVault,] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("REWARD_VAULT")], program.programId);

    // Test initialize instruction
    const tx = await provider.connection.confirmTransaction(
      await program.rpc.initialize(
        {
          accounts: {
            globalState: pdaGlobalAccount,
            rewardVault: pdaRewardVault,
            mint: mint,
            authority: admin.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY
          },
          signers: [admin]
        })
    );

    // Check global PDA after transaction
    const globalAccount = await program.account.globalState.fetch(pdaGlobalAccount);
    assert.equal(globalAccount.admin.toString(), admin.publicKey.toString());

    // Mint enough tokens to vault
    await mintToChecked(provider.connection, payer, mint, pdaRewardVault, mintAuthority, 10000, 0);
  })

  it('Test stake', async () => {

    const [pdaStakeAccount,] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("VESTING_LOCK"), user1.publicKey.toBytes()], program.programId);

    // Get user's token associated account
    const userATA = await getOrCreateAssociatedTokenAccount(provider.connection, payer, mint, user1.publicKey);

    // Mint tokens to user
    const STAKE_AMOUNT = 1000;
    await mintToChecked(provider.connection, payer, mint, userATA.address, mintAuthority, STAKE_AMOUNT, 0);

    // Stake 1000 TOKEN with 2 months of lock and 1 month of release schedule
    const tx = await provider.connection.confirmTransaction(
      await program.rpc.stake(
        new BN(STAKE_AMOUNT),
        new BN(2),
        new BN(1),
        {
          accounts: {
            globalState: pdaGlobalAccount,
            stakeState: pdaStakeAccount,
            rewardVault: pdaRewardVault,
            userVault: userATA.address,
            authority: user1.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY
          },
          signers: [user1]
        })
    );

    // Check stake PDA after transaction
    const stakeAccount = await program.account.stakeState.fetch(pdaStakeAccount);
    assert.equal(stakeAccount.amount.toNumber(), STAKE_AMOUNT);
  })

  it('Test claim', async () => {

    // Test claim user with lockup period
    try {
      await claim(user1);
    }
    catch (e) {
      const errorMsg = e.error.errorMessage;
      assert.equal(errorMsg, "Error: You need to wait at least lockup period.");
    }

    // Sleep 2 months for lockup period (as test, just 2 * 5 seconds)
    await sleep(5 * 2 + 1);

    // Expect claim success
    await claim(user1);

    // Check user's token balance
    const userATA = await getAssociatedTokenAddress(mint, user1.publicKey);
    const tokenAccount = await getAccount(provider.connection, userATA);
    console.log('Balance:', tokenAccount.amount);

  })

});
