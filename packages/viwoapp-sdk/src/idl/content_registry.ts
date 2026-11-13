/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/content_registry.json`.
 */
export type ContentRegistry = {
  "address": "MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV",
  "metadata": {
    "name": "contentRegistry",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "ViWoApp Content Registry - On-chain content tracking with energy system"
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
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  121,
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
      "name": "applyOracle",
      "discriminator": [
        192,
        35,
        58,
        45,
        187,
        88,
        53,
        235
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
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  121,
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
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  121,
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
      "name": "cancelOracle",
      "discriminator": [
        247,
        248,
        0,
        71,
        2,
        229,
        113,
        227
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
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  121,
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
      "name": "claimRefund",
      "discriminator": [
        15,
        16,
        30,
        161,
        255,
        228,
        97,
        60
      ],
      "accounts": [
        {
          "name": "config",
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
                  121,
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
          "name": "author",
          "signer": true,
          "relations": [
            "content"
          ]
        },
        {
          "name": "userEnergy",
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
                  101,
                  110,
                  101,
                  114,
                  103,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "author"
              }
            ]
          }
        },
        {
          "name": "content",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  116,
                  101,
                  110,
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
                "path": "author"
              },
              {
                "kind": "account",
                "path": "content.tracking_id",
                "account": "contentRecord"
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "createContent",
      "discriminator": [
        196,
        78,
        200,
        14,
        158,
        190,
        68,
        223
      ],
      "accounts": [
        {
          "name": "config",
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
                  121,
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
          "name": "author"
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
                  5,
                  51,
                  183,
                  40,
                  188,
                  15,
                  103,
                  254,
                  234,
                  163,
                  142,
                  219,
                  10,
                  183,
                  110,
                  184,
                  87,
                  117,
                  223,
                  252,
                  30,
                  163,
                  175,
                  246,
                  128,
                  109,
                  37,
                  34,
                  133,
                  207,
                  96,
                  18
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              },
              {
                "kind": "account",
                "path": "author"
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
          "name": "userEnergy",
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
                  101,
                  110,
                  101,
                  114,
                  103,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "author"
              }
            ]
          }
        },
        {
          "name": "stake",
          "docs": [
            "The author's staking deposits; refreshes the tier on every post. Omitted = tier 0."
          ],
          "optional": true,
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
                "path": "author"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                77,
                175,
                130,
                234,
                236,
                183,
                242,
                140,
                88,
                220,
                16,
                6,
                145,
                239,
                100,
                212,
                48,
                93,
                96,
                72,
                241,
                157,
                52,
                146,
                186,
                137,
                103,
                17,
                16,
                165,
                22,
                226
              ]
            }
          }
        },
        {
          "name": "content",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  116,
                  101,
                  110,
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
                "path": "author"
              },
              {
                "kind": "arg",
                "path": "trackingId"
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
          "name": "trackingId",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "contentHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "uri",
          "type": "string"
        },
        {
          "name": "contentType",
          "type": "u8"
        }
      ]
    },
    {
      "name": "deleteContent",
      "discriminator": [
        5,
        233,
        219,
        88,
        173,
        220,
        96,
        213
      ],
      "accounts": [
        {
          "name": "content",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  116,
                  101,
                  110,
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
                "path": "author"
              },
              {
                "kind": "account",
                "path": "content.tracking_id",
                "account": "contentRecord"
              }
            ]
          }
        },
        {
          "name": "author",
          "writable": true,
          "signer": true,
          "relations": [
            "content"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "editContent",
      "discriminator": [
        159,
        243,
        0,
        246,
        151,
        76,
        241,
        132
      ],
      "accounts": [
        {
          "name": "config",
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
                  121,
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
          "name": "author",
          "relations": [
            "content"
          ]
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
                  5,
                  51,
                  183,
                  40,
                  188,
                  15,
                  103,
                  254,
                  234,
                  163,
                  142,
                  219,
                  10,
                  183,
                  110,
                  184,
                  87,
                  117,
                  223,
                  252,
                  30,
                  163,
                  175,
                  246,
                  128,
                  109,
                  37,
                  34,
                  133,
                  207,
                  96,
                  18
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              },
              {
                "kind": "account",
                "path": "author"
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
          "name": "userEnergy",
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
                  101,
                  110,
                  101,
                  114,
                  103,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "author"
              }
            ]
          }
        },
        {
          "name": "content",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  116,
                  101,
                  110,
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
                "path": "author"
              },
              {
                "kind": "account",
                "path": "content.tracking_id",
                "account": "contentRecord"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "contentHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "uri",
          "type": "string"
        }
      ]
    },
    {
      "name": "initUserEnergy",
      "discriminator": [
        9,
        74,
        82,
        84,
        139,
        134,
        138,
        253
      ],
      "accounts": [
        {
          "name": "config",
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
                  121,
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
          "name": "user"
        },
        {
          "name": "userEnergy",
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
                  101,
                  110,
                  101,
                  114,
                  103,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "stake",
          "docs": [
            "The user's staking deposits; omitted = tier 0."
          ],
          "optional": true,
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
                "path": "user"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                77,
                175,
                130,
                234,
                236,
                183,
                242,
                140,
                88,
                220,
                16,
                6,
                145,
                239,
                100,
                212,
                48,
                93,
                96,
                72,
                241,
                157,
                52,
                146,
                186,
                137,
                103,
                17,
                16,
                165,
                22,
                226
              ]
            }
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
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  121,
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
          "name": "program",
          "address": "MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV"
        },
        {
          "name": "programData"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "engagementOracle",
          "type": "pubkey"
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
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  121,
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
      "name": "proposeOracle",
      "discriminator": [
        95,
        188,
        180,
        73,
        7,
        248,
        232,
        78
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
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  121,
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
                  114,
                  101,
                  103,
                  105,
                  115,
                  116,
                  114,
                  121,
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
      "name": "updateEngagement",
      "discriminator": [
        199,
        7,
        61,
        55,
        85,
        5,
        124,
        167
      ],
      "accounts": [
        {
          "name": "config",
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
                  121,
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
          "name": "oracle",
          "signer": true
        },
        {
          "name": "content",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  110,
                  116,
                  101,
                  110,
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
                "path": "content.author",
                "account": "contentRecord"
              },
              {
                "kind": "account",
                "path": "content.tracking_id",
                "account": "contentRecord"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "engagementCount",
          "type": "u32"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "contentRecord",
      "discriminator": [
        22,
        30,
        119,
        161,
        251,
        124,
        237,
        60
      ]
    },
    {
      "name": "registryConfig",
      "discriminator": [
        23,
        118,
        10,
        246,
        173,
        231,
        243,
        156
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
      "name": "userEnergy",
      "discriminator": [
        113,
        252,
        23,
        136,
        198,
        208,
        195,
        145
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
      "name": "contentCreated",
      "discriminator": [
        157,
        185,
        30,
        198,
        225,
        95,
        224,
        120
      ]
    },
    {
      "name": "contentDeleted",
      "discriminator": [
        240,
        112,
        24,
        64,
        114,
        150,
        36,
        148
      ]
    },
    {
      "name": "contentEdited",
      "discriminator": [
        98,
        8,
        87,
        128,
        247,
        68,
        134,
        22
      ]
    },
    {
      "name": "energyInitialized",
      "discriminator": [
        229,
        244,
        220,
        25,
        9,
        95,
        235,
        41
      ]
    },
    {
      "name": "energySpent",
      "discriminator": [
        13,
        135,
        89,
        202,
        115,
        38,
        86,
        179
      ]
    },
    {
      "name": "engagementUpdated",
      "discriminator": [
        92,
        20,
        236,
        211,
        222,
        109,
        222,
        172
      ]
    },
    {
      "name": "oracleChangeCancelled",
      "discriminator": [
        200,
        44,
        104,
        43,
        174,
        109,
        128,
        213
      ]
    },
    {
      "name": "oracleChangeProposed",
      "discriminator": [
        157,
        226,
        113,
        87,
        201,
        38,
        180,
        183
      ]
    },
    {
      "name": "oracleChanged",
      "discriminator": [
        151,
        212,
        213,
        104,
        49,
        131,
        55,
        23
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
      "name": "refundClaimed",
      "discriminator": [
        136,
        64,
        242,
        99,
        4,
        244,
        208,
        130
      ]
    },
    {
      "name": "registryInitialized",
      "discriminator": [
        144,
        138,
        62,
        105,
        58,
        38,
        100,
        177
      ]
    },
    {
      "name": "tierChanged",
      "discriminator": [
        126,
        9,
        150,
        127,
        199,
        123,
        136,
        1
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "paused",
      "msg": "Content registry is paused"
    },
    {
      "code": 6001,
      "name": "invalidProgramData",
      "msg": "Program data account does not belong to this program"
    },
    {
      "code": 6002,
      "name": "invalidKey",
      "msg": "Invalid key"
    },
    {
      "code": 6003,
      "name": "unauthorized",
      "msg": "Signer is neither the author nor a valid content session for the author"
    },
    {
      "code": 6004,
      "name": "uriTooLong",
      "msg": "URI longer than 128 bytes"
    },
    {
      "code": 6005,
      "name": "invalidContentType",
      "msg": "Unknown content type"
    },
    {
      "code": 6006,
      "name": "insufficientEnergy",
      "msg": "Not enough energy"
    },
    {
      "code": 6007,
      "name": "dailyCapExceeded",
      "msg": "Daily post cap reached"
    },
    {
      "code": 6008,
      "name": "notEngagementOracle",
      "msg": "Signer is not the engagement oracle"
    },
    {
      "code": 6009,
      "name": "refundTooEarly",
      "msg": "Refunds open 24h after posting"
    },
    {
      "code": 6010,
      "name": "refundWindowClosed",
      "msg": "The 7-day refund window has closed"
    },
    {
      "code": 6011,
      "name": "refundNotReady",
      "msg": "Engagement does not qualify for a refund yet"
    },
    {
      "code": 6012,
      "name": "refundAlreadyClaimed",
      "msg": "Refund already claimed"
    },
    {
      "code": 6013,
      "name": "dailyRefundCapExceeded",
      "msg": "Daily refund cap reached"
    },
    {
      "code": 6014,
      "name": "noPendingChange",
      "msg": "No oracle change pending"
    },
    {
      "code": 6015,
      "name": "timelockActive",
      "msg": "Oracle change timelock still active"
    },
    {
      "code": 6016,
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
      "name": "contentCreated",
      "docs": [
        "Every version's hash is in the event log (the record keeps only the latest)."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "author",
            "type": "pubkey"
          },
          {
            "name": "trackingId",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "contentHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "contentType",
            "type": "u8"
          },
          {
            "name": "signer",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "contentDeleted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "author",
            "type": "pubkey"
          },
          {
            "name": "trackingId",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "contentHash",
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
      "name": "contentEdited",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "author",
            "type": "pubkey"
          },
          {
            "name": "trackingId",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "previousHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "contentHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "version",
            "type": "u16"
          },
          {
            "name": "signer",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "contentRecord",
      "docs": [
        "PDA `[\"content-record\", author, tracking_id]`: the author is part of the address, so",
        "nobody can squat another author's id (audit CR-02)."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "author",
            "type": "pubkey"
          },
          {
            "name": "trackingId",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "contentHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "uri",
            "type": {
              "array": [
                "u8",
                128
              ]
            }
          },
          {
            "name": "uriLen",
            "type": "u8"
          },
          {
            "name": "contentType",
            "type": "u8"
          },
          {
            "name": "state",
            "type": {
              "defined": {
                "name": "contentState"
              }
            }
          },
          {
            "name": "version",
            "type": "u16"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "updatedAt",
            "type": "i64"
          },
          {
            "name": "energySpent",
            "type": "u16"
          },
          {
            "name": "engagementCount",
            "type": "u32"
          },
          {
            "name": "refundClaimed",
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
      "name": "contentState",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "active"
          },
          {
            "name": "edited"
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
          }
        ]
      }
    },
    {
      "name": "energyInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "tier",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "energySpent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u16"
          },
          {
            "name": "remaining",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "engagementUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "author",
            "type": "pubkey"
          },
          {
            "name": "trackingId",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "engagementCount",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "oracleChangeCancelled",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "oracleChangeProposed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "oracle",
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
      "name": "oracleChanged",
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
      "name": "refundClaimed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "author",
            "type": "pubkey"
          },
          {
            "name": "trackingId",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "percent",
            "type": "u16"
          },
          {
            "name": "amount",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "registryConfig",
      "docs": [
        "Singleton, PDA `[\"registry-config\"]`. Read-only on every user path (audit CR-05)."
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
            "name": "engagementOracle",
            "docs": [
              "Backend key that reports engagement; separate from the admin (audit CR-07)."
            ],
            "type": "pubkey"
          },
          {
            "name": "pendingOracle",
            "docs": [
              "Timelocked oracle change (`pending_oracle_eta == 0` = none)."
            ],
            "type": "pubkey"
          },
          {
            "name": "pendingOracleEta",
            "type": "i64"
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
      "name": "registryInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "engagementOracle",
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
      "name": "tierChanged",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "oldTier",
            "type": "u8"
          },
          {
            "name": "newTier",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "userEnergy",
      "docs": [
        "Per-user energy and daily post counter, PDA `[\"user-energy\", user]`."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "tier",
            "type": "u8"
          },
          {
            "name": "currentEnergy",
            "type": "u16"
          },
          {
            "name": "maxEnergy",
            "type": "u16"
          },
          {
            "name": "regenPerHour",
            "type": "u16"
          },
          {
            "name": "lastRegenTime",
            "docs": [
              "Regeneration is accounted up to this time; partial hours carry over (audit CR-03)."
            ],
            "type": "i64"
          },
          {
            "name": "dayStart",
            "type": "i64"
          },
          {
            "name": "postsToday",
            "type": "u16"
          },
          {
            "name": "refundedToday",
            "type": "u16"
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
    }
  ],
  "constants": [
    {
      "name": "contentRecordSeed",
      "type": "bytes",
      "value": "[99, 111, 110, 116, 101, 110, 116, 45, 114, 101, 99, 111, 114, 100]"
    },
    {
      "name": "registryConfigSeed",
      "type": "bytes",
      "value": "[114, 101, 103, 105, 115, 116, 114, 121, 45, 99, 111, 110, 102, 105, 103]"
    },
    {
      "name": "userEnergySeed",
      "type": "bytes",
      "value": "[117, 115, 101, 114, 45, 101, 110, 101, 114, 103, 121]"
    }
  ]
};
