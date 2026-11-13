/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/vevcoin_token.json`.
 */
export type VevcoinToken = {
  "address": "FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx",
  "metadata": {
    "name": "vevcoinToken",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "veVCoin Soulbound Token with Non-Transferable Extension"
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
                  118,
                  101,
                  118,
                  99,
                  111,
                  105,
                  110,
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
      "name": "burnVevcoin",
      "docs": [
        "Burns `min(amount, balance)` receipts. Signer must be the staking registrar PDA."
      ],
      "discriminator": [
        83,
        99,
        164,
        1,
        222,
        148,
        225,
        85
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  101,
                  118,
                  99,
                  111,
                  105,
                  110,
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
          "name": "mint",
          "writable": true,
          "relations": [
            "config"
          ]
        },
        {
          "name": "source",
          "writable": true
        },
        {
          "name": "stakingAuthority",
          "signer": true,
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
                  118,
                  101,
                  118,
                  99,
                  111,
                  105,
                  110,
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
      "name": "initialize",
      "docs": [
        "Creates the config and the veVCoin mint. Upgrade authority only."
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
                  118,
                  101,
                  118,
                  99,
                  111,
                  105,
                  110,
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
          "name": "mint",
          "docs": [
            "handler (Anchor 0.32 has no NonTransferable init constraint)."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  101,
                  118,
                  99,
                  111,
                  105,
                  110,
                  45,
                  109,
                  105,
                  110,
                  116
                ]
              }
            ]
          }
        },
        {
          "name": "program",
          "address": "FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx"
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
      "name": "mintVevcoin",
      "docs": [
        "Mints receipts. Signer must be the staking registrar PDA."
      ],
      "discriminator": [
        14,
        183,
        224,
        164,
        59,
        30,
        74,
        130
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  101,
                  118,
                  99,
                  111,
                  105,
                  110,
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
          "name": "mint",
          "writable": true,
          "relations": [
            "config"
          ]
        },
        {
          "name": "destination",
          "writable": true
        },
        {
          "name": "stakingAuthority",
          "signer": true,
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
                  118,
                  101,
                  118,
                  99,
                  111,
                  105,
                  110,
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
    }
  ],
  "accounts": [
    {
      "name": "veVCoinConfig",
      "discriminator": [
        224,
        204,
        184,
        106,
        133,
        166,
        3,
        16
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
      "name": "veVCoinBurned",
      "discriminator": [
        63,
        242,
        221,
        85,
        248,
        215,
        232,
        0
      ]
    },
    {
      "name": "veVCoinInitialized",
      "discriminator": [
        142,
        220,
        181,
        142,
        76,
        187,
        194,
        202
      ]
    },
    {
      "name": "veVCoinMinted",
      "discriminator": [
        250,
        98,
        197,
        105,
        23,
        8,
        61,
        36
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "unauthorized",
      "msg": "Only the staking registrar may mint or burn veVCoin"
    },
    {
      "code": 6001,
      "name": "zeroAmount",
      "msg": "Amount must be greater than zero"
    },
    {
      "code": 6002,
      "name": "invalidProgramData",
      "msg": "Program data account does not belong to this program"
    },
    {
      "code": 6003,
      "name": "invalidMint",
      "msg": "Mint account does not match the expected PDA"
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
      "name": "veVCoinBurned",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "source",
            "type": "pubkey"
          },
          {
            "name": "requested",
            "docs": [
              "Amount the staking program asked to burn."
            ],
            "type": "u64"
          },
          {
            "name": "burned",
            "docs": [
              "Amount actually burned: `min(requested, balance)` (a holder may have burned",
              "their own receipt; that must never block an unstake)."
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "veVCoinConfig",
      "docs": [
        "Singleton configuration, PDA `[\"vevcoin-config\"]`. It is the veVCoin mint authority",
        "and permanent delegate, so only this program (on the registrar's instruction) can",
        "mint or burn receipts."
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
            "name": "mint",
            "docs": [
              "veVCoin mint, PDA `[\"vevcoin-mint\"]`."
            ],
            "type": "pubkey"
          },
          {
            "name": "stakingAuthority",
            "docs": [
              "Staking registrar PDA `[\"registrar\"]` under the staking program, derived once at init."
            ],
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "mintBump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "veVCoinInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "config",
            "type": "pubkey"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "stakingAuthority",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "veVCoinMinted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "destination",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "constants": [
    {
      "name": "stakingRegistrarSeed",
      "docs": [
        "Seed of the staking program's registrar PDA (must match `staking_protocol::REGISTRAR_SEED`)."
      ],
      "type": "bytes",
      "value": "[114, 101, 103, 105, 115, 116, 114, 97, 114]"
    },
    {
      "name": "vevcoinConfigSeed",
      "type": "bytes",
      "value": "[118, 101, 118, 99, 111, 105, 110, 45, 99, 111, 110, 102, 105, 103]"
    },
    {
      "name": "vevcoinDecimals",
      "type": "u8",
      "value": "9"
    },
    {
      "name": "vevcoinMintSeed",
      "docs": [
        "Mint PDA seed. The veVCoin mint is created by this program at this address."
      ],
      "type": "bytes",
      "value": "[118, 101, 118, 99, 111, 105, 110, 45, 109, 105, 110, 116]"
    }
  ]
};
