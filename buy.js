import {
  Connection,
  PublicKey,
  SystemProgram,
  Transaction
} from "@solana/web3.js";

import {
  createUmi,
  publicKey,
  signerIdentity
} from "@metaplex-foundation/umi";

import {
  mplCore,
  transferV1
} from "@metaplex-foundation/mpl-core";

const RPC = "https://api.devnet.solana.com";

const COLLECTION =
  publicKey("6XGhHkPAHJ5XayXwn1p7t3XEUmyYEx3racfwnpbEwp6a");

const SELLER =
  new PublicKey("D3CaBnL7q2Fo4mmkxwTvwXrJcq4bXzHBctfQUEdnvfd5");

const connection = new Connection(RPC, "confirmed");

const GUARDIANS = {
  "D4CBoNmuCm2GDSwJi2YHyxTMte66nJGNYE55WJWa4kzU": 5,
  "47KQo8ddMP2q6z3zzs1E85f24cRgEFCe52iudvY2eG2K": 5,
  "3ZUJcmFQMcGbk4MdAeGoY3DLkY1us2hxbbFNzeD1H9mZ": 5,
  "BmUjwD3D3bPDYGu7hoR38VNNnhbcSxBwJgnRxkFpCxGk": 5,
  "9xHiNWXKfuhE2rVvCZXqNmD2gSLKj4KwRbRTTbLf2StS": 5,
  "sFGhKnzJfbbnFTbiVMp93H4iamP1cJMhbfMydppjp6F": 5
};

export async function buyGuardian(assetAddress) {

  if (!window.solana?.isPhantom) {
    throw new Error("Phantom wallet not detected.");
  }

  const buyerResponse = await window.solana.connect();
  const buyer = buyerResponse.publicKey;

  const priceSol = GUARDIANS[assetAddress];

  if (!priceSol) {
    throw new Error("Unknown Genesis Guardian.");
  }

  if (buyer.toString() === SELLER.toString()) {
    throw new Error("Seller cannot buy their own asset.");
  }

  /*
   * IMPORTANT:
   * Payment and Core Asset transfer must be executed
   * atomically. Do not send the SOL separately.
   */

  throw new Error(
    "Marketplace escrow is not activated yet. No SOL has been sent."
  );
}
