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

/// Deploys and initializes vcoin-token, vevcoin-token and staking-protocol (the token +
/// staking stack every other suite builds on).
pub mod stack {
    use super::{deploy_upgradeable, funded_keypair, ix, program_so, send, token};
    use anchor_lang::system_program;
    use anchor_spl::token_2022::spl_token_2022;
    use litesvm::LiteSVM;
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    };

    pub const VCOIN: u64 = 1_000_000_000;

    pub struct Stack {
        pub svm: LiteSVM,
        /// Upgrade authority of every program and initial admin.
        pub admin: Keypair,
        pub vcoin_config: Pubkey,
        pub vcoin_mint: Pubkey,
        pub vevcoin_config: Pubkey,
        pub vevcoin_mint: Pubkey,
        pub registrar: Pubkey,
        pub vault: Pubkey,
    }

    pub struct Staker {
        pub kp: Keypair,
        pub vcoin: Pubkey,
        pub vevcoin: Pubkey,
        pub voter: Pubkey,
    }

    pub fn pda(seeds: &[&[u8]], program: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(seeds, program).0
    }

    /// Deploys the three programs (upgrade authority = `admin`) without initializing them.
    pub fn deploy() -> Stack {
        let mut svm = LiteSVM::new();
        let admin = funded_keypair(&mut svm, 100);
        for (id, name) in [
            (vcoin_token::ID, "vcoin_token"),
            (vevcoin_token::ID, "vevcoin_token"),
            (staking_protocol::ID, "staking_protocol"),
        ] {
            deploy_upgradeable(&mut svm, id, &program_so(name), Some(admin.pubkey()));
        }
        Stack {
            svm,
            admin,
            vcoin_config: pda(
                &[vcoin_token::constants::VCOIN_CONFIG_SEED],
                &vcoin_token::ID,
            ),
            vcoin_mint: pda(&[vcoin_token::constants::VCOIN_MINT_SEED], &vcoin_token::ID),
            vevcoin_config: pda(
                &[vevcoin_token::constants::VEVCOIN_CONFIG_SEED],
                &vevcoin_token::ID,
            ),
            vevcoin_mint: pda(
                &[vevcoin_token::constants::VEVCOIN_MINT_SEED],
                &vevcoin_token::ID,
            ),
            registrar: pda(
                &[staking_protocol::constants::REGISTRAR_SEED],
                &staking_protocol::ID,
            ),
            vault: pda(
                &[staking_protocol::constants::VAULT_SEED],
                &staking_protocol::ID,
            ),
        }
    }

    pub fn programdata(program: &Pubkey) -> Pubkey {
        super::programdata_address(program)
    }

    pub fn init_tokens(s: &mut Stack) {
        let admin = s.admin.insecure_clone();
        let vcoin_init = ix(
            vcoin_token::ID,
            vcoin_token::accounts::Initialize {
                authority: admin.pubkey(),
                config: s.vcoin_config,
                mint: s.vcoin_mint,
                program: vcoin_token::ID,
                program_data: programdata(&vcoin_token::ID),
                token_program: spl_token_2022::ID,
                system_program: system_program::ID,
            },
            vcoin_token::instruction::Initialize {
                name: "VCoin".into(),
                symbol: "VIWO".into(),
                uri: String::new(),
            },
        );
        let vevcoin_init = ix(
            vevcoin_token::ID,
            vevcoin_token::accounts::Initialize {
                authority: admin.pubkey(),
                config: s.vevcoin_config,
                mint: s.vevcoin_mint,
                program: vevcoin_token::ID,
                program_data: programdata(&vevcoin_token::ID),
                token_program: spl_token_2022::ID,
                system_program: system_program::ID,
            },
            vevcoin_token::instruction::Initialize {},
        );
        send(&mut s.svm, &[vcoin_init, vevcoin_init], &admin, &[]).expect("init vcoin + vevcoin");
    }

    pub fn create_registrar_ix(s: &Stack, signer: Pubkey) -> solana_sdk::instruction::Instruction {
        ix(
            staking_protocol::ID,
            staking_protocol::accounts::CreateRegistrar {
                authority: signer,
                registrar: s.registrar,
                vcoin_mint: s.vcoin_mint,
                vevcoin_mint: s.vevcoin_mint,
                vault: s.vault,
                program: staking_protocol::ID,
                program_data: programdata(&staking_protocol::ID),
                token_program: spl_token_2022::ID,
                system_program: system_program::ID,
            },
            staking_protocol::instruction::CreateRegistrar {},
        )
    }

    /// Deploys and fully initializes the stack.
    pub fn setup() -> Stack {
        let mut s = deploy();
        init_tokens(&mut s);
        let admin = s.admin.insecure_clone();
        let ixn = create_registrar_ix(&s, admin.pubkey());
        send(&mut s.svm, &[ixn], &admin, &[]).expect("create registrar");
        s
    }

    /// Mints `amount` VCoin to a fresh token account owned by `owner`.
    pub fn fund_vcoin(s: &mut Stack, owner: &Pubkey, amount: u64) -> Pubkey {
        let admin = s.admin.insecure_clone();
        let account = token::create_token_account(&mut s.svm, &admin, &s.vcoin_mint, owner);
        if amount > 0 {
            let mint = ix(
                vcoin_token::ID,
                vcoin_token::accounts::MintTokens {
                    config: s.vcoin_config,
                    mint: s.vcoin_mint,
                    destination: account,
                    authority: admin.pubkey(),
                    token_program: spl_token_2022::ID,
                },
                vcoin_token::instruction::MintTokens { amount },
            );
            send(&mut s.svm, &[mint], &admin, &[]).expect("mint vcoin");
        }
        account
    }

    /// A funded user with VCoin, an empty veVCoin account and a voter account.
    pub fn new_staker(s: &mut Stack, vcoin_amount: u64) -> Staker {
        let kp = funded_keypair(&mut s.svm, 10);
        let vcoin = fund_vcoin(s, &kp.pubkey(), vcoin_amount);
        let admin = s.admin.insecure_clone();
        let vevcoin =
            token::create_token_account(&mut s.svm, &admin, &s.vevcoin_mint, &kp.pubkey());
        let voter = pda(
            &[
                staking_protocol::constants::VOTER_SEED,
                kp.pubkey().as_ref(),
            ],
            &staking_protocol::ID,
        );
        let create = ix(
            staking_protocol::ID,
            staking_protocol::accounts::CreateVoter {
                registrar: s.registrar,
                voter,
                owner: kp.pubkey(),
                payer: kp.pubkey(),
                system_program: system_program::ID,
            },
            staking_protocol::instruction::CreateVoter {},
        );
        send(&mut s.svm, &[create], &kp, &[]).expect("create voter");
        Staker {
            kp,
            vcoin,
            vevcoin,
            voter,
        }
    }

    pub fn move_stake_accounts(s: &Stack, st: &Staker) -> staking_protocol::accounts::MoveStake {
        staking_protocol::accounts::MoveStake {
            registrar: s.registrar,
            voter: st.voter,
            owner: st.kp.pubkey(),
            vault: s.vault,
            vcoin_mint: s.vcoin_mint,
            owner_vcoin: st.vcoin,
            vevcoin_mint: s.vevcoin_mint,
            owner_vevcoin: st.vevcoin,
            vevcoin_config: s.vevcoin_config,
            vevcoin_program: vevcoin_token::ID,
            token_program: spl_token_2022::ID,
        }
    }

    pub fn deposit_ix(
        s: &Stack,
        st: &Staker,
        index: u8,
        amount: u64,
        lock_secs: i64,
    ) -> solana_sdk::instruction::Instruction {
        ix(
            staking_protocol::ID,
            move_stake_accounts(s, st),
            staking_protocol::instruction::Deposit {
                index,
                amount,
                lock_secs,
            },
        )
    }

    /// Withdraw with no governance wired (the token owner record is not consulted).
    pub fn withdraw_ix(
        s: &Stack,
        st: &Staker,
        index: u8,
        amount: u64,
    ) -> solana_sdk::instruction::Instruction {
        withdraw_ix_with_record(s, st, index, amount, Pubkey::new_unique())
    }

    /// Withdraw passing the owner's Realms TokenOwnerRecord.
    pub fn withdraw_ix_with_record(
        s: &Stack,
        st: &Staker,
        index: u8,
        amount: u64,
        token_owner_record: Pubkey,
    ) -> solana_sdk::instruction::Instruction {
        ix(
            staking_protocol::ID,
            staking_protocol::accounts::Withdraw {
                stake: move_stake_accounts(s, st),
                token_owner_record,
            },
            staking_protocol::instruction::Withdraw { index, amount },
        )
    }
}
