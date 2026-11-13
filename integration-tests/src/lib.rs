//! Shared LiteSVM harness for the VCoin program integration tests.
//!
//! Programs are loaded from `target/deploy/*.so`, so run `anchor build` (or
//! `anchor build --no-idl`) before `cargo test -p integration-tests`.
//! Programs are deployed under the *upgradeable* loader with a real ProgramData
//! account, so `initialize` instructions gated on the upgrade authority can be
//! exercised exactly as on a cluster.

// `TransactionResult` is LiteSVM's own (large) error type.
#![allow(clippy::result_large_err)]

use anchor_lang::{InstructionData, ToAccountMetas};
use litesvm::{types::TransactionResult, LiteSVM};
use solana_loader_v3_interface::state::UpgradeableLoaderState;
use solana_sdk::{
    account::Account,
    clock::Clock,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_sdk_ids::bpf_loader_upgradeable;

pub use litesvm;
pub use solana_sdk;

/// Directory that `anchor build` writes program binaries to.
pub const DEPLOY_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/deploy");

/// One SOL in lamports.
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

/// Reads `target/deploy/<name>.so`, panicking with an actionable message if missing.
pub fn program_so(name: &str) -> Vec<u8> {
    let path = format!("{DEPLOY_DIR}/{name}.so");
    std::fs::read(&path)
        .unwrap_or_else(|e| panic!("cannot read {path}: {e}. Run `anchor build` first."))
}

/// Deploys `so` at `program_id` under the upgradeable BPF loader.
///
/// Writes the ProgramData account (with `upgrade_authority`) before the Program
/// account, which is the order LiteSVM requires to load an upgradeable program.
/// Returns the ProgramData address.
pub fn deploy_upgradeable(
    svm: &mut LiteSVM,
    program_id: Pubkey,
    so: &[u8],
    upgrade_authority: Option<Pubkey>,
) -> Pubkey {
    let loader = bpf_loader_upgradeable::id();
    let (programdata, _) = Pubkey::find_program_address(&[program_id.as_ref()], &loader);

    let header = bincode::serialize(&UpgradeableLoaderState::ProgramData {
        slot: 0,
        upgrade_authority_address: upgrade_authority,
    })
    .expect("serialize ProgramData header");
    let mut data = vec![0u8; UpgradeableLoaderState::size_of_programdata_metadata()];
    data[..header.len()].copy_from_slice(&header);
    data.extend_from_slice(so);
    let lamports = svm.minimum_balance_for_rent_exemption(data.len());
    svm.set_account(
        programdata,
        Account {
            lamports,
            data,
            owner: loader,
            executable: false,
            rent_epoch: 0,
        },
    )
    .expect("set ProgramData account");

    let program = bincode::serialize(&UpgradeableLoaderState::Program {
        programdata_address: programdata,
    })
    .expect("serialize Program account");
    let lamports = svm.minimum_balance_for_rent_exemption(program.len());
    svm.set_account(
        program_id,
        Account {
            lamports,
            data: program,
            owner: loader,
            executable: true,
            rent_epoch: 0,
        },
    )
    .expect("set Program account");
    programdata
}

/// Returns the ProgramData address for an upgradeable program.
pub fn programdata_address(program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[program_id.as_ref()], &bpf_loader_upgradeable::id()).0
}

/// Creates a keypair funded with `sol` SOL.
pub fn funded_keypair(svm: &mut LiteSVM, sol: u64) -> Keypair {
    let kp = Keypair::new();
    svm.airdrop(&kp.pubkey(), sol * LAMPORTS_PER_SOL)
        .expect("airdrop");
    kp
}

/// Builds an Anchor instruction from the generated `accounts::X` and `instruction::X` types.
pub fn ix<A: ToAccountMetas, D: InstructionData>(
    program_id: Pubkey,
    accounts: A,
    data: D,
) -> Instruction {
    Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: data.data(),
    }
}

/// Signs and sends `ixs` with `payer` paying fees, plus any extra signers.
pub fn send(
    svm: &mut LiteSVM,
    ixs: &[Instruction],
    payer: &Keypair,
    extra: &[&Keypair],
) -> TransactionResult {
    let mut signers: Vec<&Keypair> = vec![payer];
    signers.extend_from_slice(extra);
    let tx = Transaction::new_signed_with_payer(
        ixs,
        Some(&payer.pubkey()),
        &signers,
        svm.latest_blockhash(),
    );
    let result = svm.send_transaction(tx);
    svm.expire_blockhash();
    result
}

