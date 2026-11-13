//! Every built program loads under the upgradeable loader with a real ProgramData account.

use integration_tests::solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use integration_tests::{deploy_upgradeable, litesvm::LiteSVM, program_so, programdata_address};

/// (binary name, program id) for every program in the workspace.
const PROGRAMS: &[(&str, &str)] = &[
    (
        "vcoin_token",
        "Gg1dtrjAfGYi6NLC31WaJjZNBoucvD98rK2h1u9qrUjn",
    ),
    (
        "vevcoin_token",
        "FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx",
    ),
    (
        "staking_protocol",
        "6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu",
    ),
    (
        "identity_protocol",
        "3egAds3pFR5oog6iQCN42KPvgih8HQz2FGybNjiVWixG",
    ),
    (
        "five_a_protocol",
        "783PbtJw5cc7yatnr9fsvTGSnkKaV6iJe6E8VUPTYrT8",
    ),
    (
        "content_registry",
        "MJn1A4MPCBPJGWWuZrtq7bHSo2G289sUwW3ej2wcmLV",
    ),
    (
        "private_poll",
        "3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe",
    ),
    (
        "sscre_protocol",
        "6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ",
    ),
    (
        "vilink_protocol",
        "CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7",
    ),
    (
        "gasless_protocol",
        "FcXJAjzJs8eVY2WTRFXynQBpC7WZUqKZppyp9xS6PaB3",
    ),
];

#[test]
fn every_program_loads_as_upgradeable_with_programdata() {
    let mut svm = LiteSVM::new();
    let upgrade_authority = Keypair::new().pubkey();
    for (name, id) in PROGRAMS {
        let program_id: Pubkey = id.parse().expect("valid program id");
        let programdata = deploy_upgradeable(
            &mut svm,
            program_id,
            &program_so(name),
            Some(upgrade_authority),
        );
        assert_eq!(programdata, programdata_address(&program_id));
        let program = svm.get_account(&program_id).expect("program account");
        assert!(program.executable, "{name} not executable");
        let data = svm.get_account(&programdata).expect("programdata account");
        assert!(data.data.len() > 45, "{name} programdata missing ELF");
    }
}
