import {
  Cluster,
  Config,
  MangoClient,
} from "@blockworks-foundation/mango-client";
import { AnchorProvider, Provider, Wallet } from "@project-serum/anchor";
import { bs58 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { Connection, Keypair, MemcmpFilter, PublicKey } from "@solana/web3.js";
import { getMint } from "@solana/spl-token";
import { MangoV3ReimbursementClient } from "./client";
import BN from "bn.js";
import fs from "fs";
import { set } from "@project-serum/anchor/dist/cjs/utils/features";

const CLUSTER_URL =
  process.env.CLUSTER_URL_OVERRIDE || process.env.MB_CLUSTER_URL;
const PAYER_KEYPAIR =
  process.env.PAYER_KEYPAIR_OVERRIDE || process.env.MB_PAYER_KEYPAIR;
const GROUP_NUM = Number(process.env.GROUP_NUM || 2);
const CLUSTER: Cluster =
  (process.env.CLUSTER_OVERRIDE as Cluster) || "mainnet-beta";
const MANGO_V3_CLUSTER: Cluster =
  (process.env.MANGO_V3_CLUSTER_OVERRIDE as Cluster) || "mainnet";
const MANGO_V3_GROUP_NAME: Cluster =
  (process.env.MANGO_V3_GROUP_NAME_OVERRIDE as Cluster) || "mainnet.1";

const options = AnchorProvider.defaultOptions();
const connection = new Connection(CLUSTER_URL!, options);

const config = Config.ids();
const groupIds = config.getGroup(MANGO_V3_CLUSTER, MANGO_V3_GROUP_NAME);
if (!groupIds) {
  throw new Error(`Group ${MANGO_V3_GROUP_NAME} not found`);
}
const mangoProgramId = groupIds.mangoProgramId;
const mangoGroupKey = groupIds.publicKey;
const mangoV3Client = new MangoClient(connection, mangoProgramId);

// set("debug-logs");
Error.stackTraceLimit = 1000;

async function main() {
  const admin = Keypair.fromSecretKey(
    Buffer.from(JSON.parse(fs.readFileSync(PAYER_KEYPAIR!, "utf-8")))
  );
  const adminWallet = new Wallet(admin);
  const provider = new AnchorProvider(connection, adminWallet, options);
  const mangoV3ReimbursementClient = new MangoV3ReimbursementClient(provider);

  if (
    !(await mangoV3ReimbursementClient.program.account.group.all()).find(
      (group) => group.account.groupNum === GROUP_NUM
    )
  ) {
    const sig = await mangoV3ReimbursementClient.program.methods
      .createGroup(
        GROUP_NUM,
        new PublicKey("tab26VnfeLLkhVUa87mt5EWHA5PAbrVL1NCuSvZUSvc"),
        new PublicKey("mdcXrm2NkzXYvHNcKXzCLXT58R4UN8Rzd1uzD4h8338")
      )
      .accounts({
        payer: (mangoV3ReimbursementClient.program.provider as AnchorProvider)
          .wallet.publicKey,
        authority: (
          mangoV3ReimbursementClient.program.provider as AnchorProvider
        ).wallet.publicKey,
      })
      .rpc();
    console.log(
      `created group, sig https://explorer.solana.com/tx/${
        sig + (CLUSTER === "devnet" ? "cluster=devnet" : "")
      }`
    );
  }
  let group = (
    await mangoV3ReimbursementClient.program.account.group.all()
  ).find((group) => group.account.groupNum === GROUP_NUM);

  for (const [index, tokenInfo] of (
    await mangoV3Client.getMangoGroup(mangoGroupKey)
  ).tokens.entries()!) {
    if (tokenInfo.mint.equals(PublicKey.default)) {
      continue;
    }
    if (tokenInfo.oracleInactive === true) {
      continue;
    }
    if (!group?.account.vaults[index].equals(PublicKey.default)) {
      continue;
    }
    const mint = await getMint(connection, tokenInfo.mint);
    const sig = await mangoV3ReimbursementClient.program.methods
      .createVault(new BN(index), mint.decimals)
      .accounts({
        group: (group as any).publicKey,
        mint: tokenInfo.mint,
        payer: (mangoV3ReimbursementClient.program.provider as AnchorProvider)
          .wallet.publicKey,
      })
      .rpc();
    console.log(
      `setup vault, sig https://explorer.solana.com/tx/${
        sig + (CLUSTER === "devnet" ? "cluster=devnet" : "")
      }`
    );
  }

  // await mangoV3ReimbursementClient.program.methods
  //   .startReimbursement()
  //   .accounts({
  //     group: (group as any).publicKey,
  //     authority: (mangoV3ReimbursementClient.program.provider as AnchorProvider)
  //       .wallet.publicKey,
  //   })
  //   .rpc();
}

main();