/// Sets the cluster clock to `unix_timestamp` and advances the slot.
pub fn warp_to(svm: &mut LiteSVM, unix_timestamp: i64) {
    let mut clock: Clock = svm.get_sysvar();
    clock.unix_timestamp = unix_timestamp;
    clock.slot += 1;
    svm.set_sysvar(&clock);
    svm.expire_blockhash();
}

/// Advances the cluster clock by `seconds`.
pub fn warp_by(svm: &mut LiteSVM, seconds: i64) {
    let now = now(svm);
    warp_to(svm, now + seconds);
}

/// Current cluster unix timestamp.
pub fn now(svm: &LiteSVM) -> i64 {
    svm.get_sysvar::<Clock>().unix_timestamp
}

/// Deserializes an Anchor account (discriminator checked).
pub fn read_account<T: anchor_lang::AccountDeserialize>(svm: &LiteSVM, address: &Pubkey) -> T {
    let acc = svm
        .get_account(address)
        .unwrap_or_else(|| panic!("account {address} not found"));
    T::try_deserialize(&mut acc.data.as_slice()).expect("deserialize account")
}

/// Asserts a transaction failed with the given Anchor custom error code.
pub fn assert_anchor_error(result: TransactionResult, code: u32) {
    use solana_sdk::{instruction::InstructionError, transaction::TransactionError};
    match result {
        Ok(_) => panic!("expected custom error {code}, but the transaction succeeded"),
        Err(f) => match f.err {
            TransactionError::InstructionError(_, InstructionError::Custom(c)) if c == code => {}
            other => panic!(
                "expected custom error {code}, got {other:?}\nlogs: {:#?}",
                f.meta.logs
            ),
        },
    }
}

/// Token-2022 helpers shared by the program suites.
pub mod token {
    use super::send;
    use anchor_spl::token_2022::spl_token_2022::{
        self,
        extension::{BaseStateWithExtensions, ExtensionType, StateWithExtensions},
        state::{Account as TokenAccountState, Mint as MintState},
    };
    use litesvm::LiteSVM;
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    };

    /// Creates a Token-2022 account for `mint` owned by `owner`, sized for every account
    /// extension the mint requires (e.g. NonTransferableAccount for soulbound mints).
    pub fn create_token_account(
        svm: &mut LiteSVM,
        payer: &Keypair,
        mint: &Pubkey,
        owner: &Pubkey,
    ) -> Pubkey {
        let mint_data = svm.get_account(mint).expect("mint exists").data;
        let mint_state = StateWithExtensions::<MintState>::unpack(&mint_data).expect("unpack mint");
        let mint_extensions = mint_state.get_extension_types().expect("mint extensions");
        let required = ExtensionType::get_required_init_account_extensions(&mint_extensions);
        let space = ExtensionType::try_calculate_account_len::<TokenAccountState>(&required)
            .expect("account len");

        let account = Keypair::new();
        let lamports = svm.minimum_balance_for_rent_exemption(space);
        let create = solana_system_interface::instruction::create_account(
            &payer.pubkey(),
            &account.pubkey(),
            lamports,
            space as u64,
            &spl_token_2022::ID,
        );
        let init = spl_token_2022::instruction::initialize_account3(
            &spl_token_2022::ID,
            &account.pubkey(),
            mint,
            owner,
        )
        .expect("initialize_account3 ix");
        send(svm, &[create, init], payer, &[&account]).expect("create token account");
        account.pubkey()
    }

    /// Token amount held by a Token-2022 account.
    pub fn balance(svm: &LiteSVM, account: &Pubkey) -> u64 {
        let data = svm.get_account(account).expect("token account exists").data;
        StateWithExtensions::<TokenAccountState>::unpack(&data)
            .expect("unpack token account")
            .base
            .amount
    }

    /// Current supply of a Token-2022 mint.
    pub fn supply(svm: &LiteSVM, mint: &Pubkey) -> u64 {
        let data = svm.get_account(mint).expect("mint exists").data;
        StateWithExtensions::<MintState>::unpack(&data)
            .expect("unpack mint")
            .base
            .supply
    }
}
