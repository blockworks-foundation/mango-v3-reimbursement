import { Cluster, Config } from "@blockworks-foundation/mango-client";
import { AnchorProvider, Provider, Wallet } from "@project-serum/anchor";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
import { bs58 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { Connection, Keypair, MemcmpFilter, PublicKey } from "@solana/web3.js";
import { MangoV3ReimbursementClient } from "./client";

const GROUP_NUM = Number(process.env.GROUP_NUM || 0);

const cluster = "mainnet" as Cluster;
const config = Config.ids();
const groupName = "mainnet.1";
const groupIds = config.getGroup(cluster, groupName);
if (!groupIds) {
  throw new Error(`Group ${groupName} not found`);
}

async function getGroups(client: MangoV3ReimbursementClient) {
  const bbuf = Buffer.alloc(4);
  bbuf.writeUInt32LE(GROUP_NUM);
  return await client.program.account.group.all([
    {
      memcmp: {
        bytes: bs58.encode(bbuf),
        offset: 8,
      },
    },
  ]);
}

async function main() {
  const options = AnchorProvider.defaultOptions();
  const connection = new Connection(
    "https://api.mainnet-beta.solana.com",
    options
  );
  const wallet = new Wallet(Keypair.generate());
  const provider = new AnchorProvider(connection, wallet, options);
  const client = new MangoV3ReimbursementClient(provider);

  // if ((await getGroups(client)).length === 0) {
  //   await client.program.methods
  //     .createGroup(GROUP_NUM)
  //     .accounts({
  //       table: new PublicKey("9Ut1gZJnd5D7EjPXm2DygYWZkZGpt5QSMEYAaVx2hur4"),
  //       payer: (this.program.provider as AnchorProvider).wallet.publicKey,
  //     })
  //     .rpc();
  // }
  // const group = (await getGroups(client))[0];

  // for (const tokenConfig of groupIds?.tokens!) {
  //   await client.program.methods
  //     .createVault(0, 6)
  //     .accounts({
  //       group: group.publicKey,
  //       mint: tokenConfig.mintKey,
  //       payer: (this.program.provider as AnchorProvider).wallet.publicKey,
  //     })
  //     .rpc();
  // }
}

main();
