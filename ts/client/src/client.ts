import { Program, ProgramAccount, Provider } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { MangoV3Reimbursement, IDL } from "./mango_v3_reimbursement";

export const ID = new PublicKey("m3roABq4Ta3sGyFRLdY4LH1KN16zBtg586gJ3UxoBzb");

export class MangoV3ReimbursementClient {
  public program: Program<MangoV3Reimbursement>;
  constructor(provider: Provider) {
    this.program = new Program<MangoV3Reimbursement>(
      IDL as MangoV3Reimbursement,
      ID,
      provider
    );
  }

  public async decodeTable(group) {
    const ai = await this.program.provider.connection.getAccountInfo(
      group.account.table
    );

    if (!ai) {
      throw new Error(`Table ai cannot be undefined!`);
    }

    return (this.program as any)._coder.accounts.accountLayouts
      .get("table")
      .decode(ai.data.subarray(40));
  }
}
