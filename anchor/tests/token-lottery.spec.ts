import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { TokenLottery } from '../target/types/token_lottery';

describe('basic', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.TokenLottery as Program<TokenLottery>;

  it('should init config', async () => {
    const initConfigIx = await program.methods.initializeConfig(
      new anchor.BN(0),
      new anchor.BN(1839809564),
      new anchor.BN(10000),
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
});
