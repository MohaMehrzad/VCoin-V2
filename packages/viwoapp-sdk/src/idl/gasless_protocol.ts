/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/gasless_protocol.json`.
 */
export type GaslessProtocol = {
  "address": "FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3",
  "metadata": {
    "name": "gaslessProtocol",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Gasless Layer - Paymaster and Session Keys for Zero-Friction UX"
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
                  103,
                  97,
                  115,
                  108,
                  101,
                  115,
                  115,
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
                  103,
                  97,
                  115,
                  108,
                  101,
                  115,
                  115,
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
      "name": "applyTreasury",
      "discriminator": [
        226,
        18,
        133,
        106,
        27,
        32,
        83,
        204
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
                  103,
                  97,
                  115,
                  108,
                  101,
                  115,
                  115,
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
          "name": "vcoinMint",
          "relations": [
            "config"
          ]
        },
        {
          "name": "newTreasury"
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
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
                  103,
                  97,
                  115,
                  108,
                  101,
                  115,
                  115,
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
      "name": "createSession",
      "discriminator": [
        242,
        193,
        143,
        179,
        150,
        25,
        122,
        227
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  97,
                  115,
                  108,
                  101,
                  115,
                  115,
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
          "name": "session",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  101,
                  115,
                  115,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "arg",
                "path": "targetProgram"
              },
              {
                "kind": "account",
                "path": "sessionSigner"
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "sessionSigner",
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
      "args": [
        {
          "name": "targetProgram",
          "type": "pubkey"
        },
        {
          "name": "scope",
          "type": "u16"
        },
        {
          "name": "validUntil",
          "type": "i64"
        }
      ]
    },
    {
      "name": "initialize",
      "docs": [
        "Upgrade authority only."
      ],
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
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
                  103,
                  97,
                  115,
                  108,
                  101,
                  115,
                  115,
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
          "name": "treasury"
        },
        {
          "name": "program",
          "address": "FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3"
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
          "name": "vcoinFeePerTx",
          "type": "u64"
        }
      ]
    },
    {
      "name": "payVcoinFee",
      "discriminator": [
        148,
        237,
        210,
        122,
        129,
        59,
        87,
        123
      ],
      "accounts": [
        {
          "name": "config",
          "docs": [
            "Read-only: paying a fee never write-locks global state (audit GAS-09)."
          ],
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  97,
                  115,
                  108,
                  101,
                  115,
                  115,
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
          "name": "vcoinMint",
          "relations": [
            "config"
          ]
        },
        {
          "name": "feeVault",
          "writable": true,
          "relations": [
            "config"
          ]
        },
        {
          "name": "payerVcoin",
          "writable": true
        },
        {
          "name": "payer",
          "signer": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": [
        {
          "name": "maxFee",
          "type": "u64"
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
                  103,
                  97,
                  115,
                  108,
                  101,
                  115,
                  115,
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
      "name": "revokeSession",
      "discriminator": [
        86,
        92,
        198,
        120,
        144,
        2,
        7,
        194
      ],
      "accounts": [
        {
          "name": "session",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  101,
                  115,
                  115,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "session.target_program",
                "account": "sessionToken"
              },
              {
                "kind": "account",
                "path": "session.session_signer",
                "account": "sessionToken"
              },
              {
                "kind": "account",
                "path": "session.authority",
                "account": "sessionToken"
              }
            ]
          }
        },
        {
          "name": "payer",
          "writable": true,
          "relations": [
            "session"
          ]
        },
        {
          "name": "revoker",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "scheduleFee",
      "discriminator": [
        67,
        220,
        176,
        104,
        30,
        254,
        14,
        75
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
                  103,
                  97,
                  115,
                  108,
                  101,
                  115,
                  115,
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
          "name": "newFee",
          "type": "u64"
        }
      ]
    },
    {
      "name": "scheduleTreasury",
      "discriminator": [
        26,
        251,
        92,
        172,
        144,
        95,
        77,
        68
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
                  103,
                  97,
                  115,
                  108,
                  101,
                  115,
                  115,
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
          "name": "newTreasury",
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
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  97,
                  115,
                  108,
                  101,
                  115,
                  115,
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
      "name": "withdrawFees",
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
                  103,
                  97,
                  115,
                  108,
                  101,
                  115,
                  115,
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
          "name": "vcoinMint",
          "relations": [
            "config"
          ]
        },
        {
          "name": "feeVault",
          "writable": true,
          "relations": [
            "config"
          ]
        },
        {
          "name": "treasury",
          "writable": true,
          "relations": [
            "config"
          ]
        },
        {
          "name": "tokenProgram",
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "gaslessConfig",
      "discriminator": [
        127,
        255,
        120,
        55,
        81,
        121,
        160,
        49
      ]
    },
    {
      "name": "sessionToken",
      "discriminator": [
        233,
        4,
        115,
        14,
        46,
        21,
        1,
        15
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
      "name": "feeChangeScheduled",
      "discriminator": [
        13,
        77,
        179,
        118,
        246,
        34,
        119,
        205
      ]
    },
    {
      "name": "feeChanged",
      "discriminator": [
        103,
        252,
        132,
        250,
        1,
        49,
        116,
        145
      ]
    },
    {
      "name": "feePaid",
      "discriminator": [
        159,
        12,
        52,
        212,
        249,
        36,
        24,
        18
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
      "name": "gaslessInitialized",
      "discriminator": [
        0,
        194,
        64,
        64,
        32,
        87,
        160,
        245
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
      "name": "sessionCreated",
      "discriminator": [
        107,
        111,
        254,
        25,
        21,
        122,
        220,
        225
      ]
    },
    {
      "name": "sessionRevoked",
      "discriminator": [
        90,
        48,
        35,
        234,
        203,
        192,
        126,
        211
      ]
    },
    {
      "name": "treasuryChangeScheduled",
      "discriminator": [
        195,
        70,
        226,
        198,
        179,
        132,
        185,
        154
      ]
    },
    {
      "name": "treasuryChanged",
      "discriminator": [
        252,
        2,
        55,
        56,
        106,
        205,
        227,
        47
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "paused",
      "msg": "Gasless protocol is paused"
    },
    {
      "code": 6001,
      "name": "invalidScope",
      "msg": "Scope must be non-empty and use only allowed bits"
    },
    {
      "code": 6002,
      "name": "invalidValidity",
      "msg": "Session validity must be in the future and at most 24h"
    },
    {
      "code": 6003,
      "name": "sessionExpired",
      "msg": "Session has expired"
    },
    {
      "code": 6004,
      "name": "sessionMismatch",
      "msg": "Session does not belong to this authority, signer or program"
    },
    {
      "code": 6005,
      "name": "actionNotInScope",
      "msg": "Exactly one in-scope action bit must be requested"
    },
    {
      "code": 6006,
      "name": "revokeNotAllowed",
      "msg": "Only the session authority may revoke an unexpired session"
    },
    {
      "code": 6007,
      "name": "feeAboveMax",
      "msg": "Fee exceeds the caller's max_fee"
    },
    {
      "code": 6008,
      "name": "feeTooHigh",
      "msg": "Fee above the allowed maximum"
    },
    {
      "code": 6009,
      "name": "nothingScheduled",
      "msg": "No change is scheduled"
    },
    {
      "code": 6010,
      "name": "timelockActive",
      "msg": "The scheduled change is not yet effective"
    },
    {
      "code": 6011,
      "name": "invalidProgramData",
      "msg": "Program data account does not belong to this program"
    },
    {
      "code": 6012,
      "name": "zeroAmount",
      "msg": "Amount must be greater than zero"
    },
    {
      "code": 6013,
      "name": "overflow",
      "msg": "Arithmetic overflow"
    }
  ],
  "types": [
    {
      "name": "adminState",
      "docs": [
        "Authority with a two-step, time-locked transfer.",
        "",
        "`propose` (current authority) → wait [`AUTHORITY_TRANSFER_DELAY`] → `accept`",
        "(new authority). The current authority can `cancel` at any time before acceptance."
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
      "name": "feeChangeScheduled",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "newFee",
            "type": "u64"
          },
          {
            "name": "effectiveAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "feeChanged",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "oldFee",
            "type": "u64"
          },
          {
            "name": "newFee",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "feePaid",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "payer",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
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
            "name": "treasury",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "gaslessConfig",
      "docs": [
        "Singleton configuration, PDA `[\"gasless-config\"]`."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
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
            "name": "feeVault",
            "docs": [
              "VCoin vault (PDA `[\"fee-vault\"]`, authority = this config) collecting fees."
            ],
            "type": "pubkey"
          },
          {
            "name": "treasury",
            "docs": [
              "VCoin token account that `withdraw_fees` pays out to."
            ],
            "type": "pubkey"
          },
          {
            "name": "pendingTreasury",
            "type": "pubkey"
          },
          {
            "name": "treasuryEta",
            "type": "i64"
          },
          {
            "name": "vcoinFeePerTx",
            "docs": [
              "VCoin charged per `pay_vcoin_fee` (base units)."
            ],
            "type": "u64"
          },
          {
            "name": "pendingFee",
            "type": "u64"
          },
          {
            "name": "feeEta",
            "type": "i64"
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
      "name": "gaslessInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "config",
            "type": "pubkey"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "feeVault",
            "type": "pubkey"
          },
          {
            "name": "treasury",
            "type": "pubkey"
          },
          {
            "name": "vcoinFeePerTx",
            "type": "u64"
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
      "name": "sessionCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "session",
            "type": "pubkey"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "sessionSigner",
            "type": "pubkey"
          },
          {
            "name": "targetProgram",
            "type": "pubkey"
          },
          {
            "name": "scope",
            "type": "u16"
          },
          {
            "name": "validUntil",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "sessionRevoked",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "session",
            "type": "pubkey"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "revokedBy",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "sessionToken",
      "docs": [
        "A scoped, expiring session key, PDA",
        "`[\"session\", target_program, session_signer, authority]` (MagicBlock gpl-session model).",
        "",
        "Consuming programs load it as `Account<SessionToken>` with",
        "`seeds::program = gasless_protocol::ID` and call [`SessionToken::verify`]."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "docs": [
              "The wallet the session acts for."
            ],
            "type": "pubkey"
          },
          {
            "name": "targetProgram",
            "docs": [
              "The only program this session may be used with."
            ],
            "type": "pubkey"
          },
          {
            "name": "sessionSigner",
            "docs": [
              "Ephemeral key that signs on the authority's behalf."
            ],
            "type": "pubkey"
          },
          {
            "name": "scope",
            "type": "u16"
          },
          {
            "name": "validUntil",
            "type": "i64"
          },
          {
            "name": "payer",
            "docs": [
              "Account that paid rent; refunded on revoke."
            ],
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
      "name": "treasuryChangeScheduled",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "newTreasury",
            "type": "pubkey"
          },
          {
            "name": "effectiveAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "treasuryChanged",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "oldTreasury",
            "type": "pubkey"
          },
          {
            "name": "newTreasury",
            "type": "pubkey"
          }
        ]
      }
    }
  ],
  "constants": [
    {
      "name": "feeVaultSeed",
      "type": "bytes",
      "value": "[102, 101, 101, 45, 118, 97, 117, 108, 116]"
    },
    {
      "name": "gaslessConfigSeed",
      "type": "bytes",
      "value": "[103, 97, 115, 108, 101, 115, 115, 45, 99, 111, 110, 102, 105, 103]"
    },
    {
      "name": "maxSessionDuration",
      "docs": [
        "Longest allowed session (24 h), following MagicBlock gpl-session practice."
      ],
      "type": "i64",
      "value": "86400"
    },
    {
      "name": "maxVcoinFeePerTx",
      "docs": [
        "Upper bound on the per-transaction VCoin fee the admin may schedule (100 VCoin)."
      ],
      "type": "u64",
      "value": "100000000000"
    },
    {
      "name": "paramChangeDelay",
      "docs": [
        "Delay before a fee or treasury change takes effect."
      ],
      "type": "i64",
      "value": "86400"
    },
    {
      "name": "scopeContent",
      "type": "u16",
      "value": "4"
    },
    {
      "name": "scopeFollow",
      "type": "u16",
      "value": "128"
    },
    {
      "name": "scopeVouch",
      "docs": [
        "Session scopes. Sessions may only authorize low-risk, non-value-moving social actions:",
        "never transfers, tips, withdrawals, claims, staking or votes (those always need the",
        "main wallet). Bit positions are kept from v2 for client compatibility."
      ],
      "type": "u16",
      "value": "2"
    },
    {
      "name": "sessionSeed",
      "docs": [
        "Session PDA: `[\"session\", target_program, session_signer, authority]`."
      ],
      "type": "bytes",
      "value": "[115, 101, 115, 115, 105, 111, 110]"
    },
    {
      "name": "validScopeMask",
      "docs": [
        "Union of every allowed scope bit; any other bit is rejected."
      ],
      "type": "u16",
      "value": "134"
    }
  ]
};
