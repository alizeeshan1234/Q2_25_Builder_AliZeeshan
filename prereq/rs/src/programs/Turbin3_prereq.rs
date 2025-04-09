use solana_idlgen::idlgen;

idlgen!({
    "version": "0.1.0",
    "name": "turbine_prereq",
    "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa",
    "metadata": {
      "name": "turbine_prereq",
      "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa",
      "version": "0.1.0",
      "spec": "0.1.0",
      "description": "Created with Anchor"
    },
    "instructions": [
      {
        "name": "complete",
        "discriminator": [
          0,
          77,
          224,
          147,
          136,
          25,
          88,
          76
        ],
        "accounts": [
          {
            "name": "signer",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "prereq",
            "isMut": true,
            "isSigner": false,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "type": "bytes",
                  "value": [
                    112,
                    114,
                    101,
                    114,
                    101,
                    113
                  ]
                },
                {
                  "kind": "account",
                  "type": "pubkey",
                  "path": "signer"
                }
              ]
            }
          },
          {
            "name": "system_program",
            "isMut": false,
            "isSigner": false,
            "address": "11111111111111111111111111111111"
          }
        ],
        "args": [
          {
            "name": "github",
            "type": "bytes"
          }
        ]
      },
      {
        "name": "update",
        "discriminator": [
          219,
          200,
          88,
          176,
          158,
          63,
          253,
          127
        ],
        "accounts": [
          {
            "name": "signer",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "prereq",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "system_program",
            "isMut": false,
            "isSigner": false,
            "address": "11111111111111111111111111111111"
          }
        ],
        "args": [
          {
            "name": "github",
            "type": "bytes"
          }
        ]
      }
    ],
    "accounts": [
      {
        "name": "SolanaCohort5Account",
        "type": {
            "kind": "struct",
            "fields": [
                {
                    "name": "github",
                    "type": "bytes"
                },
                {
                    "name": "key",
                    "type": "pubkey"
                }
            ]
        },
        "discriminator": [
          167,
          81,
          85,
          136,
          32,
          169,
          137,
          77
        ]
      }
    ],
    "errors": [
      {
        "code": 6000,
        "name": "InvalidGithubAccount",
        "type": "pubkey",
        "msg": "Invalid Github account"
      }
    ],
    "types": [
      {
        "name": "SolanaCohort5Account",
        "version": "0.1.0",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "github",
              "type": "bytes"
            },
            {
              "name": "key",
              "type": "pubkey"
            }
          ]
        }
      }
    ]
});
