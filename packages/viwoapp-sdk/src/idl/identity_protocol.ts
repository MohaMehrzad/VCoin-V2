/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/identity_protocol.json`.
 */
export type IdentityProtocol = {
  "address": "3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG",
  "metadata": {
    "name": "identityProtocol",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "ViWoApp Identity Protocol with SAS integration and verification levels"
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
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
      "name": "addAttester",
      "discriminator": [
        252,
        9,
        41,
        55,
        134,
        171,
        164,
        244
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
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
          "name": "attester",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "adminLowerLevel",
      "discriminator": [
        26,
        213,
        82,
        109,
        233,
        202,
        230,
        165
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
          "name": "identity",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "identity.owner",
                "account": "identity"
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
          "name": "newLevel",
          "type": "u8"
        }
      ]
    },
    {
      "name": "applyPayment",
      "discriminator": [
        210,
        105,
        49,
        74,
        155,
        158,
        144,
        31
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
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
      "name": "cancelPayment",
      "discriminator": [
        217,
        129,
        71,
        37,
        216,
        193,
        38,
        33
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
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
      "name": "closeIdentity",
      "discriminator": [
        158,
        183,
        97,
        254,
        215,
        179,
        139,
        12
      ],
      "accounts": [
        {
          "name": "identity",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
                  121
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
          "name": "usernameRecord",
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
                  110,
                  97,
                  109,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "identity"
              }
            ]
          }
        },
        {
          "name": "owner",
          "writable": true,
          "signer": true,
          "relations": [
            "identity",
            "usernameRecord"
          ]
        }
      ],
      "args": []
    },
    {
      "name": "createIdentity",
      "discriminator": [
        12,
        253,
        209,
        41,
        176,
        51,
        195,
        179
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "identity",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
                  121
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
          "name": "usernameRecord",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "didHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "username",
          "type": "string"
        }
      ]
    },
    {
      "name": "initialize",
      "docs": [
        "Upgrade authority only; USDC mint (6 decimals) and treasury token account fixed here."
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
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
          "name": "usdcMint"
        },
        {
          "name": "treasuryToken"
        },
        {
          "name": "program",
          "address": "3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG"
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
      "name": "linkAttestation",
      "discriminator": [
        207,
        102,
        227,
        179,
        162,
        63,
        201,
        7
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
          "name": "identity",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
                  121
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
            "identity"
          ]
        },
        {
          "name": "attester",
          "signer": true
        },
        {
          "name": "attestationLink",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  116,
                  116,
                  101,
                  115,
                  116,
                  97,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "arg",
                "path": "attestationId"
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
          "name": "attestationId",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "claims",
          "type": "u16"
        },
        {
          "name": "expiresAt",
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
          "name": "config",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
      "name": "proposePayment",
      "discriminator": [
        102,
        219,
        97,
        51,
        201,
        219,
        247,
        27
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
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
        },
        {
          "name": "usdcMint",
          "relations": [
            "config"
          ]
        },
        {
          "name": "treasuryToken"
        }
      ],
      "args": [
        {
          "name": "prices",
          "type": {
            "array": [
              "u64",
              3
            ]
          }
        }
      ]
    },
    {
      "name": "removeAttester",
      "discriminator": [
        97,
        4,
        116,
        6,
        32,
        73,
        185,
        217
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
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
          "name": "attester",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "revokeAttestation",
      "discriminator": [
        12,
        156,
        103,
        161,
        194,
        246,
        211,
        179
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
          "name": "attestationLink",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  97,
                  116,
                  116,
                  101,
                  115,
                  116,
                  97,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "attestation_link.attestation_id",
                "account": "attestationLink"
              }
            ]
          }
        },
        {
          "name": "identity",
          "docs": [
            "The owner's identity, if it still exists."
          ],
          "writable": true,
          "optional": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "attestation_link.owner",
                "account": "attestationLink"
              }
            ]
          }
        },
        {
          "name": "rentPayer",
          "writable": true,
          "relations": [
            "attestationLink"
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
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
      "name": "subscribe",
      "discriminator": [
        254,
        28,
        191,
        138,
        156,
        179,
        183,
        53
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
          "name": "subscription",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  115,
                  117,
                  98,
                  115,
                  99,
                  114,
                  105,
                  112,
                  116,
                  105,
                  111,
                  110
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
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "userUsdc",
          "writable": true
        },
        {
          "name": "treasuryToken",
          "writable": true,
          "relations": [
            "config"
          ]
        },
        {
          "name": "usdcMint",
          "relations": [
            "config"
          ]
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "tier",
          "type": "u8"
        },
        {
          "name": "maxPayment",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updateDidHash",
      "discriminator": [
        99,
        154,
        144,
        88,
        221,
        122,
        113,
        33
      ],
      "accounts": [
        {
          "name": "config",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
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
          "name": "identity",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  105,
                  100,
                  101,
                  110,
                  116,
                  105,
                  116,
                  121
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
            "identity"
          ]
        }
      ],
      "args": [
        {
          "name": "didHash",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "attestationLink",
      "discriminator": [
        2,
        119,
        213,
        81,
        215,
        100,
        101,
        10
      ]
    },
    {
      "name": "identity",
      "discriminator": [
        58,
        132,
        5,
        12,
        176,
        164,
        85,
        112
      ]
    },
    {
      "name": "identityConfig",
      "discriminator": [
        211,
        169,
        51,
        80,
        164,
        108,
        157,
        36
      ]
    },
    {
      "name": "subscription",
      "discriminator": [
        64,
        7,
        26,
        135,
        102,
        132,
        98,
        33
      ]
    },
    {
      "name": "usernameRecord",
      "discriminator": [
        42,
        172,
        136,
        41,
        240,
        123,
        100,
        204
      ]
    }
  ],
  "events": [
    {
      "name": "attestationLinked",
      "discriminator": [
        152,
        15,
        82,
        199,
        156,
        90,
        9,
        47
      ]
    },
    {
      "name": "attestationRevoked",
      "discriminator": [
        47,
        106,
        65,
        238,
        200,
        127,
        163,
        50
      ]
    },
    {
      "name": "attesterAdded",
      "discriminator": [
        82,
        0,
        46,
        55,
        0,
        75,
        21,
        136
      ]
    },
    {
      "name": "attesterRemoved",
      "discriminator": [
        246,
        184,
        57,
        239,
        78,
        217,
        73,
        192
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
      "name": "didHashUpdated",
      "discriminator": [
        90,
        58,
        184,
        207,
        60,
        198,
        204,
        94
      ]
    },
    {
      "name": "identityClosed",
      "discriminator": [
        158,
        208,
        39,
        137,
        100,
        208,
        216,
        163
      ]
    },
    {
      "name": "identityCreated",
      "discriminator": [
        247,
        185,
        231,
        174,
        133,
        94,
        200,
        142
      ]
    },
    {
      "name": "identityInitialized",
      "discriminator": [
        135,
        129,
        79,
        89,
        88,
        118,
        169,
        194
      ]
    },
    {
      "name": "levelLowered",
      "discriminator": [
        242,
        222,
        205,
        218,
        226,
        24,
        50,
        80
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
      "name": "paymentChangeApplied",
      "discriminator": [
        145,
        118,
        48,
        117,
        67,
        111,
        94,
        2
      ]
    },
    {
      "name": "paymentChangeCancelled",
      "discriminator": [
        152,
        199,
        196,
        248,
        56,
        226,
        210,
        203
      ]
    },
    {
      "name": "paymentChangeProposed",
      "discriminator": [
        128,
        66,
        254,
        242,
        47,
        183,
        169,
        12
      ]
    },
    {
      "name": "subscribed",
      "discriminator": [
        135,
        59,
        105,
        76,
        190,
        236,
        138,
        228
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "paused",
      "msg": "Identity is paused"
    },
    {
      "code": 6001,
      "name": "invalidProgramData",
      "msg": "Program data account does not belong to this program"
    },
    {
      "code": 6002,
      "name": "invalidUsdcMint",
      "msg": "USDC mint must have 6 decimals"
    },
    {
      "code": 6003,
      "name": "invalidUsername",
      "msg": "Usernames are 3-32 characters of a-z, 0-9 and _"
    },
    {
      "code": 6004,
      "name": "invalidAttester",
      "msg": "Invalid attester key"
    },
    {
      "code": 6005,
      "name": "attesterExists",
      "msg": "Attester already trusted"
    },
    {
      "code": 6006,
      "name": "tooManyAttesters",
      "msg": "All attester slots are used"
    },
    {
      "code": 6007,
      "name": "attesterNotFound",
      "msg": "Attester not found"
    },
    {
      "code": 6008,
      "name": "notTrustedAttester",
      "msg": "Signer is not a trusted attester"
    },
    {
      "code": 6009,
      "name": "selfAttestation",
      "msg": "An attester cannot attest itself"
    },
    {
      "code": 6010,
      "name": "invalidClaims",
      "msg": "Unknown claim bits"
    },
    {
      "code": 6011,
      "name": "invalidExpiry",
      "msg": "Expiry must be in the future and at most two years away"
    },
    {
      "code": 6012,
      "name": "attestationAlreadyLinked",
      "msg": "This attestation is linked to another wallet"
    },
    {
      "code": 6013,
      "name": "notRevoker",
      "msg": "Only the issuing attester or the authority may revoke"
    },
    {
      "code": 6014,
      "name": "notADowngrade",
      "msg": "Admin can only lower a level"
    },
    {
      "code": 6015,
      "name": "invalidTier",
      "msg": "Tier must be 1, 2 or 3"
    },
    {
      "code": 6016,
      "name": "cannotDowngradeActive",
      "msg": "Cannot downgrade while a higher tier is active"
    },
    {
      "code": 6017,
      "name": "priceAboveMax",
      "msg": "Price exceeds the maximum you signed"
    },
    {
      "code": 6018,
      "name": "noPendingChange",
      "msg": "No payment change pending"
    },
    {
      "code": 6019,
      "name": "timelockActive",
      "msg": "Payment change timelock still active"
    },
    {
      "code": 6020,
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
      "name": "attestationLink",
      "docs": [
        "Binds an attestation id to exactly one wallet, PDA `[\"attestation\", attestation_id]`",
        "(audit ID-03: one proof cannot verify many wallets)."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "attestationId",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "attester",
            "type": "pubkey"
          },
          {
            "name": "claims",
            "type": "u16"
          },
          {
            "name": "level",
            "type": "u8"
          },
          {
            "name": "expiresAt",
            "type": "i64"
          },
          {
            "name": "linkedAt",
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
      "name": "attestationLinked",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "attester",
            "type": "pubkey"
          },
          {
            "name": "attestationId",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "claims",
            "type": "u16"
          },
          {
            "name": "level",
            "type": "u8"
          },
          {
            "name": "expiresAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "attestationRevoked",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "attestationId",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "revokedBy",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "attesterAdded",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "attester",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "attesterRemoved",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "attester",
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
      "name": "didHashUpdated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "didHash",
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
      "name": "identity",
      "docs": [
        "One per wallet, PDA `[\"identity\", owner]`."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "didHash",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "username",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "usernameLen",
            "type": "u8"
          },
          {
            "name": "level",
            "docs": [
              "Level from the current attestation (see `effective_level`)."
            ],
            "type": "u8"
          },
          {
            "name": "claims",
            "type": "u16"
          },
          {
            "name": "attester",
            "type": "pubkey"
          },
          {
            "name": "attestationId",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "levelExpiresAt",
            "type": "i64"
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
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "identityClosed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "identityConfig",
      "docs": [
        "Singleton, PDA `[\"identity-config\"]`."
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
            "name": "attesters",
            "docs": [
              "Trusted attesters (`Pubkey::default()` = empty slot). Removing one immediately voids",
              "the levels it granted (see [`Identity::effective_level`])."
            ],
            "type": {
              "array": [
                "pubkey",
                10
              ]
            }
          },
          {
            "name": "usdcMint",
            "docs": [
              "USDC mint (6 decimals), fixed at initialization."
            ],
            "type": "pubkey"
          },
          {
            "name": "treasuryToken",
            "docs": [
              "Token account (USDC) that receives subscription payments."
            ],
            "type": "pubkey"
          },
          {
            "name": "prices",
            "docs": [
              "Price per 30 days for tiers 1..=3."
            ],
            "type": {
              "array": [
                "u64",
                3
              ]
            }
          },
          {
            "name": "pendingPrices",
            "docs": [
              "Timelocked payment change (`pending_eta == 0` = none)."
            ],
            "type": {
              "array": [
                "u64",
                3
              ]
            }
          },
          {
            "name": "pendingTreasuryToken",
            "type": "pubkey"
          },
          {
            "name": "pendingEta",
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
      "name": "identityCreated",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "username",
            "type": "string"
          },
          {
            "name": "didHash",
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
      "name": "identityInitialized",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "usdcMint",
            "type": "pubkey"
          },
          {
            "name": "treasuryToken",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "levelLowered",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "oldLevel",
            "type": "u8"
          },
          {
            "name": "newLevel",
            "type": "u8"
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
      "name": "paymentChangeApplied",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "prices",
            "type": {
              "array": [
                "u64",
                3
              ]
            }
          },
          {
            "name": "treasuryToken",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "paymentChangeCancelled",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "paymentChangeProposed",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "prices",
            "type": {
              "array": [
                "u64",
                3
              ]
            }
          },
          {
            "name": "treasuryToken",
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
      "name": "subscribed",
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
            "name": "paid",
            "type": "u64"
          },
          {
            "name": "expiresAt",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "subscription",
      "docs": [
        "PDA `[\"subscription\", user]`."
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
            "docs": [
              "1..=3 (0 = never subscribed)."
            ],
            "type": "u8"
          },
          {
            "name": "startedAt",
            "type": "i64"
          },
          {
            "name": "expiresAt",
            "type": "i64"
          },
          {
            "name": "totalPaid",
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
      "name": "usernameRecord",
      "docs": [
        "Reserves a username, PDA `[\"username\", name]`."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
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
      "name": "attestationSeed",
      "type": "bytes",
      "value": "[97, 116, 116, 101, 115, 116, 97, 116, 105, 111, 110]"
    },
    {
      "name": "identityConfigSeed",
      "type": "bytes",
      "value": "[105, 100, 101, 110, 116, 105, 116, 121, 45, 99, 111, 110, 102, 105, 103]"
    },
    {
      "name": "identitySeed",
      "type": "bytes",
      "value": "[105, 100, 101, 110, 116, 105, 116, 121]"
    },
    {
      "name": "subscriptionSeed",
      "type": "bytes",
      "value": "[115, 117, 98, 115, 99, 114, 105, 112, 116, 105, 111, 110]"
    },
    {
      "name": "usernameSeed",
      "type": "bytes",
      "value": "[117, 115, 101, 114, 110, 97, 109, 101]"
    }
  ]
};
