/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/vcoin_token.json`.
 */
export type VcoinToken = {
  "address": "Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn",
  "metadata": {
    "name": "vcoinToken",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "VCoin Token-2022 with Transfer Hook, Permanent Delegate, and Metadata"
  },
  "instructions": [
    {
      "name": "acceptAuthority",
      "docs": [
        "Completes a pending authority transfer after the 24h delay."
      ],
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
                  103,
                  45,
                  118,
                  50
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
      "docs": [
        "Cancels a pending authority transfer."
      ],
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
                  103,
                  45,
                  118,
                  50
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
        "Creates the config and the metadata-only VCoin mint. Upgrade authority only."
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
                  103,
                  45,
                  118,
                  50
                ]
              }
            ]
          }
        },
        {
          "name": "mint",
          "docs": [
            "The VCoin mint: Token-2022, 9 decimals, mint authority = config PDA,",
            "no freeze authority, and only the MetadataPointer extension (pointing at itself;",
            "the TokenMetadata extension is written by the handler)."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
          "address": "Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn"
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
          "name": "name",
          "type": "string"
        },
        {
          "name": "symbol",
          "type": "string"
        },
        {
          "name": "uri",
          "type": "string"
        }
      ]
    },
    {
      "name": "mintTokens",
      "docs": [
        "Mints VCoin to `destination`; authority only, capped at the 1B supply."
      ],
      "discriminator": [
        59,
        132,
        24,
        246,
        122,
        39,
        8,
        243
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
                  103,
                  45,
                  118,
                  50
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
          "name": "authority",
          "signer": true
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
      "docs": [
        "Starts a 24h two-step authority transfer."
      ],
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
                  103,
                  45,
                  118,
                  50
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
      "name": "setPaused",
      "docs": [
        "Pauses or resumes `mint_tokens`."
      ],
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
                  103,
                  45,
                  118,
                  50
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
      "name": "updateMetadata",
      "docs": [
        "Updates a token-metadata field (name, symbol or URI)."
      ],
      "discriminator": [
        170,
        182,
        43,
        239,
        97,
        78,
        225,
        186
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
                  103,
                  45,
                  118,
                  50
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
          "name": "authority",
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
          "name": "field",
          "type": {
            "defined": {
              "name": "metadataField"
            }
          }
        },
        {
          "name": "value",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "vCoinConfig",
      "discriminator": [
        91,
        194,
        43,
        151,
        200,
        225,
        194,
        113
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
      "name": "metadataUpdated",
      "discriminator": [
        132,
        36,
        215,
        246,
        166,
        90,
        189,
        44
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
      "name": "tokensMinted",
      "discriminator": [
        207,
        212,
        128,
        194,
        175,
        54,
        64,
        24
      ]
    },
    {
      "name": "vCoinInitialized",
      "discriminator": [
        89,
        204,
        65,
        206,
        161,
        125,
        219,
        148
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "tokenPaused",
      "msg": "Token minting is paused"
    },
    {
      "code": 6001,
      "name": "zeroAmount",
      "msg": "Amount must be greater than zero"
    },
    {
      "code": 6002,
      "name": "exceedsMaxSupply",
      "msg": "Minting would exceed the 1B VCoin supply cap"
    },
    {
      "code": 6003,
      "name": "invalidProgramData",
      "msg": "Program data account does not belong to this program"
    },
    {
      "code": 6004,
      "name": "metadataTooLong",
      "msg": "Metadata field exceeds its maximum length"
    },
    {
      "code": 6005,
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
      "name": "metadataField",
      "docs": [
        "Metadata fields the authority may update."
      ],
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "name"
          },
          {
            "name": "symbol"
          },
          {
            "name": "uri"
          }
        ]
      }
    },
    {
      "name": "metadataUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "field",
            "type": {
              "defined": {
                "name": "metadataField"
              }
            }
          },
          {
            "name": "value",
            "type": "string"
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
      "name": "tokensMinted",
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
          },
          {
            "name": "newSupply",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "vCoinConfig",
      "docs": [
        "Singleton configuration, PDA `[\"vcoin-config-v2\"]`. Also the VCoin mint authority",
        "and the metadata update authority."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "docs": [
              "Authority with a 24h two-step transfer (audit X-2)."
            ],
            "type": {
              "defined": {
                "name": "adminState"
              }
            }
          },
          {
            "name": "mint",
            "docs": [
              "The VCoin mint, PDA `[\"vcoin-mint\"]`, created by `initialize`."
            ],
            "type": "pubkey"
          },
          {
            "name": "paused",
            "docs": [
              "Pauses `mint_tokens`. Transfers are never pausable (the mint has no",
              "pausable/freeze capability by design)."
            ],
            "type": "bool"
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
      "name": "vCoinInitialized",
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
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "uri",
            "type": "string"
          }
        ]
      }
    }
  ],
  "constants": [
    {
      "name": "totalSupply",
      "docs": [
        "Hard supply cap: 1,000,000,000 VCoin in base units. Enforced against the mint's",
        "real on-chain supply, not a program-side counter (audit MON-8 / TOK-04)."
      ],
      "type": "u64",
      "value": "1000000000000000000"
    },
    {
      "name": "vcoinConfigSeed",
      "docs": [
        "Config PDA seed. `-v2` avoids the stale v1 `vcoin-config` account left on devnet."
      ],
      "type": "bytes",
      "value": "[118, 99, 111, 105, 110, 45, 99, 111, 110, 102, 105, 103, 45, 118, 50]"
    },
    {
      "name": "vcoinDecimals",
      "docs": [
        "VCoin decimals."
      ],
      "type": "u8",
      "value": "9"
    },
    {
      "name": "vcoinMintSeed",
      "docs": [
        "Mint PDA seed. The VCoin mint is created by this program at this address."
      ],
      "type": "bytes",
      "value": "[118, 99, 111, 105, 110, 45, 109, 105, 110, 116]"
    }
  ]
};
