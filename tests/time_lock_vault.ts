import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TimeLockVault } from "../target/types/time_lock_vault";
import { Connection } from "@solana/web3.js";
import { expect } from "chai";


describe("time_lock_vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider)
  const payer = provider.wallet.payer as anchor.web3.Keypair;

  const program = anchor.workspace.timeLockVault as Program<TimeLockVault>;
  const alice = anchor.web3.Keypair.generate();
  const bob = anchor.web3.Keypair.generate();

  async function fundWallet(recipient: anchor.web3.PublicKey, amount: number) {
    const tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: payer.publicKey,
        toPubkey: recipient,
        lamports: amount * anchor.web3.LAMPORTS_PER_SOL,
      })
    );
    await provider.sendAndConfirm(tx, [payer]);
  }
  const RETURN_WALLET = new anchor.web3.PublicKey("4ncsvGw6AuXFjgA328JaZHkjzNHTWLHw9yZ8A9JTqZ5n");

  async function drainWallet(kp: anchor.web3.Keypair) {
  const connection = provider.connection;
  const balance = await connection.getBalance(kp.publicKey);

  if (balance <= 5000) return;

  const { blockhash, lastValidBlockHeight } =
    await connection.getLatestBlockhash();

  const tx = new anchor.web3.Transaction({
    feePayer: kp.publicKey,
    blockhash,
    lastValidBlockHeight,
  }).add(
    anchor.web3.SystemProgram.transfer({
      fromPubkey: kp.publicKey,
      toPubkey: RETURN_WALLET,
      lamports: balance - 5000,
    })
  );

  tx.sign(kp);

  const sig = await connection.sendRawTransaction(tx.serialize());
    await connection.confirmTransaction({
      signature: sig,
      blockhash,
      lastValidBlockHeight,
    });
  }

  async function returnAllSol() {
    console.log("Draining test wallets...");

    await drainWallet(alice);
    await drainWallet(bob);

    console.log("All test wallets drained.");
  }

  function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }


  const getTreasurePda = ()=>{
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("treasury")],
    program.programId
  );
}
  const getTreasureWalletPda = (maker : any)=>{
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("treasury_wallet"),maker.toBuffer()],
    program.programId
  );
}
  const getVaulWalletPda = (maker : anchor.web3.PublicKey)=>{
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault_wallet"),maker.toBuffer()],
    program.programId
  );
}
  const getVaultPda = (maker : anchor.web3.PublicKey)=>{
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), maker.toBuffer()],
    program.programId
  );
}
  const[aliceVaultPDA] = getVaultPda(alice.publicKey);
  const[aliceVaultWalletPda] = getVaulWalletPda(alice.publicKey);
  const[bobVaultPDA] = getVaultPda(bob.publicKey);
  const[bobVaultWalletPDA] = getVaulWalletPda(bob.publicKey);
  const [treasuryPda] = getTreasurePda();
  const [treasuryWalletPDA] = getTreasureWalletPda(treasuryPda);
  it("1) Initialize Alice Vault With 1 sol!", async () => {
    await fundWallet(alice.publicKey,2)
    const depositAmount = new anchor.BN(1_000_000_000);//1 sol = 1 x 10^9 lamports
    const duration = new anchor.BN(5);

    const tx = await program.methods.initializeVault(
      depositAmount,
      duration
    )
    .accountsStrict({
      user : alice.publicKey,
      vault : aliceVaultPDA,
      vaultWallet : aliceVaultWalletPda,
      systemProgram : anchor.web3.SystemProgram.programId
    })
    .signers([alice])
    .rpc()

    const result = await program.account.vault.fetch(aliceVaultPDA);
    const testingDuration = result.maturityTime.sub(result.depositTime);
    expect(testingDuration.toNumber()).to.equal(5);
  });
  it("2) Initialize Bob Vault with 1 sol!",async ()=>{
    await fundWallet(bob.publicKey,2)
    const depositAmount = new anchor.BN(1_000_000_000);//1 sol = 1 x 10^9 lamports
    const duration = new anchor.BN(10);
    const tx = await program.methods.initializeVault(depositAmount,duration).accountsStrict({
      user : bob.publicKey,
      vault : bobVaultPDA,
      vaultWallet : bobVaultWalletPDA,
      systemProgram : anchor.web3.SystemProgram.programId
    }).signers([bob]).rpc();
    const vault = await program.account.vault.fetch(bobVaultPDA);
    expect(vault.amount.toNumber()).to.equal(1_000_000_000);

    const actualDuration = vault.maturityTime.sub(vault.depositTime);
    expect(actualDuration.toNumber()).to.equal(10);
  })
  it("3) Initialize Treasury for Penalties",async ()=>{
    const tx = await program.methods.initializeTreasury().accountsStrict({
      user : payer.publicKey,
      treasury : treasuryPda,
      treasuryWallet : treasuryWalletPDA,
      systemProgram : anchor.web3.SystemProgram.programId
    }).signers([payer]).rpc()
    const treasury = await program.account.treasury.fetch(treasuryPda);
    expect(treasury.authority.toBase58()).to.eq(payer.publicKey.toBase58());
  })
  it("4) Withdraw : Alice Vault Early(Penalty 10%)",async()=>{
    const aliceBefore = await provider.connection.getBalance(alice.publicKey);
    const tx = await program.methods.withdrawAndCloseVault().accountsStrict({
      user : alice.publicKey,
      vault : aliceVaultPDA,
      treasury : treasuryPda,
      treasuryWallet : treasuryWalletPDA,
      vaultWallet : aliceVaultWalletPda,
      systemProgram : anchor.web3.SystemProgram.programId
    }).signers([alice]).rpc()

    const aliceAfter = await provider.connection.getBalance(alice.publicKey);
    const treasury = await program.account.treasury.fetch(treasuryPda);
    expect(treasury.totalPenalties.toNumber()).to.equal(100_000_000);

    const expectedReturn = 900_000_000;
    const received = aliceAfter - aliceBefore;
    expect(received).to.be.closeTo(expectedReturn, 5000000); 
  })
  it("5) Withdraw : Bob (After Maturity Time)",async()=>{
    console.log("Waiting 11 seconds...");
    await sleep(11000); // 11 seconds
    const before = await provider.connection.getBalance(bob.publicKey);
    const tx = await program.methods.withdrawAndCloseVault().accountsStrict({
      user : bob.publicKey,
      vault : bobVaultPDA,
      vaultWallet : bobVaultWalletPDA,
      treasury : treasuryPda,
      treasuryWallet : treasuryWalletPDA,
      systemProgram : anchor.web3.SystemProgram.programId
    }).signers([bob]).rpc();
    const after = await provider.connection.getBalance(bob.publicKey);
    expect(after).to.be.greaterThan(before);
  })
  it("6) Withdraw : Treasury to Owner", async()=>{
    const beforeBalance = await provider.connection.getBalance(payer.publicKey);
    const tx = await program.methods.withdrawAndCloseTreasury().accountsStrict({
      user: payer.publicKey,
      treasury : treasuryPda,
      treasuryWallet : treasuryWalletPDA,
      systemProgram : anchor.web3.SystemProgram.programId
    }).signers([payer]).rpc();
    const afterBalance = await provider.connection.getBalance(payer.publicKey);
    expect(afterBalance).to.be.greaterThan(beforeBalance);

    const treasuryAccount = await provider.connection.getAccountInfo(treasuryPda);
    expect(treasuryAccount).to.be.null;

    const treasuryWalletBalance = await provider.connection.getBalance(treasuryWalletPDA);
    expect(treasuryWalletBalance).to.equal(0);
  })
  after(async () => {
    await returnAllSol();
  });
});

