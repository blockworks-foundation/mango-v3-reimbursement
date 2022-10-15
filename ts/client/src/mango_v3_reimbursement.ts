export type MangoV3Reimbursement = {
  "version": "0.1.0",
  "name": "mango_v3_reimbursement",
  "instructions": [
    {
      "name": "createGroup",
      "accounts": [
        {
          "name": "group",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "groupNum",
          "type": "u32"
        },
        {
          "name": "table",
          "type": "publicKey"
        },
        {
          "name": "claimTransferDestinationAtasOwner",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "createVault",
      "accounts": [
        {
          "name": "group",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "claimMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "tokenIndex",
          "type": {
            "defined": "usize"
          }
        },
        {
          "name": "mintDecimals",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createReimbursementAccount",
      "accounts": [
        {
          "name": "group",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reimbursementAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccountOwner",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "startReimbursement",
      "accounts": [
        {
          "name": "group",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "reimburse",
      "accounts": [
        {
          "name": "group",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reimbursementAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccountOwner",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "claimMintTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "claimMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "table",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "indexIntoTable",
          "type": {
            "defined": "usize"
          }
        },
        {
          "name": "tokenIndex",
          "type": {
            "defined": "usize"
          }
        },
        {
          "name": "transferClaim",
          "type": "bool"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "group",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "groupNum",
            "type": "u32"
          },
          {
            "name": "table",
            "type": "publicKey"
          },
          {
            "name": "claimTransferDestinationAtasOwner",
            "type": "publicKey"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "vaults",
            "type": {
              "array": [
                "publicKey",
                16
              ]
            }
          },
          {
            "name": "claimMints",
            "type": {
              "array": [
                "publicKey",
                16
              ]
            }
          },
          {
            "name": "mints",
            "type": {
              "array": [
                "publicKey",
                16
              ]
            }
          },
          {
            "name": "reimbursementStarted",
            "type": "u8"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                3
              ]
            }
          }
        ]
      }
    },
    {
      "name": "table",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rows",
            "type": {
              "array": [
                {
                  "defined": "Row"
                },
                32000
              ]
            }
          }
        ]
      }
    },
    {
      "name": "row",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "balances",
            "type": {
              "array": [
                "u64",
                16
              ]
            }
          }
        ]
      }
    },
    {
      "name": "reimbursementAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "done",
            "type": "u16"
          },
          {
            "name": "claimTransferred",
            "type": "u16"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          }
        ]
      }
    },
    {
      "name": "mangoAccountReimbursementState",
      "type": {
        "kind": "struct",
        "fields": []
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "SomeError"
    }
  ]
};

export const IDL: MangoV3Reimbursement = {
  "version": "0.1.0",
  "name": "mango_v3_reimbursement",
  "instructions": [
    {
      "name": "createGroup",
      "accounts": [
        {
          "name": "group",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "groupNum",
          "type": "u32"
        },
        {
          "name": "table",
          "type": "publicKey"
        },
        {
          "name": "claimTransferDestinationAtasOwner",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "createVault",
      "accounts": [
        {
          "name": "group",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "claimMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "tokenIndex",
          "type": {
            "defined": "usize"
          }
        },
        {
          "name": "mintDecimals",
          "type": "u8"
        }
      ]
    },
    {
      "name": "createReimbursementAccount",
      "accounts": [
        {
          "name": "group",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reimbursementAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccountOwner",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "CHECK"
          ]
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "startReimbursement",
      "accounts": [
        {
          "name": "group",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "reimburse",
      "accounts": [
        {
          "name": "group",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "reimbursementAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mangoAccountOwner",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "claimMintTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "claimMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "table",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "indexIntoTable",
          "type": {
            "defined": "usize"
          }
        },
        {
          "name": "tokenIndex",
          "type": {
            "defined": "usize"
          }
        },
        {
          "name": "transferClaim",
          "type": "bool"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "group",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "groupNum",
            "type": "u32"
          },
          {
            "name": "table",
            "type": "publicKey"
          },
          {
            "name": "claimTransferDestinationAtasOwner",
            "type": "publicKey"
          },
          {
            "name": "authority",
            "type": "publicKey"
          },
          {
            "name": "vaults",
            "type": {
              "array": [
                "publicKey",
                16
              ]
            }
          },
          {
            "name": "claimMints",
            "type": {
              "array": [
                "publicKey",
                16
              ]
            }
          },
          {
            "name": "mints",
            "type": {
              "array": [
                "publicKey",
                16
              ]
            }
          },
          {
            "name": "reimbursementStarted",
            "type": "u8"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                3
              ]
            }
          }
        ]
      }
    },
    {
      "name": "table",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rows",
            "type": {
              "array": [
                {
                  "defined": "Row"
                },
                32000
              ]
            }
          }
        ]
      }
    },
    {
      "name": "row",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "balances",
            "type": {
              "array": [
                "u64",
                16
              ]
            }
          }
        ]
      }
    },
    {
      "name": "reimbursementAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "done",
            "type": "u16"
          },
          {
            "name": "claimTransferred",
            "type": "u16"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          }
        ]
      }
    },
    {
      "name": "mangoAccountReimbursementState",
      "type": {
        "kind": "struct",
        "fields": []
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "SomeError"
    }
  ]
};
