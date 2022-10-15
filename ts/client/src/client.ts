import { Program, Provider } from "@project-serum/anchor";
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
}
