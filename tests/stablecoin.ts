import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stablecoin } from "../target/types/stablecoin";
import { program } from "@coral-xyz/anchor/dist/cjs/native/system";
import { assert, use } from "chai";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";

const STABLECOIN_VAULT_SEED = "stablecoin_vault";
const STABLECOIN_MINT_SEED = "stablecoin_mint";
const STABLECOIN_STATE_SEED = "stablecoin";

describe("stablecoin", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.stablecoin as Program<Stablecoin>;

  const pythSolanaReceiver = new PythSolanaReceiver({ connection, wallet });
  const SOL_PRICE_FEED_ID =
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
  const solUsdPriceFeedAccount = pythSolanaReceiver
    .getPriceFeedAccountAddress(0, SOL_PRICE_FEED_ID)
    .toBase58();

  const admin = anchor.web3.Keypair.generate();
  const user = anchor.web3.Keypair.generate();

  const liquidationThreshold = new anchor.BN(80); // 80% of collateral value
  const liquidationBonus = new anchor.BN(5); // 5% bonus to liquidators

  // describe("Initialize Stablecoin Program", () =>
  let stablecoinAddress: anchor.web3.PublicKey;
  before(async () => {
    await airdrop(connection, admin.publicKey);
    [stablecoinAddress] = await getStablecoinAddress(program.programId);
    await airdrop(connection, user.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    [stablecoinAddress] = await getStablecoinAddress(program.programId);
  });
  it("Should initialize the program", async () => {
    try {
      const tx = await program.methods
        .initialize(liquidationThreshold, liquidationBonus)
        .accountsPartial({
          admin: admin.publicKey,
          stablecoinState: stablecoinAddress,
        })
        .signers([admin])
        .rpc();
      console.log("Transaction signature", tx);
    } catch (error) {
      console.log(error);
      assert.fail("Initialization failed");
    }

    const stablecoinState = await program.account.stablecoinState.fetch(
      stablecoinAddress
    );
    assert.equal(
      stablecoinState.admin.toBase58(),
      admin.publicKey.toBase58(),
      "Admin address mismatch"
    );
    assert.equal(
      Number(stablecoinState.liquidationThreshold),
      Number(liquidationThreshold),
      "Liquidation threshold mismatch"
    );
    assert.equal(
      Number(stablecoinState.liquidationBonus),
      Number(liquidationBonus),
      "Liquidation bonus mismatch"
    );
    assert.equal(stablecoinState.paused, false, "Program should be unpaused");
    console.log(
      "Stablecoin State account address:",
      stablecoinAddress.toBase58()
    );
  });

  it("Sol holder can deposit collateral and mint stablecoin", async () => {
    const solAmount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL); // 1 SOL
    const stablecoinAmount = new anchor.BN(50 * 10 ** 9); // 50 stablecoins with 9 decimals
    try {
      const tx = await program.methods
        .depositCollateralAndMintStablecoin(solAmount, stablecoinAmount)
        .accountsPartial({
          minter: user.publicKey,
          priceUpdate: solUsdPriceFeedAccount,
          stablecoinState: stablecoinAddress,
        })
        .signers([user])
        .rpc();
      console.log("Transaction signature", tx);
    } catch (error) {
      console.log(error);
      assert.fail("Deposit and mint failed");
    }
    const userAccount = await getUserAccount(
      user.publicKey,
      program.programId,
      program
    );
    assert.equal(
      userAccount.mintedStablecoins.toString(),
      stablecoinAmount.toString(),
      "Minted stablecoin amount mismatch"
    );
  });

  it("Stablecoin holder can burn stablecoin and withdraw collateral", async () => {
    const solAmount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL); // 1 SOL
    const stablecoinAmount = new anchor.BN(50 * 10 ** 9);
    try {
      const tx = await program.methods
        .redeemCollateralAndBurnStablecoin(solAmount, stablecoinAmount)
        .accountsPartial({
          user: user.publicKey,
          stablecoinState: stablecoinAddress,
          priceUpdate: solUsdPriceFeedAccount,
        })
        .signers([user])
        .rpc();
      console.log("Transaction signature", tx);
    } catch (error) {
      console.log(error);
      assert.fail("Burn and withdraw failed");
    }
  });

  it("Admin can toggle pause", async () => {
    try {
      const tx = await program.methods
        .togglePause()
        .accountsPartial({
          admin: admin.publicKey,
          stablecoinState: stablecoinAddress,
        })
        .signers([admin])
        .rpc();

      console.log("Transaction signature", tx);
    } catch (error) {
      console.log(error);
      assert.fail("Toggle pause failed");
    }
    const stablecoinState = await program.account.stablecoinState.fetch(
      stablecoinAddress
    );
    assert.equal(stablecoinState.paused, true, "Program should be paused");
  });

  // it("Should fail if liquidation threshold is out of range", async () => {
  //   let liquidationThreshold = new anchor.BN(150); // Invalid threshold
  //   try {
  //     await program.methods
  //       .initialize(liquidationThreshold, liquidationBonus)
  //       .accountsPartial({
  //         admin: admin.publicKey,
  //         stablecoinState: stablecoinAddress,
  //       })
  //       .signers([admin])
  //       .rpc();
  //     // throws error here
  //   } catch (error) {
  //     const err = await error.getLogs();
  //     // console.log(err);
  //     console.log(error);
  //   }
  // });
  // });
});

async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    "confirmed"
  );
}

async function getStablecoinAddress(programId: anchor.web3.PublicKey) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(STABLECOIN_STATE_SEED)],
    programId
  );
}

async function getUserAccount(
  user: anchor.web3.PublicKey,
  programId: anchor.web3.PublicKey,
  program: Program<Stablecoin>
) {
  let address = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user"), user.toBuffer()],
    programId
  );
  return program.account.userAccount.fetch(address[0]);
}
