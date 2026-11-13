/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/vilink_protocol.json`.
 */
export type VilinkProtocol = {
  "address": "CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7",
  "metadata": {
    "name": "vilinkProtocol",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "ViLink Protocol - Cross-dApp Action Deep Links"
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
                  105,
                  108,
                  105,
                  110,
                  107,
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
      "name": "applyConfig",
      "discriminator": [
        121,
        42,
        53,
        8,
        51,
        150,
        105,
        249
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
                  105,
                  108,
                  105,
                  110,
                  107,
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
                  118,
                  105,
                  108,
                  105,
                  110,
                  107,
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
      "name": "cancelConfig",
      "discriminator": [
        91,
        124,
        207,
        75,
        179,
        108,
        227,
        13
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
                  105,
                  108,
                  105,
                  110,
                  107,
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
      "name": "closeAction",
      "discriminator": [
        68,
        91,
        38,
        183,
        124,
        74,
        239,
        136
      ],
      "accounts": [
        {
          "name": "action",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  99,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "creator"
              },
              {
                "kind": "account",
                "path": "action.nonce",
                "account": "viLinkAction"
              }
            ]
          }
        },
        {
          "name": "creator",
          "writable": true,
          "signer": true,
          "relations": [
            "action"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "closeReceipt",
      "discriminator": [
        126,
        254,
        244,
        203,
        124,
        164,
        134,
        89
      ],
      "accounts": [
        {
          "name": "receipt",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  120,
                  101,
                  99
                ]
              },
              {
                "kind": "account",
                "path": "receipt.action",
                "account": "executionReceipt"
              },
              {
                "kind": "account",
                "path": "receipt.executor",
                "account": "executionReceipt"
              }
            ]
          }
        },
        {
          "name": "action"
        },
        {
          "name": "rentPayer",
          "writable": true,
          "relations": [
            "receipt"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "createAction",
      "discriminator": [
        125,
        159,
        123,
        141,
        6,
        144,
        175,
        4
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
                  105,
                  108,
                  105,
                  110,
                  107,
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
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "creatorStats",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  45,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "creator"
              }
            ]
          }
        },
        {
          "name": "action",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  99,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "creator"
              },
              {
                "kind": "arg",
                "path": "nonce"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "actionType",
          "type": "u8"
        },
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "target",
          "type": "pubkey"
        },
        {
          "name": "metadataHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "expirySecs",
          "type": "i64"
        },
        {
          "name": "maxExecutions",
          "type": "u32"
        },
        {
          "name": "nonce",
          "type": "u64"
        }
      ]
    },
    {
      "name": "executeFollow",
      "discriminator": [
        231,
        76,
        146,
        132,
        214,
        83,
        81,
        42
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
                  105,
                  108,
                  105,
                  110,
                  107,
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
          "name": "action",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  99,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "action.creator",
                "account": "viLinkAction"
              },
              {
                "kind": "account",
                "path": "action.nonce",
                "account": "viLinkAction"
              }
            ]
          }
        },
        {
          "name": "executor"
        },
        {
          "name": "signer",
          "signer": true
        },
        {
          "name": "session",
          "optional": true,
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
                "kind": "const",
                "value": [
                  167,
                  24,
                  61,
                  230,
                  184,
                  233,
                  202,
                  200,
                  92,
                  11,
                  173,
                  229,
                  218,
                  26,
                  157,
                  131,
                  35,
                  21,
                  206,
                  35,
                  143,
                  253,
                  94,
                  33,
                  233,
                  96,
                  63,
                  234,
                  109,
                  74,
                  29,
                  44
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              },
              {
                "kind": "account",
                "path": "executor"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                217,
                29,
                51,
                122,
                28,
                119,
                61,
                137,
                93,
                222,
                103,
                65,
                50,
                193,
                72,
                195,
                52,
                177,
                177,
                245,
                170,
                24,
                229,
                79,
                25,
                134,
                25,
                82,
                160,
                148,
                208,
                10
              ]
            }
          }
        },
        {
          "name": "receipt",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  120,
                  101,
                  99
                ]
              },
              {
                "kind": "account",
                "path": "action"
              },
              {
                "kind": "account",
                "path": "executor"
              }
            ]
          }
        },
        {
          "name": "executorStats",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  45,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "executor"
              }
            ]
          }
        },
        {
          "name": "targetStats",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  45,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "action.target",
                "account": "viLinkAction"
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
      "args": []
    },
    {
      "name": "executeTip",
      "discriminator": [
        137,
        79,
        103,
        40,
        119,
        6,
        123,
        74
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
                  105,
                  108,
                  105,
                  110,
                  107,
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
          "name": "action",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  99,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "action.creator",
                "account": "viLinkAction"
              },
              {
                "kind": "account",
                "path": "action.nonce",
                "account": "viLinkAction"
              }
            ]
          }
        },
        {
          "name": "executor",
          "writable": true,
          "signer": true
        },
        {
          "name": "executorToken",
          "writable": true
        },
        {
          "name": "targetToken",
          "writable": true
        },
        {
          "name": "treasury",
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
          "name": "receipt",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  120,
                  101,
                  99
                ]
              },
              {
                "kind": "account",
                "path": "action"
              },
              {
                "kind": "account",
                "path": "executor"
              }
            ]
          }
        },
        {
          "name": "executorStats",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  45,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "executor"
              }
            ]
          }
        },
        {
          "name": "targetStats",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  45,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "action.target",
                "account": "viLinkAction"
              }
            ]
          }
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
      "name": "executeVouch",
      "discriminator": [
        82,
        140,
        77,
        228,
        228,
        183,
        179,
        199
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
                  105,
                  108,
                  105,
                  110,
                  107,
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
          "name": "action",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  99,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "action.creator",
                "account": "viLinkAction"
              },
              {
                "kind": "account",
                "path": "action.nonce",
                "account": "viLinkAction"
              }
            ]
          }
        },
        {
          "name": "executor"
        },
        {
          "name": "signer",
          "signer": true
        },
        {
          "name": "session",
          "optional": true,
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
                "kind": "const",
                "value": [
                  167,
                  24,
                  61,
                  230,
                  184,
                  233,
                  202,
                  200,
                  92,
                  11,
                  173,
                  229,
                  218,
                  26,
                  157,
                  131,
                  35,
                  21,
                  206,
                  35,
                  143,
                  253,
                  94,
                  33,
                  233,
                  96,
                  63,
                  234,
                  109,
                  74,
                  29,
                  44
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              },
              {
                "kind": "account",
                "path": "executor"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                217,
                29,
                51,
                122,
                28,
                119,
                61,
                137,
                93,
                222,
                103,
                65,
                50,
                193,
                72,
                195,
                52,
                177,
                177,
                245,
                170,
                24,
                229,
                79,
                25,
                134,
                25,
                82,
                160,
                148,
                208,
                10
              ]
            }
          }
        },
        {
          "name": "receipt",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  120,
                  101,
                  99
                ]
              },
              {
                "kind": "account",
                "path": "action"
              },
              {
                "kind": "account",
                "path": "executor"
              }
            ]
          }
        },
        {
          "name": "executorStats",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  45,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "executor"
              }
            ]
          }
        },
        {
          "name": "targetStats",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114,
                  45,
                  115,
                  116,
                  97,
                  116,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "action.target",
                "account": "viLinkAction"
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
      "args": []
    },
    {
      "name": "initialize",
      "docs": [
        "Upgrade authority only; VCoin mint pinned."
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
                  105,
                  108,
                  105,
                  110,
                  107,
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
          "name": "treasury"
        },
        {
          "name": "program",
          "address": "CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7"
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
                  118,
                  105,
                  108,
                  105,
                  110,
                  107,
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
      "name": "proposeConfig",
      "discriminator": [
        163,
        247,
        238,
        160,
        236,
        129,
        153,
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
                  105,
                  108,
                  105,
                  110,
                  107,
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
          "name": "treasury"
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
      "name": "setEnabledActions",
      "discriminator": [
        130,
        16,
        176,
        34,
        80,
        175,
        142,
        135
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
                  105,
                  108,
                  105,
                  110,
                  107,
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
          "name": "enabledActions",
          "type": "u8"
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
                  118,
                  105,
                  108,
                  105,
                  110,
                  107,
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
                  118,
                  105,
                  108,
                  105,
                  110,
                  107,
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
    }
  ],
  "accounts": [
    {
      "name": "executionReceipt",
      "discriminator": [
        126,
        220,
        166,
        135,
        248,
        208,
        202,
        171
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
    },
    {
      "name": "userActionStats",
      "discriminator": [
        32,
        234,
        88,
        84,
        10,
        113,
        53,
        29
      ]
    },
    {
      "name": "viLinkAction",
      "discriminator": [
        11,
        137,
        230,
        211,
        5,
        70,
        36,
        146
      ]
    },
    {
      "name": "viLinkConfig",
      "discriminator": [
        132,
        173,
        139,
        223,
        155,
        231,
        22,
        32
      ]
    }
  ],
  "events": [
    {
      "name": "actionClosed",
      "discriminator": [
        12,
        207,
        175,
        173,
        7,
        142,
        107,
        78
      ]
    },
    {
      "name": "actionCreated",
      "discriminator": [
        224,
        166,
        37,
        66,
        93,
        1,
        115,
        234
      ]
    },
    {
      "name": "actionExecuted",
      "discriminator": [
        116,
        101,
        146,
        36,
        160,
        153,
        182,
        233
      ]
    },
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
      "name": "configChangeApplied",
      "discriminator": [
        65,
        124,
        47,
        248,
        1,
        25,
        230,
        154
      ]
    },
    {
      "name": "configChangeCancelled",
      "discriminator": [
        66,
        48,
        129,
        141,
        104,
        33,
        113,
        2
      ]
    },
    {
      "name": "configChangeProposed",
      "discriminator": [
        57,
        212,
        96,
        3,
        9,
        29,
        225,
        198
      ]
    },
    {
      "name": "enabledActionsSet",
      "discriminator": [
        170,
        2,
        142,
        61,
        212,
        94,
        54,
        127
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
      "name": "viLinkInitialized",
      "discriminator": [
        66,
        108,
        34,
        154,
        158,
        14,
        215,
        34
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "paused",
      "msg": "ViLink is paused"
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
      "name": "invalidFee",
      "msg": "Fee must be between 0.1% and 10%"
    },
    {
      "code": 6004,
      "name": "actionDisabled",
      "msg": "Action type is unknown or disabled"
    },
    {
      "code": 6005,
      "name": "wrongActionType",
      "msg": "Wrong action type for this instruction"
    },
    {
      "code": 6006,
      "name": "invalidAmount",
      "msg": "Tip amount must be between 0.1 and 10,000 VCoin; other actions carry no amount"
    },
    {
      "code": 6007,
      "name": "invalidTarget",
      "msg": "Invalid target"
    },
    {
      "code": 6008,
      "name": "invalidExpiry",
      "msg": "Expiry must be between 1 hour and 7 days"
    },
    {
      "code": 6009,
      "name": "invalidMaxExecutions",
      "msg": "max_executions must be between 1 and 10,000"
    },
    {
      "code": 6010,
      "name": "invalidNonce",
      "msg": "Nonce must equal the creator's actions_created"
    },
    {
      "code": 6011,
      "name": "actionClosed",
      "msg": "Action expired or fully executed"
    },
    {
      "code": 6012,
      "name": "actionStillOpen",
      "msg": "Action is still open"
    },
    {
      "code": 6013,
      "name": "invalidExecutor",
      "msg": "The creator or target cannot execute this action"
    },
    {
      "code": 6014,
      "name": "unauthorized",
      "msg": "Signer is neither the executor nor a valid session for the executor"
    },
    {
      "code": 6015,
      "name": "noPendingChange",
      "msg": "No config change pending"
    },
    {
      "code": 6016,
      "name": "timelockActive",
      "msg": "Config change timelock still active"
    },
    {
      "code": 6017,
      "name": "overflow",
      "msg": "Arithmetic overflow"
    }
  ],
  "types": [
    {
      "name": "actionClosed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "action",
            "type": "pubkey"
          },
          {
            "name": "executionCount",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "actionCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "action",
            "type": "pubkey"
          },
          {
            "name": "actionId",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "actionType",
            "type": "u8"
          },
          {
            "name": "target",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "feeBps",
            "type": "u16"
          },
          {
            "name": "expiresAt",
            "type": "i64"
          },
          {
            "name": "maxExecutions",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "actionExecuted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "action",
            "type": "pubkey"
          },
          {
            "name": "actionType",
            "type": "u8"
          },
          {
            "name": "executor",
            "type": "pubkey"
          },
          {
            "name": "target",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          },
          {
            "name": "fee",
            "type": "u64"
          },
          {
            "name": "executionCount",
            "type": "u32"
          },
          {
            "name": "signer",
            "type": "pubkey"
          }
        ]
      }
    },
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
      "name": "configChangeApplied",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "feeBps",
            "type": "u16"
          },
          {
            "name": "treasury",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "configChangeCancelled",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "configChangeProposed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "feeBps",
            "type": "u16"
          },
          {
            "name": "treasury",
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
      "name": "enabledActionsSet",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "enabledActions",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "executionReceipt",
      "docs": [
        "One execution of an action by one executor, PDA `[\"exec\", action, executor]`: each",
        "executor can act on a link once, whatever its type (audit VL-3)."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "action",
            "type": "pubkey"
          },
          {
            "name": "executor",
            "type": "pubkey"
          },
          {
            "name": "executedAt",
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
      "name": "userActionStats",
      "docs": [
        "Per-user counters and the next action nonce, PDA `[\"user-stats\", user]`."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "actionsCreated",
            "docs": [
              "Nonce for the next action this user creates."
            ],
            "type": "u64"
          },
          {
            "name": "tipsSent",
            "type": "u64"
          },
          {
            "name": "tipsReceived",
            "type": "u64"
          },
          {
            "name": "vouchesGiven",
            "type": "u32"
          },
          {
            "name": "vouchesReceived",
            "type": "u32"
          },
          {
            "name": "followsGiven",
            "type": "u32"
          },
          {
            "name": "followsReceived",
            "type": "u32"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "viLinkAction",
      "docs": [
        "A shareable action, PDA `[\"action\", creator, nonce]`."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "actionId",
            "docs": [
              "Unique per (creator, nonce); used in share URIs."
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "nonce",
            "type": "u64"
          },
          {
            "name": "actionType",
            "type": "u8"
          },
          {
            "name": "amount",
            "docs": [
              "Tip amount (gross); 0 for vouch / follow."
            ],
            "type": "u64"
          },
          {
            "name": "target",
            "type": "pubkey"
          },
          {
            "name": "metadataHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "feeBps",
            "type": "u16"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "expiresAt",
            "type": "i64"
          },
          {
            "name": "maxExecutions",
            "type": "u32"
          },
          {
            "name": "executionCount",
            "type": "u32"
          },
          {
            "name": "totalVolume",
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "viLinkConfig",
      "docs": [
        "Singleton, PDA `[\"vilink-config\"]`. Read-only on create and execute (audit VL-13)."
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
            "name": "treasury",
            "docs": [
              "VCoin token account receiving tip fees."
            ],
            "type": "pubkey"
          },
          {
            "name": "feeBps",
            "docs": [
              "Fee for new links (each link keeps the fee it was created with, audit VL-16)."
            ],
            "type": "u16"
          },
          {
            "name": "pendingFeeBps",
            "docs": [
              "Timelocked change of fee and treasury (`pending_eta == 0` = none, audit VL-4)."
            ],
            "type": "u16"
          },
          {
            "name": "pendingTreasury",
            "type": "pubkey"
          },
          {
            "name": "pendingEta",
            "type": "i64"
          },
          {
            "name": "enabledActions",
            "docs": [
              "Bit per action type; disabling stops creation and execution (audit VL-2)."
            ],
            "type": "u8"
          },
          {
            "name": "paused",
            "type": "bool"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "viLinkInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "treasury",
            "type": "pubkey"
          },
          {
            "name": "feeBps",
            "type": "u16"
          }
        ]
      }
    }
  ],
  "constants": [
    {
      "name": "actionSeed",
      "type": "bytes",
      "value": "[97, 99, 116, 105, 111, 110]"
    },
    {
      "name": "configSeed",
      "type": "bytes",
      "value": "[118, 105, 108, 105, 110, 107, 45, 99, 111, 110, 102, 105, 103]"
    },
    {
      "name": "receiptSeed",
      "type": "bytes",
      "value": "[101, 120, 101, 99]"
    },
    {
      "name": "userStatsSeed",
      "type": "bytes",
      "value": "[117, 115, 101, 114, 45, 115, 116, 97, 116, 115]"
    }
  ]
};
