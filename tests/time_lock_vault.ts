import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TimeLockVault } from "../target/types/time_lock_vault";

describe("time_lock_vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider)
  const payer = provider.wallet.payer as anchor.web3.Keypair;

  const program = anchor.workspace.timeLockVault as Program<TimeLockVault>;
  const alice = anchor.web3.Keypair.generate();
  const bob = anchor.web3.Keypair.generate();
  const john = anchor.web3.Keypair.generate();

  async function airdrop(connection: any, address: any, amount = 100 * anchor.web3.LAMPORTS_PER_SOL) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
  }


  const getTreasurePda = (maker : anchor.web3.PublicKey)=>{
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("treasury"), maker.toBuffer()],
    program.programId
  );
}
  const getVaultPda = (maker : anchor.web3.PublicKey)=>{
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("treasury"), maker.toBuffer()],
    program.programId
  );
}
  const[aliceVaultPDA] = getVaultPda(alice.publicKey);
  const [treasuryPda] = getTreasurePda(payer.publicKey);
  it("Initialize Alice Vault With 10 sol!", async () => {
    await airdrop(provider.connection,alice.publicKey)
    console.log(await provider.connection.getBalance(alice.publicKey));

    await program
  });
});
