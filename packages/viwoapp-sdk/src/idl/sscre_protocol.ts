/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/sscre_protocol.json`.
 */
export type SscreProtocol = {
  "address": "6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ",
  "metadata": {
    "name": "sscreProtocol",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "SSCRE rewards: per-epoch merkle distributor funded from a reserve on a declining emission schedule, roots approved by an oracle quorum with a dispute delay and DAO veto, unclaimed funds clawed back"
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
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
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
      "name": "activateRoot",
      "discriminator": [
        67,
        150,
        173,
        61,
        217,
        174,
        188,
        125
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "epoch",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  112,
                  111,
                  99,
                  104
                ]
              },
              {
                "kind": "account",
                "path": "epoch.index",
                "account": "epoch"
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "applyFee",
      "discriminator": [
        59,
        156,
        222,
        176,
        105,
        28,
        247,
        64
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
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
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
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
      "name": "cancelFee",
      "discriminator": [
        5,
        77,
        56,
        64,
        104,
        252,
        30,
        22
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
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
      "name": "claim",
      "discriminator": [
        62,
        198,
        214,
        193,
        213,
        159,
        108,
        210
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "epoch",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  112,
                  111,
                  99,
                  104
                ]
              },
              {
                "kind": "account",
                "path": "epoch.index",
                "account": "epoch"
              }
            ]
          }
        },
        {
          "name": "vault",
          "writable": true,
          "relations": [
            "epoch"
          ]
        },
        {
          "name": "vcoinMint",
          "relations": [
            "config"
          ]
        },
        {
          "name": "claimant",
          "signer": true
        },
        {
          "name": "claimantToken",
          "writable": true
        },
        {
          "name": "feeVault",
          "writable": true,
          "relations": [
            "config"
          ]
        },
        {
          "name": "claimStatus",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "epoch"
              },
              {
                "kind": "account",
                "path": "claimant"
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
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "proof",
          "type": {
            "vec": {
              "array": [
                "u8",
                32
              ]
            }
          }
        }
      ]
    },
    {
      "name": "clawback",
      "discriminator": [
        111,
        92,
        142,
        79,
        33,
        234,
        82,
        27
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "epoch",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  112,
                  111,
                  99,
                  104
                ]
              },
              {
                "kind": "account",
                "path": "epoch.index",
                "account": "epoch"
              }
            ]
          }
        },
        {
          "name": "vault",
          "writable": true,
          "relations": [
            "epoch"
          ]
        },
        {
          "name": "reserveVault",
          "writable": true,
          "relations": [
            "config"
          ]
        },
        {
          "name": "vcoinMint",
          "relations": [
            "config"
          ]
        },
        {
          "name": "rentPayer",
          "writable": true,
          "relations": [
            "epoch"
          ]
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
    },
    {
      "name": "closeClaimStatus",
      "discriminator": [
        163,
        214,
        191,
        165,
        245,
        188,
        17,
        185
      ],
      "accounts": [
        {
          "name": "epoch",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  112,
                  111,
                  99,
                  104
                ]
              },
              {
                "kind": "account",
                "path": "epoch.index",
                "account": "epoch"
              }
            ]
          },
          "relations": [
            "claimStatus"
          ]
        },
        {
          "name": "claimStatus",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  108,
                  97,
                  105,
                  109
                ]
              },
              {
                "kind": "account",
                "path": "epoch"
              },
              {
                "kind": "account",
                "path": "claim_status.claimant",
                "account": "claimStatus"
              }
            ]
          }
        },
        {
          "name": "rentPayer",
          "writable": true,
          "relations": [
            "claimStatus"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "initializePool",
      "docs": [
        "Creates the pool and reserve vault. Upgrade authority only; VCoin mint pinned."
      ],
      "discriminator": [
        95,
        180,
        10,
        172,
        84,
        174,
        232,
        40
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "vcoinMint"
        },
        {
          "name": "reserveVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  115,
                  101,
                  114,
                  118,
                  101,
                  45,
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
          "name": "feeVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  101,
                  101,
                  45,
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
          "name": "feeDestination"
        },
        {
          "name": "program",
          "address": "6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ"
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
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "poolParams"
            }
          }
        },
        {
          "name": "firstEpochBudget",
          "type": "u64"
        },
        {
          "name": "feeBps",
          "type": "u16"
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
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
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
      "name": "proposeFee",
      "discriminator": [
        103,
        204,
        172,
        134,
        248,
        252,
        27,
        170
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
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
            "config"
          ]
        },
        {
          "name": "feeDestination"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": [
        {
          "name": "feeBps",
          "type": "u16"
        }
      ]
    },
    {
      "name": "registerOracle",
      "discriminator": [
        176,
        200,
        234,
        37,
        199,
        129,
        164,
        111
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
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
          "name": "oracle",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "removeOracle",
      "discriminator": [
        60,
        93,
        51,
        197,
        182,
        42,
        170,
        26
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
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
          "name": "oracle",
          "type": "pubkey"
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
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
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
      "name": "setNextEpochBudget",
      "discriminator": [
        31,
        146,
        92,
        20,
        29,
        197,
        51,
        85
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
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
          "name": "budget",
          "type": "u64"
        }
      ]
    },
    {
      "name": "setParams",
      "discriminator": [
        27,
        234,
        178,
        52,
        147,
        2,
        187,
        141
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
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
              "name": "poolParams"
            }
          }
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
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
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
      "name": "setRootQuorum",
      "discriminator": [
        29,
        109,
        121,
        116,
        0,
        238,
        20,
        175
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
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
          "name": "quorum",
          "type": "u8"
        }
      ]
    },
    {
      "name": "startEpoch",
      "docs": [
        "Funds the next epoch from the reserve (permissionless, rate-limited)."
      ],
      "discriminator": [
        204,
        248,
        232,
        82,
        251,
        45,
        164,
        113
      ],
      "accounts": [
        {
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "reserveVault",
          "writable": true,
          "relations": [
            "config"
          ]
        },
        {
          "name": "vcoinMint",
          "relations": [
            "config"
          ]
        },
        {
          "name": "epoch",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  112,
                  111,
                  99,
                  104
                ]
              },
              {
                "kind": "account",
                "path": "config.epoch_count",
                "account": "poolConfig"
              }
            ]
          }
        },
        {
          "name": "epochVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  112,
                  111,
                  99,
                  104,
                  45,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "config.epoch_count",
                "account": "poolConfig"
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
          "name": "authority",
          "docs": [
            "Only needed to start an epoch the reserve cannot fund in full: the pool authority."
          ],
          "signer": true,
          "optional": true
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
      "name": "vetoRoot",
      "discriminator": [
        141,
        196,
        194,
        143,
        204,
        114,
        90,
        226
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "epoch",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  112,
                  111,
                  99,
                  104
                ]
              },
              {
                "kind": "account",
                "path": "epoch.index",
                "account": "epoch"
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
      "name": "voteRoot",
      "discriminator": [
        84,
        39,
        101,
        24,
        102,
        244,
        191,
        247
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "epoch",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  112,
                  111,
                  99,
                  104
                ]
              },
              {
                "kind": "account",
                "path": "epoch.index",
                "account": "epoch"
              }
            ]
          }
        },
        {
          "name": "oracle",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "root",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "maxTotalClaim",
          "type": "u64"
        }
      ]
    },
    {
      "name": "withdrawFees",
      "docs": [
        "Sweeps collected claim fees to the configured fee account (permissionless)."
      ],
      "discriminator": [
        198,
        212,
        171,
        109,
        144,
        215,
        174,
        89
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  45,
                  99,
                  111,
                  110,
                  102,
                  105,
                  103
                ]
              }
            ]
          }
        },
        {
          "name": "feeVault",
          "writable": true,
          "relations": [
            "config"
          ]
        },
        {
          "name": "feeDestination",
          "writable": true,
          "relations": [
            "config"
          ]
        },
        {
          "name": "vcoinMint",
          "relations": [
            "config"
          ]
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "claimStatus",
      "discriminator": [
        22,
        183,
        249,
        157,
        247,
        95,
        150,
        96
      ]
    },
    {
      "name": "epoch",
      "discriminator": [
        93,
        83,
        120,
        89,
        151,
        138,
        152,
        108
      ]
    },
    {
      "name": "poolConfig",
      "discriminator": [
        26,
        108,
        14,
        123,
        116,
        230,
        129,
        43
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
      "name": "epochClawedBack",
      "discriminator": [
        254,
        108,
        134,
        173,
        101,
        1,
        30,
        24
      ]
    },
    {
      "name": "epochStarted",
      "discriminator": [
        46,
        87,
        217,
        228,
        8,
        115,
        2,
        97
      ]
    },
    {
      "name": "feeChangeApplied",
      "discriminator": [
        29,
        161,
        73,
        128,
        118,
        93,
        106,
        11
      ]
    },
    {
      "name": "feeChangeCancelled",
      "discriminator": [
        161,
        33,
        166,
        220,
        65,
        151,
        132,
        41
      ]
    },
    {
      "name": "feeChangeProposed",
      "discriminator": [
        227,
        197,
        50,
        240,
        150,
        215,
        223,
        243
      ]
    },
    {
      "name": "feesWithdrawn",
      "discriminator": [
        234,
        15,
        0,
        119,
        148,
        241,
        40,
        21
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
      "name": "nextEpochBudgetSet",
      "discriminator": [
        54,
        138,
        88,
        69,
        146,
        206,
        164,
        242
      ]
    },
    {
      "name": "oracleRegistered",
      "discriminator": [
        97,
        225,
        241,
        71,
        180,
        49,
        38,
        60
      ]
    },
    {
      "name": "oracleRemoved",
      "discriminator": [
        62,
        112,
        125,
        81,
        128,
        93,
        194,
        96
      ]
    },
    {
      "name": "paramsUpdated",
      "discriminator": [
        2,
        163,
        138,
        99,
        135,
        11,
        136,
        169
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
      "name": "poolInitialized",
      "discriminator": [
        100,
        118,
        173,
        87,
        12,
        198,
        254,
        229
      ]
    },
    {
      "name": "rewardClaimed",
      "discriminator": [
        49,
        28,
        87,
        84,
        158,
        48,
        229,
        175
      ]
    },
    {
      "name": "rootActivated",
      "discriminator": [
        24,
        59,
        236,
        54,
        131,
        27,
        149,
        19
      ]
    },
    {
      "name": "rootProposed",
      "discriminator": [
        42,
        37,
        28,
        46,
        99,
        215,
        183,
        163
      ]
    },
    {
      "name": "rootQuorumSet",
      "discriminator": [
        13,
        138,
        123,
        252,
        244,
        216,
        189,
        120
      ]
    },
    {
      "name": "rootVetoed",
      "discriminator": [
        104,
        203,
        229,
        184,
        178,
        81,
        64,
        117
      ]
    },
    {
      "name": "rootVoted",
      "discriminator": [
        107,
        145,
        234,
        68,
        199,
        173,
        224,
        233
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "paused",
      "msg": "Rewards are paused"
    },
    {
      "code": 6001,
      "name": "invalidProgramData",
      "msg": "Program data account does not belong to this program"
    },
    {
      "code": 6002,
      "name": "invalidVCoinMint",
      "msg": "VCoin mint must be the vcoin-token program's mint PDA"
    },
    {
      "code": 6003,
      "name": "invalidParams",
      "msg": "Parameters out of bounds"
    },
    {
      "code": 6004,
      "name": "feeTooHigh",
      "msg": "Fee above 1%"
    },
    {
      "code": 6005,
      "name": "invalidOracle",
      "msg": "Invalid oracle key"
    },
    {
      "code": 6006,
      "name": "oracleExists",
      "msg": "Oracle already registered"
    },
    {
      "code": 6007,
      "name": "tooManyOracles",
      "msg": "All oracle slots are used"
    },
    {
      "code": 6008,
      "name": "oracleNotFound",
      "msg": "Oracle not registered"
    },
    {
      "code": 6009,
      "name": "invalidQuorum",
      "msg": "Quorum must be between 2 and the number of registered oracles"
    },
    {
      "code": 6010,
      "name": "notActiveOracle",
      "msg": "Signer is not an active oracle"
    },
    {
      "code": 6011,
      "name": "epochTooSoon",
      "msg": "The previous epoch has not run its full duration"
    },
    {
      "code": 6012,
      "name": "reserveEmpty",
      "msg": "Reserve is empty"
    },
    {
      "code": 6013,
      "name": "budgetTooHigh",
      "msg": "Budget above the configured ceiling"
    },
    {
      "code": 6014,
      "name": "epochNotEnded",
      "msg": "The epoch has not ended"
    },
    {
      "code": 6015,
      "name": "notVoting",
      "msg": "Epoch is not accepting root votes"
    },
    {
      "code": 6016,
      "name": "invalidRoot",
      "msg": "Root must be non-zero and max_total_claim within (0, budget]"
    },
    {
      "code": 6017,
      "name": "noProposedRoot",
      "msg": "No root has reached quorum"
    },
    {
      "code": 6018,
      "name": "disputeDelayActive",
      "msg": "Dispute delay has not passed"
    },
    {
      "code": 6019,
      "name": "rootDeadlinePassed",
      "msg": "The root deadline passed; claw the budget back instead"
    },
    {
      "code": 6020,
      "name": "quorumLost",
      "msg": "Proposed root lost quorum"
    },
    {
      "code": 6021,
      "name": "notActive",
      "msg": "Epoch has no active root"
    },
    {
      "code": 6022,
      "name": "claimWindowClosed",
      "msg": "Claim window closed"
    },
    {
      "code": 6023,
      "name": "claimWindowOpen",
      "msg": "Claim window still open"
    },
    {
      "code": 6024,
      "name": "zeroAmount",
      "msg": "Amount must be greater than zero"
    },
    {
      "code": 6025,
      "name": "proofTooLong",
      "msg": "Proof too long"
    },
    {
      "code": 6026,
      "name": "invalidProof",
      "msg": "Invalid merkle proof"
    },
    {
      "code": 6027,
      "name": "exceedsMaxTotalClaim",
      "msg": "Claim exceeds the epoch's max_total_claim"
    },
    {
      "code": 6028,
      "name": "epochNotClosed",
      "msg": "Epoch not closed yet"
    },
    {
      "code": 6029,
      "name": "noPendingFee",
      "msg": "No fee change pending"
    },
    {
      "code": 6030,
      "name": "feeTimelockActive",
      "msg": "Fee change timelock still active"
    },
    {
      "code": 6031,
      "name": "reserveBelowBudget",
      "msg": "Reserve holds less than the scheduled budget; only the authority may start a smaller epoch"
    },
    {
      "code": 6032,
      "name": "notEnoughOracles",
      "msg": "Fewer oracles are registered than the root quorum"
    },
    {
      "code": 6033,
      "name": "nothingToWithdraw",
      "msg": "No fees to withdraw"
    },
    {
      "code": 6034,
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
      "name": "claimStatus",
      "docs": [
        "Marks one claimant's claim in one epoch, PDA `[\"claim\", epoch, claimant]` (SSC-10: no",
        "per-user epoch bitmap to run out of)."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "epoch",
            "type": "pubkey"
          },
          {
            "name": "claimant",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "claimedAt",
            "type": "i64"
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
      "name": "epoch",
      "docs": [
        "One distribution, PDA `[\"epoch\", index]`, with its own vault `[\"epoch-vault\", index]`",
        "holding exactly its budget, so no epoch can pay out another's funds (SSC-02)."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "type": "u64"
          },
          {
            "name": "budget",
            "type": "u64"
          },
          {
            "name": "vault",
            "type": "pubkey"
          },
          {
            "name": "startTs",
            "type": "i64"
          },
          {
            "name": "endTs",
            "type": "i64"
          },
          {
            "name": "pauseAccumAtStart",
            "type": "i64"
          },
          {
            "name": "claimWindow",
            "docs": [
              "`PoolParams` snapshot at start, so later parameter changes never move this epoch's",
              "deadlines."
            ],
            "type": "i64"
          },
          {
            "name": "disputeDelay",
            "type": "i64"
          },
          {
            "name": "votes",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "oracleVote"
                  }
                },
                7
              ]
            }
          },
          {
            "name": "proposedRoot",
            "docs": [
              "Candidate that reached quorum (`proposed_at == 0` = none)."
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "proposedMaxTotal",
            "type": "u64"
          },
          {
            "name": "proposedAt",
            "type": "i64"
          },
          {
            "name": "proposedVersion",
            "docs": [
              "`PoolConfig::oracle_set_version` when the candidate was proposed."
            ],
            "type": "u64"
          },
          {
            "name": "root",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "maxTotalClaim",
            "type": "u64"
          },
          {
            "name": "feeBps",
            "docs": [
              "Fee snapshot at activation, so every claimant of an epoch pays the same rate."
            ],
            "type": "u16"
          },
          {
            "name": "activatedAt",
            "type": "i64"
          },
          {
            "name": "claimDeadline",
            "type": "i64"
          },
          {
            "name": "pauseAccumAtActivation",
            "type": "i64"
          },
          {
            "name": "totalClaimed",
            "docs": [
              "Gross (before fee), like `max_total_claim` (SSC-13)."
            ],
            "type": "u64"
          },
          {
            "name": "numClaims",
            "type": "u64"
          },
          {
            "name": "state",
            "type": {
              "defined": {
                "name": "epochState"
              }
            }
          },
          {
            "name": "rentPayer",
            "type": "pubkey"
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
      "name": "epochClawedBack",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "type": "u64"
          },
          {
            "name": "returned",
            "type": "u64"
          },
          {
            "name": "totalClaimed",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "epochStarted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "type": "u64"
          },
          {
            "name": "budget",
            "type": "u64"
          },
          {
            "name": "startTs",
            "type": "i64"
          },
          {
            "name": "endTs",
            "type": "i64"
          },
          {
            "name": "nextEpochBudget",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "epochState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "voting"
          },
          {
            "name": "active"
          },
          {
            "name": "closed"
          }
        ]
      }
    },
    {
      "name": "feeChangeApplied",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "feeBps",
            "type": "u16"
          },
          {
            "name": "feeDestination",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "feeChangeCancelled",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "feeChangeProposed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "feeBps",
            "type": "u16"
          },
          {
            "name": "feeDestination",
            "type": "pubkey"
          },
          {
            "name": "eta",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "feesWithdrawn",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "destination",
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
      "name": "nextEpochBudgetSet",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "budget",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "oracleEntry",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "key",
            "docs": [
              "`Pubkey::default()` = empty slot."
            ],
            "type": "pubkey"
          },
          {
            "name": "activeFrom",
            "docs": [
              "Votes count only from this time on (registration + 24h)."
            ],
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "oracleRegistered",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "oracle",
            "type": "pubkey"
          },
          {
            "name": "activeFrom",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "oracleRemoved",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "oracle",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "oracleVote",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "oracle",
            "docs": [
              "`Pubkey::default()` = empty slot."
            ],
            "type": "pubkey"
          },
          {
            "name": "root",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "maxTotalClaim",
            "type": "u64"
          },
          {
            "name": "votedAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "paramsUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "params",
            "type": {
              "defined": {
                "name": "poolParams"
              }
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
      "name": "poolConfig",
      "docs": [
        "Singleton, PDA `[\"pool-config\"]`. Owns the reserve vault and every epoch vault."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "docs": [
              "Bootstrapped by the upgrade authority, handed to the Realms governance PDA."
            ],
            "type": {
              "defined": {
                "name": "adminState"
              }
            }
          },
          {
            "name": "vcoinMint",
            "type": "pubkey"
          },
          {
            "name": "reserveVault",
            "docs": [
              "PDA `[\"reserve-vault\"]`: the undistributed rewards (e.g. the 350M allocation)."
            ],
            "type": "pubkey"
          },
          {
            "name": "feeBps",
            "type": "u16"
          },
          {
            "name": "feeVault",
            "docs": [
              "PDA `[\"fee-vault\"]`: claim fees collect here, so a claim never depends on an",
              "outside account. Anyone may sweep it to `fee_destination`."
            ],
            "type": "pubkey"
          },
          {
            "name": "feeDestination",
            "type": "pubkey"
          },
          {
            "name": "pendingFeeBps",
            "docs": [
              "Timelocked fee change (`pending_fee_eta == 0` = none)."
            ],
            "type": "u16"
          },
          {
            "name": "pendingFeeDestination",
            "type": "pubkey"
          },
          {
            "name": "pendingFeeEta",
            "type": "i64"
          },
          {
            "name": "params",
            "type": {
              "defined": {
                "name": "poolParams"
              }
            }
          },
          {
            "name": "maxEpochBudget",
            "docs": [
              "Ceiling for any epoch's budget, fixed at initialization."
            ],
            "type": "u64"
          },
          {
            "name": "nextEpochBudget",
            "type": "u64"
          },
          {
            "name": "epochCount",
            "type": "u64"
          },
          {
            "name": "lastEpochStart",
            "type": "i64"
          },
          {
            "name": "oracles",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "oracleEntry"
                  }
                },
                7
              ]
            }
          },
          {
            "name": "rootQuorum",
            "type": "u8"
          },
          {
            "name": "oracleSetVersion",
            "docs": [
              "Bumped whenever the oracle set or the quorum changes; a proposed root is only valid",
              "under the version that produced it."
            ],
            "type": "u64"
          },
          {
            "name": "paused",
            "type": "bool"
          },
          {
            "name": "pausedSince",
            "type": "i64"
          },
          {
            "name": "pauseAccum",
            "docs": [
              "Total seconds spent paused, so deadlines can be extended by downtime (SSC-05)."
            ],
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "reserveBump",
            "type": "u8"
          },
          {
            "name": "feeVaultBump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "poolInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "reserveVault",
            "type": "pubkey"
          },
          {
            "name": "feeVault",
            "type": "pubkey"
          },
          {
            "name": "firstEpochBudget",
            "type": "u64"
          },
          {
            "name": "feeBps",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "poolParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "epochDuration",
            "type": "i64"
          },
          {
            "name": "claimWindow",
            "docs": [
              "How long claims stay open after a root activates (also the deadline for activating one)."
            ],
            "type": "i64"
          },
          {
            "name": "disputeDelay",
            "docs": [
              "Wait between a root reaching quorum and becoming claimable (DAO can veto meanwhile)."
            ],
            "type": "i64"
          },
          {
            "name": "decayBps",
            "docs": [
              "Each epoch's budget is the previous one × (1 − decay_bps / 10000)."
            ],
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "rewardClaimed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "type": "u64"
          },
          {
            "name": "claimant",
            "type": "pubkey"
          },
          {
            "name": "gross",
            "type": "u64"
          },
          {
            "name": "fee",
            "type": "u64"
          },
          {
            "name": "totalClaimed",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "rootActivated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "type": "u64"
          },
          {
            "name": "root",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "maxTotalClaim",
            "type": "u64"
          },
          {
            "name": "claimDeadline",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "rootProposed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "type": "u64"
          },
          {
            "name": "root",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "maxTotalClaim",
            "type": "u64"
          },
          {
            "name": "claimableAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "rootQuorumSet",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "quorum",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "rootVetoed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "type": "u64"
          },
          {
            "name": "root",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "rootVoted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "type": "u64"
          },
          {
            "name": "oracle",
            "type": "pubkey"
          },
          {
            "name": "root",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "maxTotalClaim",
            "type": "u64"
          },
          {
            "name": "votesForCandidate",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "constants": [
    {
      "name": "claimSeed",
      "type": "bytes",
      "value": "[99, 108, 97, 105, 109]"
    },
    {
      "name": "epochSeed",
      "type": "bytes",
      "value": "[101, 112, 111, 99, 104]"
    },
    {
      "name": "epochVaultSeed",
      "type": "bytes",
      "value": "[101, 112, 111, 99, 104, 45, 118, 97, 117, 108, 116]"
    },
    {
      "name": "feeVaultSeed",
      "type": "bytes",
      "value": "[102, 101, 101, 45, 118, 97, 117, 108, 116]"
    },
    {
      "name": "poolConfigSeed",
      "type": "bytes",
      "value": "[112, 111, 111, 108, 45, 99, 111, 110, 102, 105, 103]"
    },
    {
      "name": "reserveVaultSeed",
      "type": "bytes",
      "value": "[114, 101, 115, 101, 114, 118, 101, 45, 118, 97, 117, 108, 116]"
    }
  ]
};
