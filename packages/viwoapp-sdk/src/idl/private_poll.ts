/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/private_poll.json`.
 */
export type PrivatePoll = {
  "address": "3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe",
  "metadata": {
    "name": "privatePoll",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "VCoin private polls: encrypted ballots weighted by staking-protocol, threshold-decrypted by a Feldman-verified committee; assert_poll_passed lets a Realms proposal depend on the outcome"
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
                  108,
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
      "name": "assertPollPassed",
      "docs": [
        "Fails unless the poll passed and this is its first use by its consumer."
      ],
      "discriminator": [
        203,
        65,
        118,
        119,
        189,
        210,
        208,
        90
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
                  108,
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
          "name": "poll",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  108,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "poll.id",
                "account": "poll"
              }
            ]
          }
        },
        {
          "name": "consumer",
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
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  108,
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
      "name": "castVote",
      "docs": [
        "Casts an encrypted ballot (`R ‖ C` per choice) with its validity proof."
      ],
      "discriminator": [
        20,
        212,
        15,
        189,
        69,
        180,
        69,
        151
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
                  108,
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
          "name": "poll",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  108,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "poll.id",
                "account": "poll"
              }
            ]
          }
        },
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
          },
          "relations": [
            "config"
          ]
        },
        {
          "name": "stake",
          "docs": [
            "The voter's staking deposits."
          ],
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
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "userScore",
          "docs": [
            "`staking_protocol::governance::five_a_score`."
          ]
        },
        {
          "name": "ballotRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  97,
                  108,
                  108,
                  111,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "poll"
              },
              {
                "kind": "account",
                "path": "owner"
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
          "name": "ballot",
          "type": {
            "array": [
              "u8",
              192
            ]
          }
        },
        {
          "name": "proof",
          "type": {
            "array": [
              "u8",
              448
            ]
          }
        }
      ]
    },
    {
      "name": "closeBallot",
      "discriminator": [
        243,
        82,
        194,
        215,
        41,
        255,
        65,
        98
      ],
      "accounts": [
        {
          "name": "ballotRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  97,
                  108,
                  108,
                  111,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "ballot_record.poll",
                "account": "ballotRecord"
              },
              {
                "kind": "account",
                "path": "voter"
              }
            ]
          }
        },
        {
          "name": "poll"
        },
        {
          "name": "voter",
          "writable": true,
          "signer": true,
          "relations": [
            "ballotRecord"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "closePoll",
      "discriminator": [
        139,
        213,
        162,
        65,
        172,
        150,
        123,
        67
      ],
      "accounts": [
        {
          "name": "poll",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  108,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "poll.id",
                "account": "poll"
              }
            ]
          }
        },
        {
          "name": "creator",
          "writable": true,
          "signer": true,
          "relations": [
            "poll"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "createPoll",
      "docs": [
        "Opens a poll. `consumer` is the only signer that may later consume a pass."
      ],
      "discriminator": [
        182,
        171,
        112,
        238,
        6,
        219,
        14,
        110
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
                  108,
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
          },
          "relations": [
            "config"
          ]
        },
        {
          "name": "creatorVoter",
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
                "path": "creator"
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
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "poll",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  108,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "config.poll_count",
                "account": "pollConfig"
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
          "name": "descriptionHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "startTime",
          "type": "i64"
        },
        {
          "name": "endTime",
          "type": "i64"
        },
        {
          "name": "consumer",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "expirePoll",
      "discriminator": [
        62,
        158,
        252,
        199,
        196,
        88,
        41,
        192
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
                  108,
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
          "name": "poll",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  108,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "poll.id",
                "account": "poll"
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "finalizePoll",
      "docs": [
        "Settles the poll once with a verified tally."
      ],
      "discriminator": [
        90,
        57,
        229,
        211,
        20,
        47,
        151,
        93
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
                  108,
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
          "name": "poll",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  108,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "poll.id",
                "account": "poll"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "tally",
          "type": {
            "array": [
              "u64",
              3
            ]
          }
        },
        {
          "name": "lagrange",
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
      "name": "initializeConfig",
      "docs": [
        "Creates the config pointing at the staking registrar. Upgrade authority only."
      ],
      "discriminator": [
        208,
        127,
        21,
        1,
        194,
        190,
        196,
        70
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
                  108,
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
          "name": "program",
          "address": "3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe"
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
          "name": "params",
          "type": {
            "defined": {
              "name": "pollParams"
            }
          }
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
                  108,
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
      "name": "setCommittee",
      "docs": [
        "Registers the decryption committee (authority = the DAO). Feldman-verified."
      ],
      "discriminator": [
        197,
        116,
        137,
        105,
        6,
        92,
        129,
        215
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
                  108,
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
          "name": "threshold",
          "type": "u8"
        },
        {
          "name": "commitments",
          "type": {
            "vec": {
              "array": [
                "u8",
                32
              ]
            }
          }
        },
        {
          "name": "memberPks",
          "type": {
            "vec": {
              "array": [
                "u8",
                32
              ]
            }
          }
        },
        {
          "name": "memberAuthorities",
          "type": {
            "vec": "pubkey"
          }
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
                  108,
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
                  108,
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
      "name": "setPollParams",
      "discriminator": [
        233,
        136,
        26,
        169,
        113,
        36,
        85,
        164
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
                  108,
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
              "name": "pollParams"
            }
          }
        }
      ]
    },
    {
      "name": "submitShare",
      "docs": [
        "Committee member `member_index` (1-based) posts its partial decryption."
      ],
      "discriminator": [
        100,
        223,
        239,
        150,
        211,
        166,
        6,
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
                  112,
                  111,
                  108,
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
          "name": "poll",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  108,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "poll.id",
                "account": "poll"
              }
            ]
          }
        },
        {
          "name": "member",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "memberIndex",
          "type": "u8"
        },
        {
          "name": "shares",
          "type": {
            "array": [
              {
                "array": [
                  "u8",
                  32
                ]
              },
              3
            ]
          }
        },
        {
          "name": "proof",
          "type": {
            "array": [
              "u8",
              64
            ]
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "ballotRecord",
      "discriminator": [
        162,
        234,
        185,
        73,
        61,
        115,
        59,
        103
      ]
    },
    {
      "name": "poll",
      "discriminator": [
        110,
        234,
        167,
        188,
        231,
        136,
        153,
        111
      ]
    },
    {
      "name": "pollConfig",
      "discriminator": [
        6,
        98,
        208,
        250,
        192,
        188,
        11,
        225
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
      "name": "ballotCast",
      "discriminator": [
        194,
        0,
        41,
        178,
        6,
        3,
        169,
        32
      ]
    },
    {
      "name": "committeeSet",
      "discriminator": [
        239,
        180,
        39,
        35,
        179,
        101,
        105,
        50
      ]
    },
    {
      "name": "configInitialized",
      "discriminator": [
        181,
        49,
        200,
        156,
        19,
        167,
        178,
        91
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
      "name": "pollConsumed",
      "discriminator": [
        46,
        166,
        126,
        73,
        158,
        73,
        125,
        40
      ]
    },
    {
      "name": "pollCreated",
      "discriminator": [
        137,
        85,
        250,
        148,
        2,
        9,
        178,
        39
      ]
    },
    {
      "name": "pollFinalized",
      "discriminator": [
        193,
        12,
        192,
        22,
        4,
        249,
        27,
        252
      ]
    },
    {
      "name": "pollParamsUpdated",
      "discriminator": [
        71,
        214,
        72,
        69,
        73,
        194,
        11,
        91
      ]
    },
    {
      "name": "shareSubmitted",
      "discriminator": [
        252,
        39,
        187,
        22,
        37,
        118,
        240,
        156
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "paused",
      "msg": "Private polls are paused"
    },
    {
      "code": 6001,
      "name": "invalidProgramData",
      "msg": "Program data account does not belong to this program"
    },
    {
      "code": 6002,
      "name": "invalidParams",
      "msg": "Poll parameters out of bounds"
    },
    {
      "code": 6003,
      "name": "committeeNotSet",
      "msg": "No decryption committee has been registered"
    },
    {
      "code": 6004,
      "name": "invalidCommittee",
      "msg": "Committee must satisfy 3 <= threshold <= size <= 5 with distinct, non-default members"
    },
    {
      "code": 6005,
      "name": "feldmanMismatch",
      "msg": "A committee member key does not match the Feldman commitments"
    },
    {
      "code": 6006,
      "name": "invalidSchedule",
      "msg": "Start must be within 30 days and the voting period within the configured bounds"
    },
    {
      "code": 6007,
      "name": "creatorWeightTooLow",
      "msg": "Creator's lock-weighted stake is below the minimum to open a poll"
    },
    {
      "code": 6008,
      "name": "invalidConsumer",
      "msg": "Consumer must be set (e.g. the Realms governance PDA)"
    },
    {
      "code": 6009,
      "name": "pollNotOpen",
      "msg": "Poll is not open for voting"
    },
    {
      "code": 6010,
      "name": "noVotingWeight",
      "msg": "No lock-weighted stake at the poll's end time"
    },
    {
      "code": 6011,
      "name": "scoreBelowGate",
      "msg": "5A reputation score is below the voting gate"
    },
    {
      "code": 6012,
      "name": "invalidBallotProof",
      "msg": "Ballot proof rejected"
    },
    {
      "code": 6013,
      "name": "invalidPoint",
      "msg": "Invalid curve point"
    },
    {
      "code": 6014,
      "name": "nonCanonicalScalar",
      "msg": "Non-canonical scalar"
    },
    {
      "code": 6015,
      "name": "votingNotEnded",
      "msg": "Voting has not ended"
    },
    {
      "code": 6016,
      "name": "revealClosed",
      "msg": "Reveal window is closed"
    },
    {
      "code": 6017,
      "name": "revealStillOpen",
      "msg": "Reveal window is still open"
    },
    {
      "code": 6018,
      "name": "notCommitteeMember",
      "msg": "Signer is not this committee member"
    },
    {
      "code": 6019,
      "name": "shareAlreadySubmitted",
      "msg": "This member already submitted a decryption share"
    },
    {
      "code": 6020,
      "name": "invalidDleqProof",
      "msg": "Decryption share proof rejected"
    },
    {
      "code": 6021,
      "name": "notEnoughShares",
      "msg": "Fewer decryption shares than the threshold"
    },
    {
      "code": 6022,
      "name": "invalidLagrange",
      "msg": "One Lagrange coefficient per submitted share is required"
    },
    {
      "code": 6023,
      "name": "tallyMismatch",
      "msg": "Claimed tally does not match the encrypted ballots"
    },
    {
      "code": 6024,
      "name": "pollAlreadyFinal",
      "msg": "Poll already finalized or expired"
    },
    {
      "code": 6025,
      "name": "pollNotFinal",
      "msg": "Poll is not final yet"
    },
    {
      "code": 6026,
      "name": "pollNotPassed",
      "msg": "Poll did not pass"
    },
    {
      "code": 6027,
      "name": "alreadyConsumed",
      "msg": "Poll outcome was already consumed"
    },
    {
      "code": 6028,
      "name": "wrongConsumer",
      "msg": "Signer is not the poll's consumer"
    },
    {
      "code": 6029,
      "name": "closeTooEarly",
      "msg": "A passed, unconsumed poll can only be closed after the grace period"
    },
    {
      "code": 6030,
      "name": "finalizeInstead",
      "msg": "Enough shares were submitted; finalize the poll instead"
    },
    {
      "code": 6031,
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
      "name": "ballotCast",
      "docs": [
        "Carries the full ballot so anyone can recompute the poll's aggregates."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poll",
            "type": "pubkey"
          },
          {
            "name": "voter",
            "type": "pubkey"
          },
          {
            "name": "weight",
            "type": "u64"
          },
          {
            "name": "ballot",
            "type": {
              "array": [
                "u8",
                192
              ]
            }
          },
          {
            "name": "totalWeight",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "ballotRecord",
      "docs": [
        "One per (poll, voter): prevents double voting. PDA `[\"ballot\", poll, voter]`."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poll",
            "type": "pubkey"
          },
          {
            "name": "voter",
            "type": "pubkey"
          },
          {
            "name": "weight",
            "type": "u64"
          },
          {
            "name": "ballotHash",
            "docs": [
              "SHA-256 of the 192-byte ballot (the ballot itself is in the transaction and event)."
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "castAt",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "committee",
      "docs": [
        "Decryption committee registered by the DAO (audit ZK-03: not chosen per poll). Polls",
        "snapshot it at creation, so a later change never affects a running poll."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "epoch",
            "docs": [
              "Incremented on every change; 0 = never set."
            ],
            "type": "u32"
          },
          {
            "name": "threshold",
            "type": "u8"
          },
          {
            "name": "size",
            "type": "u8"
          },
          {
            "name": "electionPk",
            "docs": [
              "`commitments[0]`."
            ],
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "commitments",
            "docs": [
              "Feldman commitments `A_0..A_{threshold-1}` (unused slots zero)."
            ],
            "type": {
              "array": [
                {
                  "array": [
                    "u8",
                    32
                  ]
                },
                5
              ]
            }
          },
          {
            "name": "memberPks",
            "docs": [
              "`member_pks[i] = Σ_k (i+1)^k·A_k`, verified at registration."
            ],
            "type": {
              "array": [
                {
                  "array": [
                    "u8",
                    32
                  ]
                },
                5
              ]
            }
          },
          {
            "name": "memberAuthorities",
            "docs": [
              "Wallet allowed to submit member `i + 1`'s decryption share."
            ],
            "type": {
              "array": [
                "pubkey",
                5
              ]
            }
          }
        ]
      }
    },
    {
      "name": "committeeSet",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "epoch",
            "type": "u32"
          },
          {
            "name": "threshold",
            "type": "u8"
          },
          {
            "name": "size",
            "type": "u8"
          },
          {
            "name": "electionPk",
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
      "name": "configInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "registrar",
            "type": "pubkey"
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
      "name": "poll",
      "docs": [
        "One poll, PDA `[\"poll\", id]`."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "consumer",
            "docs": [
              "The only signer that may consume a pass (e.g. the Realms governance PDA)."
            ],
            "type": "pubkey"
          },
          {
            "name": "descriptionHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "revealDeadline",
            "type": "i64"
          },
          {
            "name": "weightParams",
            "docs": [
              "Snapshot so every voter's weight uses the same parameters and timestamp."
            ],
            "type": {
              "defined": {
                "name": "weightParams"
              }
            }
          },
          {
            "name": "quorumWeight",
            "type": "u64"
          },
          {
            "name": "passBps",
            "type": "u16"
          },
          {
            "name": "committee",
            "type": {
              "defined": {
                "name": "committee"
              }
            }
          },
          {
            "name": "aggR",
            "docs": [
              "Running sums of every ballot's `R` and `C` per choice."
            ],
            "type": {
              "array": [
                {
                  "array": [
                    "u8",
                    32
                  ]
                },
                3
              ]
            }
          },
          {
            "name": "aggC",
            "type": {
              "array": [
                {
                  "array": [
                    "u8",
                    32
                  ]
                },
                3
              ]
            }
          },
          {
            "name": "totalWeight",
            "type": "u64"
          },
          {
            "name": "voterCount",
            "type": "u32"
          },
          {
            "name": "sharesBitmap",
            "docs": [
              "Bit `i` set = member `i + 1` submitted a verified share."
            ],
            "type": "u8"
          },
          {
            "name": "partials",
            "type": {
              "array": [
                {
                  "array": [
                    {
                      "array": [
                        "u8",
                        32
                      ]
                    },
                    3
                  ]
                },
                5
              ]
            }
          },
          {
            "name": "outcome",
            "type": {
              "defined": {
                "name": "pollOutcome"
              }
            }
          },
          {
            "name": "tally",
            "type": {
              "array": [
                "u64",
                3
              ]
            }
          },
          {
            "name": "finalizedAt",
            "type": "i64"
          },
          {
            "name": "consumed",
            "type": "bool"
          },
          {
            "name": "pauseAccumAtCreation",
            "docs": [
              "`PollConfig::pause_accum` when the poll was created."
            ],
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "pollConfig",
      "docs": [
        "Singleton, PDA `[\"poll-config\"]`."
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
            "name": "registrar",
            "docs": [
              "staking-protocol registrar whose voters vote here."
            ],
            "type": "pubkey"
          },
          {
            "name": "committee",
            "type": {
              "defined": {
                "name": "committee"
              }
            }
          },
          {
            "name": "params",
            "type": {
              "defined": {
                "name": "pollParams"
              }
            }
          },
          {
            "name": "pollCount",
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
              "Total seconds spent paused, so reveal deadlines can be extended by downtime."
            ],
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "pollConsumed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poll",
            "type": "pubkey"
          },
          {
            "name": "consumer",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "pollCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poll",
            "type": "pubkey"
          },
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "consumer",
            "type": "pubkey"
          },
          {
            "name": "descriptionHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "revealDeadline",
            "type": "i64"
          },
          {
            "name": "committeeEpoch",
            "type": "u32"
          },
          {
            "name": "quorumWeight",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "pollFinalized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poll",
            "type": "pubkey"
          },
          {
            "name": "outcome",
            "type": {
              "defined": {
                "name": "pollOutcome"
              }
            }
          },
          {
            "name": "tally",
            "type": {
              "array": [
                "u64",
                3
              ]
            }
          },
          {
            "name": "totalWeight",
            "type": "u64"
          },
          {
            "name": "quorumWeight",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "pollOutcome",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "open"
          },
          {
            "name": "passed"
          },
          {
            "name": "rejected"
          },
          {
            "name": "expired"
          }
        ]
      }
    },
    {
      "name": "pollParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "quorumBps",
            "docs": [
              "Minimum total weight cast, as basis points of the maximum possible weight."
            ],
            "type": "u16"
          },
          {
            "name": "passBps",
            "docs": [
              "For must exceed this share (bps) of For + Against."
            ],
            "type": "u16"
          },
          {
            "name": "minVotingSecs",
            "type": "i64"
          },
          {
            "name": "maxVotingSecs",
            "type": "i64"
          },
          {
            "name": "revealWindowSecs",
            "type": "i64"
          },
          {
            "name": "minCreatorWeight",
            "docs": [
              "Poll weight units the creator must hold."
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "pollParamsUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "params",
            "type": {
              "defined": {
                "name": "pollParams"
              }
            }
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
      "name": "shareSubmitted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poll",
            "type": "pubkey"
          },
          {
            "name": "memberIndex",
            "type": "u8"
          },
          {
            "name": "shares",
            "type": {
              "array": [
                {
                  "array": [
                    "u8",
                    32
                  ]
                },
                3
              ]
            }
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
    }
  ],
  "constants": [
    {
      "name": "ballotSeed",
      "type": "bytes",
      "value": "[98, 97, 108, 108, 111, 116]"
    },
    {
      "name": "configSeed",
      "type": "bytes",
      "value": "[112, 111, 108, 108, 45, 99, 111, 110, 102, 105, 103]"
    },
    {
      "name": "pollSeed",
      "type": "bytes",
      "value": "[112, 111, 108, 108]"
    }
  ]
};
