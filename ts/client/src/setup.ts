import {
  Cluster,
  Config,
  MangoClient,
} from "@blockworks-foundation/mango-client";
import { AnchorProvider, Provider, Wallet } from "@project-serum/anchor";
import { bs58 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { Connection, Keypair, MemcmpFilter, PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddress, getMint } from "@solana/spl-token";
import { MangoV3ReimbursementClient } from "./client";
import BN from "bn.js";
import fs from "fs";
import { set } from "@project-serum/anchor/dist/cjs/utils/features";
import { ClientRequest } from "http";

const CLUSTER_URL =
  process.env.CLUSTER_URL_OVERRIDE || process.env.MB_CLUSTER_URL;
const PAYER_KEYPAIR =
  process.env.PAYER_KEYPAIR_OVERRIDE || process.env.MB_PAYER_KEYPAIR;
const GROUP_NUM = Number(process.env.GROUP_NUM || 5);
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

Error.stackTraceLimit = 1000;

async function main() {
  let sig;

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
        sig + (CLUSTER === "devnet" ? "?cluster=devnet" : "")
      }`
    );
  }
  let group = (
    await mangoV3ReimbursementClient.program.account.group.all()
  ).find((group) => group.account.groupNum === GROUP_NUM);

  await mangoV3ReimbursementClient.program.methods
    .editGroup(new PublicKey("tabWqAkVwFcPGJTmEaik9KSbcDqRJRH4d39oyBrRzCn"))
    .accounts({
      group: group?.publicKey,
      authority: (mangoV3ReimbursementClient.program.provider as AnchorProvider)
        .wallet.publicKey,
    })
    .rpc();
  group = (await mangoV3ReimbursementClient.program.account.group.all()).find(
    (group) => group.account.groupNum === GROUP_NUM
  );

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
        sig + (CLUSTER === "devnet" ? "?cluster=devnet" : "")
      }`
    );
  }

  if (group?.account.reimbursementStarted === 0) {
    sig = await mangoV3ReimbursementClient.program.methods
      .startReimbursement()
      .accounts({
        group: (group as any).publicKey,
        authority: (
          mangoV3ReimbursementClient.program.provider as AnchorProvider
        ).wallet.publicKey,
      })
      .rpc();
    console.log(
      `start reimbursement, sig https://explorer.solana.com/tx/${
        sig + (CLUSTER === "devnet" ? "?cluster=devnet" : "")
      }`
    );
  }

  const reimbursementAccount = (
    await PublicKey.findProgramAddress(
      [
        Buffer.from("ReimbursementAccount"),
        group?.publicKey.toBuffer()!,
        admin.publicKey.toBuffer(),
      ],
      mangoV3ReimbursementClient.program.programId
    )
  )[0];
  if (!(await connection.getAccountInfo(reimbursementAccount))) {
    sig = await mangoV3ReimbursementClient.program.methods
      .createReimbursementAccount()
      .accounts({
        group: (group as any).publicKey,
        mangoAccountOwner: admin.publicKey,
        payer: (mangoV3ReimbursementClient.program.provider as AnchorProvider)
          .wallet.publicKey,
      })
      .rpc({ skipPreflight: true });
    console.log(
      `created reimbursement account for ${
        admin.publicKey
      }, sig https://explorer.solana.com/tx/${
        sig + (CLUSTER === "devnet" ? "?cluster=devnet" : "")
      }`
    );
  }

  const table = await mangoV3ReimbursementClient.decodeTable(group);
  const balancesForUser = table.rows.find((row) =>
    row.owner.equals(admin.publicKey)
  ).balances;

  sig = await mangoV3ReimbursementClient.program.methods
    .reimburse(new BN(0), new BN(0), false)
    .accounts({
      group: (group as any).publicKey,
      vault: group?.account.vaults[0],
      tokenAccount: await getAssociatedTokenAddress(
        group?.account.mints[0]!,
        admin.publicKey
      ),
      mint: group?.account.mints[0],
      reimbursementAccount,
      mangoAccountOwner: admin.publicKey,
      table: group?.account.table,
    })
    .rpc({ skipPreflight: true });
  console.log(
    `reimbursing ${admin.publicKey}, sig https://explorer.solana.com/tx/${
      sig + (CLUSTER === "devnet" ? "?cluster=devnet" : "")
    }`
  );

  // TODO
  // decoding the table on client side
}

main();
