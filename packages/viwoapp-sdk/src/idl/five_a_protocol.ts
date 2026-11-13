/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/five_a_protocol.json`.
 */
export type FiveAProtocol = {
  "address": "783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8",
  "metadata": {
    "name": "fiveAProtocol",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "5A Reputation Protocol - Anti-bot reputation scoring with oracle model"
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
                  102,
                  105,
                  118,
                  101,
                  45,
                  97,
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
                  102,
                  105,
                  118,
                  101,
                  45,
                  97,
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
      "name": "closeExpiredRound",
      "discriminator": [
        12,
        97,
        81,
        56,
        237,
        112,
        122,
        127
      ],
      "accounts": [
        {
          "name": "pending",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  101,
                  110,
                  100,
                  105,
                  110,
                  103,
                  45,
                  115,
                  99,
                  111,
                  114,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "pending.user",
                "account": "pendingScoreUpdate"
              }
            ]
          }
        },
        {
          "name": "rentPayer",
          "writable": true,
          "relations": [
            "pending"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "declineVouch",
      "docs": [
        "The vouchee declines a vouch made for them."
      ],
      "discriminator": [
        179,
        206,
        212,
        152,
        129,
        254,
        67,
        211
      ],
      "accounts": [
        {
          "name": "close",
          "accounts": [
            {
              "name": "config",
              "pda": {
                "seeds": [
                  {
                    "kind": "const",
                    "value": [
                      102,
                      105,
                      118,
                      101,
                      45,
                      97,
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
              "name": "vouchRecord",
              "writable": true,
              "pda": {
                "seeds": [
                  {
                    "kind": "const",
                    "value": [
                      118,
                      111,
                      117,
                      99,
                      104,
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
                    "path": "vouch_record.voucher",
                    "account": "vouchRecord"
                  },
                  {
                    "kind": "account",
                    "path": "vouch_record.vouchee",
                    "account": "vouchRecord"
                  }
                ]
              }
            },
            {
              "name": "voucher",
              "writable": true,
              "relations": [
                "vouchRecord"
              ]
            },
            {
              "name": "voucherStats",
              "writable": true,
              "pda": {
                "seeds": [
                  {
                    "kind": "const",
                    "value": [
                      118,
                      111,
                      117,
                      99,
                      104,
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
                    "path": "vouch_record.voucher",
                    "account": "vouchRecord"
                  }
                ]
              }
            },
            {
              "name": "voucheeStatus",
              "writable": true,
              "pda": {
                "seeds": [
                  {
                    "kind": "const",
                    "value": [
                      118,
                      111,
                      117,
                      99,
                      104,
                      45,
                      115,
                      116,
                      97,
                      116,
                      117,
                      115
                    ]
                  },
                  {
                    "kind": "account",
                    "path": "vouch_record.vouchee",
                    "account": "vouchRecord"
                  }
                ]
              }
            }
          ]
        },
        {
          "name": "vouchee",
          "signer": true
        }
      ],
      "args": []
    },
    {
      "name": "evaluateVouch",
      "discriminator": [
        42,
        106,
        189,
        248,
        143,
        243,
        79,
        103
      ],
      "accounts": [
        {
          "name": "close",
          "accounts": [
            {
              "name": "config",
              "pda": {
                "seeds": [
                  {
                    "kind": "const",
                    "value": [
                      102,
                      105,
                      118,
                      101,
                      45,
                      97,
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
              "name": "vouchRecord",
              "writable": true,
              "pda": {
                "seeds": [
                  {
                    "kind": "const",
                    "value": [
                      118,
                      111,
                      117,
                      99,
                      104,
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
                    "path": "vouch_record.voucher",
                    "account": "vouchRecord"
                  },
                  {
                    "kind": "account",
                    "path": "vouch_record.vouchee",
                    "account": "vouchRecord"
                  }
                ]
              }
            },
            {
              "name": "voucher",
              "writable": true,
              "relations": [
                "vouchRecord"
              ]
            },
            {
              "name": "voucherStats",
              "writable": true,
              "pda": {
                "seeds": [
                  {
                    "kind": "const",
                    "value": [
                      118,
                      111,
                      117,
                      99,
                      104,
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
                    "path": "vouch_record.voucher",
                    "account": "vouchRecord"
                  }
                ]
              }
            },
            {
              "name": "voucheeStatus",
              "writable": true,
              "pda": {
                "seeds": [
                  {
                    "kind": "const",
                    "value": [
                      118,
                      111,
                      117,
                      99,
                      104,
                      45,
                      115,
                      116,
                      97,
                      116,
                      117,
                      115
                    ]
                  },
                  {
                    "kind": "account",
                    "path": "vouch_record.vouchee",
                    "account": "vouchRecord"
                  }
                ]
              }
            }
          ]
        },
        {
          "name": "oracle",
          "signer": true
        },
        {
          "name": "voucheeScore"
        }
      ],
      "args": []
    },
    {
      "name": "expireVouch",
      "discriminator": [
        254,
        165,
        50,
        29,
        118,
        185,
        71,
        69
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  105,
                  118,
                  101,
                  45,
                  97,
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
          "name": "vouchRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  117,
                  99,
                  104,
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
                "path": "vouch_record.voucher",
                "account": "vouchRecord"
              },
              {
                "kind": "account",
                "path": "vouch_record.vouchee",
                "account": "vouchRecord"
              }
            ]
          }
        },
        {
          "name": "voucher",
          "writable": true,
          "relations": [
            "vouchRecord"
          ]
        },
        {
          "name": "voucherStats",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  117,
                  99,
                  104,
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
                "path": "vouch_record.voucher",
                "account": "vouchRecord"
              }
            ]
          }
        },
        {
          "name": "voucheeStatus",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  117,
                  99,
                  104,
                  45,
                  115,
                  116,
                  97,
                  116,
                  117,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "vouch_record.vouchee",
                "account": "vouchRecord"
              }
            ]
          }
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
                  102,
                  105,
                  118,
                  101,
                  45,
                  97,
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
          "address": "783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8"
        },
        {
          "name": "programData"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
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
                  102,
                  105,
                  118,
                  101,
                  45,
                  97,
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
                  102,
                  105,
                  118,
                  101,
                  45,
                  97,
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
                  102,
                  105,
                  118,
                  101,
                  45,
                  97,
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
                  102,
                  105,
                  118,
                  101,
                  45,
                  97,
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
                  102,
                  105,
                  118,
                  101,
                  45,
                  97,
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
      "name": "setRequiredConsensus",
      "discriminator": [
        89,
        218,
        141,
        135,
        150,
        101,
        197,
        124
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
                  102,
                  105,
                  118,
                  101,
                  45,
                  97,
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
          "name": "requiredConsensus",
          "type": "u8"
        }
      ]
    },
    {
      "name": "submitScore",
      "docs": [
        "An active oracle submits (or changes) its values for `user`."
      ],
      "discriminator": [
        212,
        128,
        45,
        22,
        112,
        82,
        85,
        235
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  102,
                  105,
                  118,
                  101,
                  45,
                  97,
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
          "writable": true,
          "signer": true
        },
        {
          "name": "user",
          "docs": [
            "A wallet (system-owned, possibly unfunded)."
          ]
        },
        {
          "name": "pending",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  101,
                  110,
                  100,
                  105,
                  110,
                  103,
                  45,
                  115,
                  99,
                  111,
                  114,
                  101
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
          "name": "rentPayer",
          "docs": [
            "closes. Checked against `pending.rent_payer` in the handler."
          ],
          "writable": true
        },
        {
          "name": "userScore",
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
                  99,
                  111,
                  114,
                  101
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
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "values",
          "type": {
            "defined": {
              "name": "scoreValues"
            }
          }
        }
      ]
    },
    {
      "name": "vouchForUser",
      "discriminator": [
        39,
        218,
        203,
        71,
        92,
        75,
        171,
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
                  102,
                  105,
                  118,
                  101,
                  45,
                  97,
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
          "name": "voucher",
          "writable": true,
          "signer": true
        },
        {
          "name": "voucherScore",
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
                  99,
                  111,
                  114,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "voucher"
              }
            ]
          }
        },
        {
          "name": "vouchee"
        },
        {
          "name": "voucheeScore"
        },
        {
          "name": "vouchRecord",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  117,
                  99,
                  104,
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
                "path": "voucher"
              },
              {
                "kind": "account",
                "path": "vouchee"
              }
            ]
          }
        },
        {
          "name": "reverseVouch",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  117,
                  99,
                  104,
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
                "path": "vouchee"
              },
              {
                "kind": "account",
                "path": "voucher"
              }
            ]
          }
        },
        {
          "name": "voucherStats",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  117,
                  99,
                  104,
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
                "path": "voucher"
              }
            ]
          }
        },
        {
          "name": "voucheeStatus",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  111,
                  117,
                  99,
                  104,
                  45,
                  115,
                  116,
                  97,
                  116,
                  117,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "vouchee"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "fiveAConfig",
      "discriminator": [
        135,
        235,
        169,
        40,
        209,
        8,
        34,
        71
      ]
    },
    {
      "name": "pendingScoreUpdate",
      "discriminator": [
        62,
        70,
        57,
        127,
        11,
        213,
        228,
        96
      ]
    },
    {
      "name": "userScore",
      "discriminator": [
        212,
        150,
        123,
        224,
        34,
        227,
        84,
        39
      ]
    },
    {
      "name": "userVouchStatus",
      "discriminator": [
        25,
        77,
        205,
        25,
        39,
        91,
        207,
        91
      ]
    },
    {
      "name": "vouchRecord",
      "discriminator": [
        17,
        193,
        183,
        51,
        0,
        197,
        20,
        55
      ]
    },
    {
      "name": "voucherStats",
      "discriminator": [
        42,
        238,
        46,
        244,
        38,
        148,
        42,
        84
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
      "name": "consensusSet",
      "discriminator": [
        190,
        32,
        83,
        63,
        178,
        144,
        171,
        48
      ]
    },
    {
      "name": "fiveAInitialized",
      "discriminator": [
        155,
        33,
        2,
        118,
        218,
        107,
        83,
        76
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
      "name": "scoreApplied",
      "discriminator": [
        44,
        150,
        26,
        216,
        69,
        115,
        75,
        92
      ]
    },
    {
      "name": "scoreRoundExpired",
      "discriminator": [
        243,
        227,
        129,
        160,
        213,
        119,
        84,
        192
      ]
    },
    {
      "name": "scoreSubmitted",
      "discriminator": [
        15,
        74,
        143,
        188,
        62,
        88,
        81,
        104
      ]
    },
    {
      "name": "vouchClosed",
      "discriminator": [
        140,
        55,
        117,
        124,
        169,
        37,
        17,
        111
      ]
    },
    {
      "name": "vouchCreated",
      "discriminator": [
        127,
        98,
        245,
        5,
        1,
        172,
        112,
        42
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "paused",
      "msg": "5A is paused"
    },
    {
      "code": 6001,
      "name": "invalidProgramData",
      "msg": "Program data account does not belong to this program"
    },
    {
      "code": 6002,
      "name": "invalidOracle",
      "msg": "Invalid oracle key"
    },
    {
      "code": 6003,
      "name": "oracleExists",
      "msg": "Oracle already registered"
    },
    {
      "code": 6004,
      "name": "tooManyOracles",
      "msg": "All oracle slots are used"
    },
    {
      "code": 6005,
      "name": "oracleNotFound",
      "msg": "Oracle not registered"
    },
    {
      "code": 6006,
      "name": "invalidConsensus",
      "msg": "Consensus must be between 2 and min(registered oracles, 5)"
    },
    {
      "code": 6007,
      "name": "notActiveOracle",
      "msg": "Signer is not an active oracle"
    },
    {
      "code": 6008,
      "name": "invalidScore",
      "msg": "Each score must be at most 10000"
    },
    {
      "code": 6009,
      "name": "scoreUpdateTooFrequent",
      "msg": "This user's score was updated less than an hour ago"
    },
    {
      "code": 6010,
      "name": "roundNotExpired",
      "msg": "The update round has not expired"
    },
    {
      "code": 6011,
      "name": "wrongRentPayer",
      "msg": "Rent payer does not match the round"
    },
    {
      "code": 6012,
      "name": "invalidScoreAccount",
      "msg": "Score account is not the user's PDA"
    },
    {
      "code": 6013,
      "name": "voucherScoreTooLow",
      "msg": "Voucher's 5A score is too low to vouch"
    },
    {
      "code": 6014,
      "name": "voucheeAlreadyEstablished",
      "msg": "Vouchee is already established; vouching is for new or low-score users"
    },
    {
      "code": 6015,
      "name": "cannotVouchSelf",
      "msg": "Cannot vouch for yourself"
    },
    {
      "code": 6016,
      "name": "mutualVouch",
      "msg": "Mutual vouching is not allowed"
    },
    {
      "code": 6017,
      "name": "maxVouchesReached",
      "msg": "Voucher has no free vouch slots"
    },
    {
      "code": 6018,
      "name": "voucheeFullyVouched",
      "msg": "Vouchee already has the maximum number of vouches"
    },
    {
      "code": 6019,
      "name": "evaluationTooEarly",
      "msg": "The evaluation period has not passed"
    },
    {
      "code": 6020,
      "name": "vouchExpired",
      "msg": "The vouch expired; use expire_vouch"
    },
    {
      "code": 6021,
      "name": "vouchNotExpired",
      "msg": "The vouch has not expired"
    },
    {
      "code": 6022,
      "name": "notVouchee",
      "msg": "Signer is not the vouchee of this vouch"
    },
    {
      "code": 6023,
      "name": "declineWindowClosed",
      "msg": "A vouch can only be declined before its evaluation period starts"
    },
    {
      "code": 6024,
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
      "name": "consensusSet",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "requiredConsensus",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "fiveAConfig",
      "docs": [
        "Singleton, PDA `[\"five-a-config\"]`. Read-only on the score path, so submissions for",
        "different users never contend for it (audit 5A-L1)."
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
            "name": "requiredConsensus",
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
      "name": "fiveAInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "requiredConsensus",
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
      "name": "oracleSubmission",
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
            "name": "values",
            "type": {
              "defined": {
                "name": "scoreValues"
              }
            }
          },
          {
            "name": "submittedAt",
            "type": "i64"
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
      "name": "pendingScoreUpdate",
      "docs": [
        "One update round for a user, PDA `[\"pending-score\", user]`. Each oracle has one",
        "changeable submission; competing value sets are tallied independently, so a dishonest",
        "first submission cannot block honest ones (audit 5A-04). Each submission lives for",
        "`SCORE_UPDATE_EXPIRY` on its own clock. Closed (rent to the opener) when a value set",
        "reaches consensus or once every submission has lapsed."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "submissions",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "oracleSubmission"
                  }
                },
                7
              ]
            }
          },
          {
            "name": "openedAt",
            "type": "i64"
          },
          {
            "name": "expiresAt",
            "docs": [
              "When the newest submission lapses; the round can be closed after it."
            ],
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
      "name": "scoreApplied",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "values",
            "type": {
              "defined": {
                "name": "scoreValues"
              }
            }
          },
          {
            "name": "compositeScore",
            "type": "u16"
          },
          {
            "name": "updateCount",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "scoreRoundExpired",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "scoreSubmitted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "oracle",
            "type": "pubkey"
          },
          {
            "name": "values",
            "type": {
              "defined": {
                "name": "scoreValues"
              }
            }
          },
          {
            "name": "support",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "scoreValues",
      "docs": [
        "The five dimensions, each `0..=MAX_SCORE`."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authenticity",
            "type": "u16"
          },
          {
            "name": "accuracy",
            "type": "u16"
          },
          {
            "name": "agility",
            "type": "u16"
          },
          {
            "name": "activity",
            "type": "u16"
          },
          {
            "name": "approved",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "userScore",
      "docs": [
        "A user's applied 5A score, PDA `[\"user-score\", user]`. Read by staking-protocol and",
        "private-poll as a participation gate."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "authenticity",
            "type": "u16"
          },
          {
            "name": "accuracy",
            "type": "u16"
          },
          {
            "name": "agility",
            "type": "u16"
          },
          {
            "name": "activity",
            "type": "u16"
          },
          {
            "name": "approved",
            "type": "u16"
          },
          {
            "name": "compositeScore",
            "type": "u16"
          },
          {
            "name": "lastUpdated",
            "type": "i64"
          },
          {
            "name": "updateCount",
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
      "name": "userVouchStatus",
      "docs": [
        "PDA `[\"vouch-status\", vouchee]`. `vouches_received` counts live vouches: it goes down",
        "when one is evaluated, expires or is declined."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "vouchesReceived",
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
      "name": "vouchClosed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "voucher",
            "type": "pubkey"
          },
          {
            "name": "vouchee",
            "type": "pubkey"
          },
          {
            "name": "outcome",
            "type": {
              "defined": {
                "name": "vouchOutcome"
              }
            }
          },
          {
            "name": "voucheeScore",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "vouchCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "voucher",
            "type": "pubkey"
          },
          {
            "name": "vouchee",
            "type": "pubkey"
          },
          {
            "name": "vouchedAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "vouchOutcome",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "succeeded"
          },
          {
            "name": "failed"
          },
          {
            "name": "expired"
          },
          {
            "name": "declined"
          }
        ]
      }
    },
    {
      "name": "vouchRecord",
      "docs": [
        "A live vouch, PDA `[\"vouch-record\", voucher, vouchee]`. Closed (rent to the voucher) when",
        "evaluated or expired, which also frees the voucher's slot (audit 5A-06)."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "voucher",
            "type": "pubkey"
          },
          {
            "name": "vouchee",
            "type": "pubkey"
          },
          {
            "name": "vouchedAt",
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
      "name": "voucherStats",
      "docs": [
        "PDA `[\"voucher-stats\", voucher]`."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "totalVouchesGiven",
            "type": "u32"
          },
          {
            "name": "successfulVouches",
            "type": "u32"
          },
          {
            "name": "failedVouches",
            "type": "u32"
          },
          {
            "name": "expiredVouches",
            "type": "u32"
          },
          {
            "name": "declinedVouches",
            "type": "u32"
          },
          {
            "name": "vouchesActive",
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
      "name": "fiveAConfigSeed",
      "type": "bytes",
      "value": "[102, 105, 118, 101, 45, 97, 45, 99, 111, 110, 102, 105, 103]"
    },
    {
      "name": "pendingScoreSeed",
      "type": "bytes",
      "value": "[112, 101, 110, 100, 105, 110, 103, 45, 115, 99, 111, 114, 101]"
    },
    {
      "name": "userScoreSeed",
      "type": "bytes",
      "value": "[117, 115, 101, 114, 45, 115, 99, 111, 114, 101]"
    },
    {
      "name": "voucherStatsSeed",
      "type": "bytes",
      "value": "[118, 111, 117, 99, 104, 101, 114, 45, 115, 116, 97, 116, 115]"
    },
    {
      "name": "vouchRecordSeed",
      "type": "bytes",
      "value": "[118, 111, 117, 99, 104, 45, 114, 101, 99, 111, 114, 100]"
    },
    {
      "name": "vouchStatusSeed",
      "type": "bytes",
      "value": "[118, 111, 117, 99, 104, 45, 115, 116, 97, 116, 117, 115]"
    }
  ]
};
