import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { TokenLottery } from '../target/types/token_lottery';
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe('basic', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.TokenLottery as Program<TokenLottery>;

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');

  it('should init config', async () => {
    const initConfigIx = await program.methods.initializeConfig(
      new anchor.BN(0),
      new anchor.BN(1839882018),
      new anchor.BN(9000),
    ).instruction();

    const blockHashWithContext = await provider.connection.getLatestBlockhash();

    const tx = new anchor.web3.Transaction(
      {
      feePayer: provider.wallet.publicKey,
      blockhash: blockHashWithContext.blockhash,
      lastValidBlockHeight: blockHashWithContext.lastValidBlockHeight,
      }
    ).add(initConfigIx);

   // console.log("Transaction signature: ",tx);

    const signature = await anchor.web3.sendAndConfirmTransaction(provider.connection, tx, [wallet.payer], {skipPreflight: true});

    console.log("Transaction Signature: ", signature);
  });

  it("initializes lottery", async () => {
   
  });
  
});
