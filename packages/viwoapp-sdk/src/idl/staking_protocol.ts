/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/staking_protocol.json`.
 */
export type StakingProtocol = {
  "address": "6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu",
  "metadata": {
    "name": "stakingProtocol",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "VCoin Staking Protocol with Tier System and veVCoin Minting"
  },
  "instructions": [
    {
      "name": "acceptAuthority",
      "discriminator": [
        107,
        86,
        198,
        91,
        33,
        12,
        107,
        160
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "newAuthority",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "cancelAuthorityTransfer",
      "discriminator": [
        94,
        131,
        125,
        184,
        183,
        24,
        125,
        229
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "cancelSlash",
      "discriminator": [
        222,
        163,
        161,
        54,
        103,
        113,
        42,
        67
      ],
      "accounts": [
        {
          "name": "close",
          "accounts": [
            {
              "name": "registrar",
              "pda": {
                "seeds": [
                  {
                    "kind": "const",
                    "value": [
                      114,
                      101,
                      103,
                      105,
                      115,
                      116,
                      114,
                      97,
                      114
                    ]
                  }
                ]
              }
            },
            {
              "name": "voter",
              "writable": true,
              "pda": {
                "seeds": [
                  {
                    "kind": "const",
                    "value": [
                      118,
                      111,
                      116,
                      101,
                      114
                    ]
                  },
                  {
                    "kind": "account",
                    "path": "voter.owner",
                    "account": "voter"
                  }
                ]
              },
              "relations": [
                "slashRequest"
              ]
            },
            {
              "name": "slashRequest",
              "writable": true,
              "pda": {
                "seeds": [
                  {
                    "kind": "const",
                    "value": [
                      115,
                      108,
                      97,
                      115,
                      104
                    ]
                  },
                  {
                    "kind": "account",
                    "path": "voter"
                  },
                  {
                    "kind": "account",
                    "path": "slash_request.nonce",
                    "account": "slashRequest"
                  }
                ]
              }
            },
            {
              "name": "rentPayer",
              "writable": true,
              "relations": [
                "slashRequest"
              ]
            }
          ]
        },
        {
          "name": "authority",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "closeVoter",
      "docs": [
        "Closes an empty voter account and refunds its rent."
      ],
      "discriminator": [
        117,
        35,
        234,
        247,
        206,
        131,
        182,
        149
      ],
      "accounts": [
        {
          "name": "voter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true,
          "relations": [
            "voter"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "configureGovernance",
      "docs": [
        "Wires the registrar to its Realms realm as voter-weight plugin (one-shot)."
      ],
      "discriminator": [
        46,
        219,
        164,
        132,
        222,
        7,
        9,
        91
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "realm"
        },
        {
          "name": "vcoinMint",
          "relations": [
            "registrar"
          ]
        },
        {
          "name": "slashDestination",
          "docs": [
            "DAO treasury token account that receives slashed VCoin."
          ]
        },
        {
          "name": "maxVoterWeightRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  97,
                  120,
                  45,
                  118,
                  111,
                  116,
                  101,
                  114,
                  45,
                  119,
                  101,
                  105,
                  103,
                  104,
                  116,
                  45,
                  114,
                  101,
                  99,
                  111,
                  114,
                  100
                ]
              }
            ]
          }
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "governanceProgramId",
          "type": "pubkey"
        },
        {
          "name": "minVoteScore",
          "type": "u16"
        },
        {
          "name": "minProposeScore",
          "type": "u16"
        }
      ]
    },
    {
      "name": "createRegistrar",
      "docs": [
        "Creates the registrar and VCoin vault. Upgrade authority only."
      ],
      "discriminator": [
        132,
        235,
        36,
        49,
        139,
        66,
        202,
        69
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "vcoinMint",
          "docs": [
            "Must be the vcoin-token program's mint PDA (checked in the handler) and pass",
            "`validate_vcoin_mint` (no hook, fee, freeze authority or permanent delegate)."
          ]
        },
        {
          "name": "vevcoinMint",
          "docs": [
            "Must be the vevcoin-token program's mint PDA (checked in the handler)."
          ]
        },
        {
          "name": "vault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "program",
          "address": "6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu"
        },
        {
          "name": "programData"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "createVoter",
      "docs": [
        "Creates the caller's voter account."
      ],
      "discriminator": [
        6,
        24,
        245,
        52,
        243,
        255,
        148,
        25
      ],
      "accounts": [
        {
          "name": "registrar",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "voter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "signer": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "createVoterWeightRecord",
      "discriminator": [
        184,
        249,
        133,
        178,
        88,
        152,
        250,
        186
      ],
      "accounts": [
        {
          "name": "registrar",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "voterWeightRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  116,
                  101,
                  114,
                  45,
                  119,
                  101,
                  105,
                  103,
                  104,
                  116,
                  45,
                  114,
                  101,
                  99,
                  111,
                  114,
                  100
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner"
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "deactivateExpired",
      "docs": [
        "Marks an expired deposit inactive so it stops counting toward the max voter weight."
      ],
      "discriminator": [
        161,
        129,
        114,
        115,
        201,
        239,
        157,
        233
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "voter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "voter.owner",
                "account": "voter"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "index",
          "type": "u8"
        }
      ]
    },
    {
      "name": "deposit",
      "docs": [
        "Locks VCoin into deposit slot `index` for at least `lock_secs` seconds."
      ],
      "discriminator": [
        242,
        35,
        198,
        137,
        82,
        225,
        242,
        182
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "voter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "signer": true,
          "relations": [
            "voter"
          ]
        },
        {
          "name": "vault",
          "writable": true,
          "relations": [
            "registrar"
          ]
        },
        {
          "name": "vcoinMint",
          "relations": [
            "registrar"
          ]
        },
        {
          "name": "ownerVcoin",
          "writable": true
        },
        {
          "name": "vevcoinMint",
          "writable": true,
          "relations": [
            "registrar"
          ]
        },
        {
          "name": "ownerVevcoin",
          "docs": [
            "The owner's canonical veVCoin ATA: receipts live in exactly one account per owner, so",
            "a slash can always burn them (anyone can recreate the ATA)."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "vevcoinMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "vevcoinConfig"
        },
        {
          "name": "vevcoinProgram",
          "address": "FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": [
        {
          "name": "index",
          "type": "u8"
        },
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "lockSecs",
          "type": "i64"
        }
      ]
    },
    {
      "name": "executeSlash",
      "discriminator": [
        15,
        240,
        38,
        73,
        175,
        55,
        88,
        55
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "voter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "voter.owner",
                "account": "voter"
              }
            ]
          },
          "relations": [
            "slashRequest"
          ]
        },
        {
          "name": "slashRequest",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  108,
                  97,
                  115,
                  104
                ]
              },
              {
                "kind": "account",
                "path": "voter"
              },
              {
                "kind": "account",
                "path": "slash_request.nonce",
                "account": "slashRequest"
              }
            ]
          }
        },
        {
          "name": "rentPayer",
          "writable": true,
          "relations": [
            "slashRequest"
          ]
        },
        {
          "name": "vault",
          "writable": true,
          "relations": [
            "registrar"
          ]
        },
        {
          "name": "vcoinMint",
          "relations": [
            "registrar"
          ]
        },
        {
          "name": "slashDestination",
          "writable": true,
          "relations": [
            "registrar"
          ]
        },
        {
          "name": "vevcoinMint",
          "writable": true,
          "relations": [
            "registrar"
          ]
        },
        {
          "name": "owner"
        },
        {
          "name": "ownerVevcoin",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "owner"
              },
              {
                "kind": "account",
                "path": "tokenProgram"
              },
              {
                "kind": "account",
                "path": "vevcoinMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "vevcoinConfig"
        },
        {
          "name": "vevcoinProgram",
          "address": "FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
    },
    {
      "name": "expireSlash",
      "discriminator": [
        114,
        239,
        117,
        220,
        151,
        115,
        241,
        214
      ],
      "accounts": [
        {
          "name": "registrar",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "voter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "voter.owner",
                "account": "voter"
              }
            ]
          },
          "relations": [
            "slashRequest"
          ]
        },
        {
          "name": "slashRequest",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  108,
                  97,
                  115,
                  104
                ]
              },
              {
                "kind": "account",
                "path": "voter"
              },
              {
                "kind": "account",
                "path": "slash_request.nonce",
                "account": "slashRequest"
              }
            ]
          }
        },
        {
          "name": "rentPayer",
          "writable": true,
          "relations": [
            "slashRequest"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "extendLock",
      "docs": [
        "Moves a deposit's lock end to `now + lock_secs` (must be later than today's end)."
      ],
      "discriminator": [
        68,
        151,
        140,
        144,
        139,
        122,
        118,
        170
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "voter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "signer": true,
          "relations": [
            "voter"
          ]
        }
      ],
      "args": [
        {
          "name": "index",
          "type": "u8"
        },
        {
          "name": "lockSecs",
          "type": "i64"
        }
      ]
    },
    {
      "name": "proposeAuthority",
      "discriminator": [
        20,
        148,
        236,
        198,
        76,
        119,
        99,
        142
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "newAuthority",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "proposeSlash",
      "docs": [
        "Opens a slash of `amount` against a voter (authority = the DAO). Freezes the",
        "voter's withdrawals; executable by anyone after 48h, void 7 days later."
      ],
      "discriminator": [
        198,
        144,
        128,
        49,
        72,
        78,
        74,
        118
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "voter",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "voter.owner",
                "account": "voter"
              }
            ]
          }
        },
        {
          "name": "slashRequest",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  108,
                  97,
                  115,
                  104
                ]
              },
              {
                "kind": "account",
                "path": "voter"
              },
              {
                "kind": "account",
                "path": "registrar.slash_nonce",
                "account": "registrar"
              }
            ]
          }
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "reasonHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        }
      ]
    },
    {
      "name": "setGateParams",
      "discriminator": [
        210,
        137,
        12,
        226,
        140,
        51,
        168,
        141
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "minVoteScore",
          "type": "u16"
        },
        {
          "name": "minProposeScore",
          "type": "u16"
        }
      ]
    },
    {
      "name": "setGuardian",
      "docs": [
        "Sets or clears the emergency guardian (may pause, never unpause)."
      ],
      "discriminator": [
        147,
        243,
        50,
        121,
        154,
        164,
        50,
        30
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "guardian",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "setPaused",
      "discriminator": [
        91,
        60,
        125,
        192,
        176,
        225,
        166,
        218
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "paused",
          "type": "bool"
        }
      ]
    },
    {
      "name": "setSlashDestination",
      "discriminator": [
        169,
        103,
        49,
        31,
        130,
        160,
        239,
        131
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "vcoinMint",
          "relations": [
            "registrar"
          ]
        },
        {
          "name": "slashDestination"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
    },
    {
      "name": "setWeightParams",
      "discriminator": [
        69,
        84,
        102,
        30,
        62,
        106,
        226,
        79
      ],
      "accounts": [
        {
          "name": "registrar",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "weightParams"
            }
          }
        }
      ]
    },
    {
      "name": "updateMaxVoterWeightRecord",
      "discriminator": [
        103,
        175,
        201,
        251,
        2,
        9,
        251,
        179
      ],
      "accounts": [
        {
          "name": "registrar",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "maxVoterWeightRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  97,
                  120,
                  45,
                  118,
                  111,
                  116,
                  101,
                  114,
                  45,
                  119,
                  101,
                  105,
                  103,
                  104,
                  116,
                  45,
                  114,
                  101,
                  99,
                  111,
                  114,
                  100
                ]
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "updateVoterWeightRecord",
      "docs": [
        "Writes the voter's weight for one Realms action (valid for the current slot)."
      ],
      "discriminator": [
        45,
        185,
        3,
        36,
        109,
        190,
        115,
        169
      ],
      "accounts": [
        {
          "name": "registrar",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  97,
                  114
                ]
              }
            ]
          }
        },
        {
          "name": "voter",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  116,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "voter.owner",
                "account": "voter"
              }
            ]
          }
        },
        {
          "name": "voterWeightRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  116,
                  101,
                  114,
                  45,
                  119,
                  101,
                  105,
                  103,
                  104,
                  116,
                  45,
                  114,
                  101,
                  99,
                  111,
                  114,
                  100
                ]
              },
              {
                "kind": "account",
                "path": "voter.owner",
                "account": "voter"
              }
            ]
          }
        },
        {
          "name": "userScore"
        }
      ],
      "args": [
        {
          "name": "action",
          "type": {
            "defined": {
              "name": "voterWeightAction"
            }
          }
        },
        {
          "name": "target",
          "type": {
            "option": "pubkey"
          }
        }
      ]
    },
    {
      "name": "withdraw",
      "docs": [
        "Withdraws unlocked VCoin from deposit slot `index`."
      ],
      "discriminator": [
        183,
        18,
        70,
        156,
        148,
        109,
        161,
        34
      ],
      "accounts": [
        {
          "name": "stake",
          "accounts": [
            {
              "name": "registrar",
              "writable": true,
              "pda": {
                "seeds": [
                  {
                    "kind": "const",
                    "value": [
                      114,
                      101,
                      103,
                      105,
                      115,
                      116,
                      114,
                      97,
                      114
                    ]
                  }
                ]
              }
            },
            {
              "name": "voter",
              "writable": true,
              "pda": {
                "seeds": [
                  {
                    "kind": "const",
                    "value": [
                      118,
                      111,
                      116,
                      101,
                      114
                    ]
                  },
                  {
                    "kind": "account",
                    "path": "owner"
                  }
                ]
              }
            },
            {
              "name": "owner",
              "signer": true,
              "relations": [
                "voter"
              ]
            },
            {
              "name": "vault",
              "writable": true,
              "relations": [
                "registrar"
              ]
            },
            {
              "name": "vcoinMint",
              "relations": [
                "registrar"
              ]
            },
            {
              "name": "ownerVcoin",
              "writable": true
            },
            {
              "name": "vevcoinMint",
              "writable": true,
              "relations": [
                "registrar"
              ]
            },
            {
              "name": "ownerVevcoin",
              "docs": [
                "The owner's canonical veVCoin ATA: receipts live in exactly one account per owner, so",
                "a slash can always burn them (anyone can recreate the ATA)."
              ],
              "writable": true,
              "pda": {
                "seeds": [
                  {
                    "kind": "account",
                    "path": "owner"
                  },
                  {
                    "kind": "account",
                    "path": "tokenProgram"
                  },
                  {
                    "kind": "account",
                    "path": "vevcoinMint"
                  }
                ],
                "program": {
                  "kind": "const",
                  "value": [
                    140,
                    151,
                    37,
                    143,
                    78,
                    36,
                    137,
                    241,
                    187,
                    61,
                    16,
                    41,
                    20,
                    142,
                    13,
                    131,
                    11,
                    90,
                    19,
                    153,
                    218,
                    255,
                    16,
                    132,
                    4,
                    142,
                    123,
                    216,
                    219,
                    233,
                    248,
                    89
                  ]
                }
              }
            },
            {
              "name": "vevcoinConfig"
            },
            {
              "name": "vevcoinProgram",
              "address": "FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx"
            },
            {
              "name": "tokenProgram",
              "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
            }
          ]
        },
        {
          "name": "tokenOwnerRecord"
        }
      ],
      "args": [
        {
          "name": "index",
          "type": "u8"
        },
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "maxVoterWeightRecord",
      "discriminator": [
        157,
        95,
        242,
        151,
        16,
        98,
        26,
        118
      ]
    },
    {
      "name": "registrar",
      "discriminator": [
        193,
        202,
        205,
        51,
        78,
        168,
        150,
        128
      ]
    },
    {
      "name": "slashRequest",
      "discriminator": [
        147,
        209,
        38,
        84,
        60,
        216,
        230,
        70
      ]
    },
    {
      "name": "voter",
      "discriminator": [
        241,
        93,
        35,
        191,
        254,
        147,
        17,
        202
      ]
    },
    {
      "name": "voterWeightRecord",
      "discriminator": [
        46,
        249,
        155,
        75,
        153,
        248,
        116,
        9
      ]
    }
  ],
  "events": [
    {
      "name": "authorityProposed",
      "discriminator": [
        244,
        117,
        94,
        112,
        53,
        151,
        35,
        89
      ]
    },
    {
      "name": "authorityTransferCancelled",
      "discriminator": [
        31,
        228,
        187,
        148,
        20,
        99,
        237,
        48
      ]
    },
    {
      "name": "authorityTransferred",
      "discriminator": [
        245,
        109,
        179,
        54,
        135,
        92,
        22,
        64
      ]
    },
    {
      "name": "depositDeactivated",
      "discriminator": [
        46,
        190,
        51,
        109,
        220,
        81,
        47,
        52
      ]
    },
    {
      "name": "deposited",
      "discriminator": [
        111,
        141,
        26,
        45,
        161,
        35,
        100,
        57
      ]
    },
    {
      "name": "gateParamsUpdated",
      "discriminator": [
        149,
        17,
        1,
        38,
        44,
        120,
        77,
        202
      ]
    },
    {
      "name": "governanceConfigured",
      "discriminator": [
        20,
        49,
        34,
        211,
        4,
        57,
        158,
        250
      ]
    },
    {
      "name": "guardianSet",
      "discriminator": [
        159,
        49,
        155,
        156,
        176,
        88,
        29,
        190
      ]
    },
    {
      "name": "lockExtended",
      "discriminator": [
        25,
        118,
        227,
        150,
        119,
        138,
        207,
        234
      ]
    },
    {
      "name": "pauseChanged",
      "discriminator": [
        238,
        188,
        213,
        78,
        134,
        209,
        178,
        218
      ]
    },
    {
      "name": "registrarCreated",
      "discriminator": [
        244,
        170,
        255,
        18,
        94,
        149,
        46,
        71
      ]
    },
    {
      "name": "slashCancelled",
      "discriminator": [
        30,
        217,
        21,
        135,
        62,
        236,
        194,
        221
      ]
    },
    {
      "name": "slashDestinationChanged",
      "discriminator": [
        83,
        74,
        117,
        243,
        104,
        194,
        226,
        42
      ]
    },
    {
      "name": "slashExecuted",
      "discriminator": [
        170,
        48,
        78,
        119,
        242,
        155,
        108,
        61
      ]
    },
    {
      "name": "slashExpired",
      "discriminator": [
        82,
        186,
        235,
        181,
        176,
        9,
        128,
        0
      ]
    },
    {
      "name": "slashProposed",
      "discriminator": [
        158,
        90,
        179,
        8,
        227,
        190,
        15,
        231
      ]
    },
    {
      "name": "voterCreated",
      "discriminator": [
        211,
        244,
        166,
        5,
        238,
        33,
        110,
        76
      ]
    },
    {
      "name": "weightParamsUpdated",
      "discriminator": [
        191,
        60,
        240,
        236,
        105,
        47,
        6,
        215
      ]
    },
    {
      "name": "withdrawn",
      "discriminator": [
        20,
        89,
        223,
        198,
        194,
        124,
        219,
        13
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "paused",
      "msg": "Staking is paused"
    },
    {
      "code": 6001,
      "name": "zeroAmount",
      "msg": "Amount must be greater than zero"
    },
    {
      "code": 6002,
      "name": "invalidLockDuration",
      "msg": "Lock duration outside [1 week, 4 years]"
    },
    {
      "code": 6003,
      "name": "invalidDepositIndex",
      "msg": "Deposit index out of range"
    },
    {
      "code": 6004,
      "name": "depositNotUsed",
      "msg": "Deposit slot is empty"
    },
    {
      "code": 6005,
      "name": "depositExpired",
      "msg": "Cannot add to an expired deposit; withdraw or relock it first"
    },
    {
      "code": 6006,
      "name": "lockNotExtended",
      "msg": "A lock can only be extended"
    },
    {
      "code": 6007,
      "name": "stillLocked",
      "msg": "Deposit is still locked"
    },
    {
      "code": 6008,
      "name": "insufficientDeposit",
      "msg": "Amount exceeds the deposit"
    },
    {
      "code": 6009,
      "name": "voterNotEmpty",
      "msg": "Voter still has deposits"
    },
    {
      "code": 6010,
      "name": "invalidProgramData",
      "msg": "Program data account does not belong to this program"
    },
    {
      "code": 6011,
      "name": "invalidVCoinMint",
      "msg": "VCoin mint must be the vcoin-token program's mint PDA"
    },
    {
      "code": 6012,
      "name": "invalidVeVCoinMint",
      "msg": "veVCoin mint must be the vevcoin-token program's mint PDA"
    },
    {
      "code": 6013,
      "name": "nothingReceived",
      "msg": "No VCoin was received by the vault"
    },
    {
      "code": 6014,
      "name": "invalidWeightParams",
      "msg": "Weight parameters out of bounds"
    },
    {
      "code": 6015,
      "name": "invalidRealm",
      "msg": "Realm must be a RealmV2 of the configured governance program with VCoin as community mint"
    },
    {
      "code": 6016,
      "name": "governanceAlreadyConfigured",
      "msg": "Governance is already configured"
    },
    {
      "code": 6017,
      "name": "governanceNotConfigured",
      "msg": "Governance is not configured yet"
    },
    {
      "code": 6018,
      "name": "invalidTokenOwnerRecord",
      "msg": "Token owner record does not match this realm, mint and owner"
    },
    {
      "code": 6019,
      "name": "votesNotRelinquished",
      "msg": "Relinquish all votes and finish open proposals before withdrawing"
    },
    {
      "code": 6020,
      "name": "scoreBelowGate",
      "msg": "5A reputation score is below the required gate"
    },
    {
      "code": 6021,
      "name": "invalidScoreAccount",
      "msg": "5A score account is not the owner's UserScore PDA"
    },
    {
      "code": 6022,
      "name": "invalidGateParams",
      "msg": "Gate scores must be at most 10000"
    },
    {
      "code": 6023,
      "name": "slashPending",
      "msg": "Withdrawals are frozen while a slash request is pending"
    },
    {
      "code": 6024,
      "name": "slashNotReady",
      "msg": "Slash request is still inside its appeal window"
    },
    {
      "code": 6025,
      "name": "slashWindowClosed",
      "msg": "Slash request has expired"
    },
    {
      "code": 6026,
      "name": "slashNotExpired",
      "msg": "Slash request has not expired yet"
    },
    {
      "code": 6027,
      "name": "alreadyDeactivated",
      "msg": "Deposit is already inactive"
    },
    {
      "code": 6028,
      "name": "overflow",
      "msg": "Arithmetic overflow"
    }
  ],
  "types": [
    {
      "name": "adminState",
      "docs": [
        "Authority with a two-step, time-locked transfer, plus an optional emergency guardian.",
        "",
        "`propose` (current authority) → wait [`AUTHORITY_TRANSFER_DELAY`] → `accept`",
        "(new authority). The current authority can `cancel` at any time before acceptance.",
        "",
        "After launch the authority is a Realms governance PDA, which needs a vote and a hold-up",
        "(days) to act. The `guardian` (e.g. a council multisig or a zero-hold-up council",
        "governance) exists so an emergency stop is possible within minutes: it may **pause**",
        "(never unpause) and use program-specific stop actions such as vetoing a rewards root.",
        "It cannot move funds, change parameters or keep the protocol paused against the",
        "authority, which can unpause and replace it."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "pendingAuthority",
            "docs": [
              "`Pubkey::default()` when no transfer is pending."
            ],
            "type": "pubkey"
          },
          {
            "name": "pendingSince",
            "docs": [
              "Unix time the pending transfer was proposed (0 when none)."
            ],
            "type": "i64"
          },
          {
            "name": "guardian",
            "docs": [
              "Emergency guardian (`Pubkey::default()` = none)."
            ],
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "authorityProposed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "current",
            "type": "pubkey"
          },
          {
            "name": "proposed",
            "type": "pubkey"
          },
          {
            "name": "readyAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "authorityTransferCancelled",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "cancelled",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "authorityTransferred",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "previous",
            "type": "pubkey"
          },
          {
            "name": "newAuthority",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "depositDeactivated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "index",
            "type": "u8"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "activeLocked",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "depositEntry",
      "docs": [
        "One locked deposit."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "lockStart",
            "type": "i64"
          },
          {
            "name": "lockEnd",
            "type": "i64"
          },
          {
            "name": "isUsed",
            "type": "bool"
          },
          {
            "name": "isActive",
            "docs": [
              "Counted in `Registrar::active_locked` (the base of the max voter weight). Cleared by",
              "`deactivate_expired` or a withdrawal once the lock has ended; set again by a relock."
            ],
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "deposited",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "index",
            "type": "u8"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "lockEnd",
            "type": "i64"
          },
          {
            "name": "totalLocked",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "gateParamsUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "minVoteScore",
            "type": "u16"
          },
          {
            "name": "minProposeScore",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "governanceConfigured",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "realm",
            "type": "pubkey"
          },
          {
            "name": "governanceProgramId",
            "type": "pubkey"
          },
          {
            "name": "slashDestination",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "guardianSet",
      "docs": [
        "The emergency guardian may pause (never unpause); `Pubkey::default()` = none."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "guardian",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "lockExtended",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "index",
            "type": "u8"
          },
          {
            "name": "oldLockEnd",
            "type": "i64"
          },
          {
            "name": "newLockEnd",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "maxVoterWeightRecord",
      "docs": [
        "Realm-wide maximum voter weight, PDA `[\"max-voter-weight-record\"]`."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "realm",
            "type": "pubkey"
          },
          {
            "name": "governingTokenMint",
            "type": "pubkey"
          },
          {
            "name": "maxVoterWeight",
            "type": "u64"
          },
          {
            "name": "maxVoterWeightExpiry",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          }
        ]
      }
    },
    {
      "name": "pauseChanged",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "paused",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "registrar",
      "docs": [
        "Singleton registrar, PDA `[\"registrar\"]`. Owns the VCoin vault and is the only",
        "account the veVCoin program accepts as mint/burn signer."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "docs": [
              "Authority for pause, parameters and slashing. Bootstrapped by the deployer and handed",
              "(24h two-step) to the Realms governance PDA after launch, so every change is a DAO vote."
            ],
            "type": {
              "defined": {
                "name": "adminState"
              }
            }
          },
          {
            "name": "governanceProgramId",
            "docs": [
              "Realms (SPL Governance) program and realm this registrar serves as voter-weight plugin."
            ],
            "type": "pubkey"
          },
          {
            "name": "realm",
            "type": "pubkey"
          },
          {
            "name": "realmGoverningTokenMint",
            "docs": [
              "The realm's community governing-token mint (the mint Realms votes are cast for)."
            ],
            "type": "pubkey"
          },
          {
            "name": "vcoinMint",
            "type": "pubkey"
          },
          {
            "name": "vault",
            "type": "pubkey"
          },
          {
            "name": "vevcoinMint",
            "type": "pubkey"
          },
          {
            "name": "slashDestination",
            "docs": [
              "Where slashed VCoin goes (DAO treasury token account)."
            ],
            "type": "pubkey"
          },
          {
            "name": "weight",
            "type": {
              "defined": {
                "name": "weightParams"
              }
            }
          },
          {
            "name": "minVoteScore",
            "docs": [
              "5A composite score (0..=10000) required to vote / to create proposals."
            ],
            "type": "u16"
          },
          {
            "name": "minProposeScore",
            "type": "u16"
          },
          {
            "name": "totalLocked",
            "docs": [
              "Sum of all deposits (receipt supply upper bound; drives MaxVoterWeightRecord)."
            ],
            "type": "u64"
          },
          {
            "name": "activeLocked",
            "docs": [
              "Principal in deposits not yet deactivated after their lock ended. The max voter weight",
              "is computed from this, so abandoned expired deposits cannot inflate it forever and",
              "make percentage thresholds unreachable."
            ],
            "type": "u64"
          },
          {
            "name": "slashNonce",
            "type": "u64"
          },
          {
            "name": "paused",
            "type": "bool"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "vaultBump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "registrarCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "registrar",
            "type": "pubkey"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "vcoinMint",
            "type": "pubkey"
          },
          {
            "name": "vevcoinMint",
            "type": "pubkey"
          },
          {
            "name": "vault",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "slashCancelled",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "request",
            "type": "pubkey"
          },
          {
            "name": "owner",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "slashDestinationChanged",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "previous",
            "type": "pubkey"
          },
          {
            "name": "newDestination",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "slashExecuted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "request",
            "type": "pubkey"
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "requested",
            "type": "u64"
          },
          {
            "name": "slashed",
            "type": "u64"
          },
          {
            "name": "receiptsBurned",
            "type": "u64"
          },
          {
            "name": "totalLocked",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "slashExpired",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "request",
            "type": "pubkey"
          },
          {
            "name": "owner",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "slashProposed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "request",
            "type": "pubkey"
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "reasonHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "executableAt",
            "type": "i64"
          },
          {
            "name": "expiresAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "slashRequest",
      "docs": [
        "A DAO-approved slash of one voter's locked deposits, PDA",
        "`[\"slash\", voter, nonce]`. Executable by anyone in `[executable_at, expires_at)`;",
        "the voter's withdrawals are frozen while it exists. Closed (rent to `rent_payer`) when",
        "cancelled, executed or expired."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "voter",
            "type": "pubkey"
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "docs": [
              "Requested amount; execution takes `min(amount, deposits)`."
            ],
            "type": "u64"
          },
          {
            "name": "reasonHash",
            "docs": [
              "Hash of the off-chain evidence / DAO proposal text."
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "proposedAt",
            "type": "i64"
          },
          {
            "name": "executableAt",
            "type": "i64"
          },
          {
            "name": "expiresAt",
            "type": "i64"
          },
          {
            "name": "nonce",
            "type": "u64"
          },
          {
            "name": "rentPayer",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "voter",
      "docs": [
        "Per-owner deposits, PDA `[\"voter\", owner]`."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "deposits",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "depositEntry"
                  }
                },
                8
              ]
            }
          },
          {
            "name": "pendingSlashes",
            "docs": [
              "Open slash requests against this voter; withdrawals are frozen while > 0."
            ],
            "type": "u8"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "voterCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "voter",
            "type": "pubkey"
          },
          {
            "name": "owner",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "voterWeightAction",
      "docs": [
        "Realms action a voter-weight record is valid for (order is part of the wire format)."
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "castVote"
          },
          {
            "name": "commentProposal"
          },
          {
            "name": "createGovernance"
          },
          {
            "name": "createProposal"
          },
          {
            "name": "signOffProposal"
          }
        ]
      }
    },
    {
      "name": "voterWeightRecord",
      "docs": [
        "Per-owner voter weight, PDA `[\"voter-weight-record\", owner]`. Rewritten by",
        "`update_voter_weight_record` in the same transaction as the Realms action",
        "(`voter_weight_expiry = current slot`)."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "realm",
            "type": "pubkey"
          },
          {
            "name": "governingTokenMint",
            "type": "pubkey"
          },
          {
            "name": "governingTokenOwner",
            "type": "pubkey"
          },
          {
            "name": "voterWeight",
            "type": "u64"
          },
          {
            "name": "voterWeightExpiry",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "weightAction",
            "type": {
              "option": {
                "defined": {
                  "name": "voterWeightAction"
                }
              }
            }
          },
          {
            "name": "weightActionTarget",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "reserved",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          }
        ]
      }
    },
    {
      "name": "weightParams",
      "docs": [
        "Voting-weight parameters (fixed point, [`FACTOR_SCALE`] = 1.0×)."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "baselineFactor",
            "type": "u64"
          },
          {
            "name": "maxExtraFactor",
            "type": "u64"
          },
          {
            "name": "saturationSecs",
            "type": "u64"
          },
          {
            "name": "minLockRemainingToVote",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "weightParamsUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "baselineFactor",
            "type": "u64"
          },
          {
            "name": "maxExtraFactor",
            "type": "u64"
          },
          {
            "name": "saturationSecs",
            "type": "u64"
          },
          {
            "name": "minLockRemainingToVote",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "withdrawn",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "index",
            "type": "u8"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "totalLocked",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "constants": [
    {
      "name": "maxLockSecs",
      "type": "i64",
      "value": "126144000"
    },
    {
      "name": "maxVoterWeightRecordSeed",
      "type": "bytes",
      "value": "[109, 97, 120, 45, 118, 111, 116, 101, 114, 45, 119, 101, 105, 103, 104, 116, 45, 114, 101, 99, 111, 114, 100]"
    },
    {
      "name": "minLockSecs",
      "docs": [
        "Lock bounds (seconds)."
      ],
      "type": "i64",
      "value": "604800"
    },
    {
      "name": "registrarSeed",
      "docs": [
        "Registrar PDA seed (must match `vevcoin_token::constants::STAKING_REGISTRAR_SEED`)."
      ],
      "type": "bytes",
      "value": "[114, 101, 103, 105, 115, 116, 114, 97, 114]"
    },
    {
      "name": "slashSeed",
      "docs": [
        "Slashing: DAO-proposed, executable after a 48h appeal window, void after 7 more days."
      ],
      "type": "bytes",
      "value": "[115, 108, 97, 115, 104]"
    },
    {
      "name": "vaultSeed",
      "docs": [
        "VCoin vault PDA seed (token account owned by the registrar)."
      ],
      "type": "bytes",
      "value": "[118, 97, 117, 108, 116]"
    },
    {
      "name": "voterSeed",
      "docs": [
        "Per-owner voter PDA seed: `[\"voter\", owner]`."
      ],
      "type": "bytes",
      "value": "[118, 111, 116, 101, 114]"
    },
    {
      "name": "voterWeightRecordSeed",
      "docs": [
        "Realms voter-weight records written by this plugin."
      ],
      "type": "bytes",
      "value": "[118, 111, 116, 101, 114, 45, 119, 101, 105, 103, 104, 116, 45, 114, 101, 99, 111, 114, 100]"
    }
  ]
};
