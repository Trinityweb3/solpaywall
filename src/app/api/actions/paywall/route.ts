import {
  ActionGetResponse,
  ActionPostRequest,
  ActionPostResponse,
  ACTIONS_CORS_HEADERS,
  createPostResponse,
} from "@solana/actions";
import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  clusterApiUrl,
} from "@solana/web3.js";

const DEFAULT_RECIPIENT = new PublicKey(
  "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"
);
const UNLOCK_PRICE_SOL = 0.01;

export async function GET(request: Request) {
  const payload: ActionGetResponse = {
    icon: "https://solpaywall.app/preview-lock.png",
    title: "Unlock Deep Research: SolPaywall Alpha",
    description:
      "Instant micro-monetization on Solana. Pay 0.01 SOL to read the confidential cohort notes and decrypt full resources.",
    label: "Unlock for 0.01 SOL",
    links: {
      actions: [
        {
          label: "Unlock for 0.01 SOL",
          href: "/api/actions/paywall?amount=0.01",
        },
      ],
    },
  };

  return Response.json(payload, { headers: ACTIONS_CORS_HEADERS });
}

export const OPTIONS = GET;

export async function POST(request: Request) {
  try {
    const body: ActionPostRequest = await request.json();
    let account: PublicKey;

    try {
      account = new PublicKey(body.account);
    } catch (err) {
      return new Response('Invalid "account" provided', {
        status: 400,
        headers: ACTIONS_CORS_HEADERS,
      });
    }

    const connection = new Connection(clusterApiUrl("devnet"));

    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: account,
        toPubkey: DEFAULT_RECIPIENT,
        lamports: UNLOCK_PRICE_SOL * LAMPORTS_PER_SOL,
      })
    );

    transaction.feePayer = account;
    transaction.recentBlockhash = (
      await connection.getLatestBlockhash()
    ).blockhash;

    const payload: ActionPostResponse = await createPostResponse({
      fields: {
        transaction,
        message: "Article unlocked! Redirecting to exclusive resources...",
      },
    });

    return Response.json(payload, { headers: ACTIONS_CORS_HEADERS });
  } catch (err) {
    return Response.json(
      { error: "Failed to create tx" },
      { status: 500, headers: ACTIONS_CORS_HEADERS }
    );
  }
}
