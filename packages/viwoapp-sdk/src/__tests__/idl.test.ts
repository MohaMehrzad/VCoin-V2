import { Connection, Keypair, PublicKey } from "@solana/web3.js";

import { IDLS, PROGRAM_IDS, SEEDS, pda, createPrograms } from "../index";

describe("program ids and seeds come from the IDLs", () => {
  test("program ids are the deployed ids", () => {
    expect(PROGRAM_IDS.staking.toBase58()).toBe("6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu");
    expect(PROGRAM_IDS.privatePoll.toBase58()).toBe("3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe");
    expect(PROGRAM_IDS.sscre.toBase58()).toBe("6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ");
    expect(PROGRAM_IDS.vilink.toBase58()).toBe("CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7");
    for (const [name, idl] of Object.entries(IDLS)) {
      expect(PROGRAM_IDS[name as keyof typeof IDLS].toBase58()).toBe(idl.address);
    }
  });

  test("seeds decode to the program's byte strings", () => {
    expect(SEEDS.vcoinConfig.toString()).toBe("vcoin-config-v2");
    expect(SEEDS.voter.toString()).toBe("voter");
    expect(SEEDS.voterWeightRecord.toString()).toBe("voter-weight-record");
    expect(SEEDS.contentRecord.toString()).toBe("content-record");
    expect(SEEDS.receipt.toString()).toBe("exec");
    expect(SEEDS.pollConfig.toString()).toBe("poll-config");
  });

  test("PDAs match a direct derivation", () => {
    const owner = Keypair.generate().publicKey;
    expect(pda.voter(owner)).toEqual(
      PublicKey.findProgramAddressSync([Buffer.from("voter"), owner.toBuffer()], PROGRAM_IDS.staking)[0],
    );
    const epoch = pda.epoch(7n);
    const le = Buffer.alloc(8);
    le.writeBigUInt64LE(7n);
    expect(epoch).toEqual(PublicKey.findProgramAddressSync([Buffer.from("epoch"), le], PROGRAM_IDS.sscre)[0]);
    expect(pda.contentRecord(owner, new Uint8Array(32).fill(1))).toEqual(
      PublicKey.findProgramAddressSync(
        [Buffer.from("content-record"), owner.toBuffer(), Buffer.alloc(32, 1)],
        PROGRAM_IDS.content,
      )[0],
    );
  });

  test("every program exposes its accounts through the IDL coder", () => {
    const p = createPrograms(new Connection("http://127.0.0.1:8899"));
    expect(Object.keys(p.staking.account)).toEqual(
      expect.arrayContaining(["registrar", "voter", "voterWeightRecord", "maxVoterWeightRecord", "slashRequest"]),
    );
    expect(Object.keys(p.privatePoll.account)).toEqual(expect.arrayContaining(["poll", "ballotRecord", "pollConfig"]));
  });
});
