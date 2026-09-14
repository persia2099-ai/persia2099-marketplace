import { Connection, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";

const RPC = "https://api.devnet.solana.com";

const SELLER = new PublicKey(
  "D3CaBnL7q2Fo4mmkxwTvwXrJcq4bXzHBctfQUEdnvfd5"
);

const connection = new Connection(RPC, "confirmed");

export async function buyGuardian(priceSol) {
  if (!window.solana?.isPhantom) {
    throw new Error("Phantom wallet not detected.");
  }

  const wallet = await window.solana.connect();
  const buyer = wallet.publicKey;

  const lamports = Math.round(priceSol * 1_000_000_000);

  const transaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: buyer,
      toPubkey: SELLER,
      lamports
    })
  );

  transaction.feePayer = buyer;

  const { blockhash, lastValidBlockHeight } =
    await connection.getLatestBlockhash("confirmed");

  transaction.recentBlockhash = blockhash;

  const signed = await window.solana.signTransaction(transaction);

  const signature = await connection.sendRawTransaction(
    signed.serialize(),
    { skipPreflight: false }
  );

  await connection.confirmTransaction(
    {
      signature,
      blockhash,
      lastValidBlockHeight
    },
    "confirmed"
  );

  return signature;
}
