//! identity-protocol in LiteSVM with a legacy SPL-Token USDC mint (6 decimals).
//! Audit ID-01..10, ID-L1..L3.
#![allow(clippy::result_large_err)] // TransactionResult, as in the harness

use anchor_lang::{prelude::Pubkey, system_program};
use anchor_spl::associated_token::{
    get_associated_token_address_with_program_id,
    spl_associated_token_account::instruction::create_associated_token_account,
};
use anchor_spl::token::spl_token;
use identity_protocol::{
    constants::*,
    errors::IdentityError,
    state::{Identity, IdentityConfig, Subscription},
};
use integration_tests::solana_sdk::{instruction::Instruction, signature::Keypair, signer::Signer};
use integration_tests::*;
use litesvm::{types::TransactionResult, LiteSVM};
use vcoin_common::CommonError;

const USDC: u64 = 1_000_000;

fn err(e: IdentityError) -> u32 {
    6000 + e as u32
}

fn pda(seeds: &[&[u8]]) -> Pubkey {
    Pubkey::find_program_address(seeds, &identity_protocol::ID).0
}

fn config_pda() -> Pubkey {
    pda(&[IDENTITY_CONFIG_SEED])
}

fn identity_pda(owner: &Pubkey) -> Pubkey {
    pda(&[IDENTITY_SEED, owner.as_ref()])
}

struct Env {
    svm: LiteSVM,
    admin: Keypair,
    usdc: Pubkey,
    treasury: Pubkey,
    attester: Keypair,
}

fn create_mint(svm: &mut LiteSVM, payer: &Keypair, decimals: u8) -> Pubkey {
    let mint = Keypair::new();
    let ixs = [
        solana_system_interface::instruction::create_account(
            &payer.pubkey(),
            &mint.pubkey(),
            svm.minimum_balance_for_rent_exemption(82),
            82,
            &spl_token::ID,
        ),
        spl_token::instruction::initialize_mint2(
            &spl_token::ID,
            &mint.pubkey(),
            &payer.pubkey(),
            None,
            decimals,
        )
        .unwrap(),
    ];
    send(svm, &ixs, payer, &[&mint]).expect("mint");
    mint.pubkey()
}

fn ata(svm: &mut LiteSVM, payer: &Keypair, mint: &Pubkey, owner: &Pubkey) -> Pubkey {
    let create = create_associated_token_account(&payer.pubkey(), owner, mint, &spl_token::ID);
    send(svm, &[create], payer, &[]).expect("ata");
    get_associated_token_address_with_program_id(owner, mint, &spl_token::ID)
}

fn init_ix(authority: Pubkey, usdc: Pubkey, treasury: Pubkey) -> Instruction {
    ix(
        identity_protocol::ID,
        identity_protocol::accounts::Initialize {
            authority,
            config: config_pda(),
            usdc_mint: usdc,
            treasury_token: treasury,
            program: identity_protocol::ID,
            program_data: programdata_address(&identity_protocol::ID),
            system_program: system_program::ID,
        },
        identity_protocol::instruction::Initialize {},
    )
}

fn admin_ix<D: anchor_lang::InstructionData>(authority: Pubkey, data: D) -> Instruction {
    ix(
        identity_protocol::ID,
        identity_protocol::accounts::UpdateConfig {
            config: config_pda(),
            authority,
        },
        data,
    )
}

/// Deployed, initialized, one trusted attester.
fn env() -> Env {
    let mut svm = LiteSVM::new();
    let admin = funded_keypair(&mut svm, 100);
    deploy_upgradeable(
        &mut svm,
        identity_protocol::ID,
        &program_so("identity_protocol"),
        Some(admin.pubkey()),
    );
    let usdc = create_mint(&mut svm, &admin, 6);
    let treasury = ata(&mut svm, &admin, &usdc, &admin.pubkey());
    send(
        &mut svm,
        &[init_ix(admin.pubkey(), usdc, treasury)],
        &admin,
        &[],
    )
    .expect("init");
    let attester = funded_keypair(&mut svm, 5);
    let add = admin_ix(
        admin.pubkey(),
        identity_protocol::instruction::AddAttester {
            attester: attester.pubkey(),
        },
    );
    send(&mut svm, &[add], &admin, &[]).expect("add attester");
    Env {
        svm,
        admin,
        usdc,
        treasury,
        attester,
    }
}

fn create_ix(owner: &Pubkey, username: &str) -> Instruction {
    // Same 32-byte cap as the program's seed.
    let record = pda(&[
        USERNAME_SEED,
        &username.as_bytes()[..username.len().min(32)],
    ]);
    ix(
        identity_protocol::ID,
        identity_protocol::accounts::CreateIdentity {
            config: config_pda(),
            owner: *owner,
            identity: identity_pda(owner),
            username_record: record,
            system_program: system_program::ID,
        },
        identity_protocol::instruction::CreateIdentity {
            did_hash: [1; 32],
            username: username.to_string(),
        },
    )
}

fn user_with_identity(e: &mut Env, username: &str) -> Keypair {
    let kp = funded_keypair(&mut e.svm, 5);
    send(&mut e.svm, &[create_ix(&kp.pubkey(), username)], &kp, &[]).expect("create identity");
    kp
}

fn link_ix(
    owner: &Pubkey,
    attester: &Pubkey,
    id: [u8; 32],
    claims: u16,
    expires_at: i64,
) -> Instruction {
    ix(
        identity_protocol::ID,
        identity_protocol::accounts::LinkAttestation {
            config: config_pda(),
            identity: identity_pda(owner),
            owner: *owner,
            attester: *attester,
            attestation_link: pda(&[ATTESTATION_SEED, id.as_ref()]),
            system_program: system_program::ID,
        },
        identity_protocol::instruction::LinkAttestation {
            attestation_id: id,
            claims,
            expires_at,
        },
    )
}

fn link(
    e: &mut Env,
    owner: &Keypair,
    id: [u8; 32],
    claims: u16,
    expires_at: i64,
) -> TransactionResult {
    let attester = e.attester.insecure_clone();
    send(
        &mut e.svm,
        &[link_ix(
            &owner.pubkey(),
            &attester.pubkey(),
            id,
            claims,
            expires_at,
        )],
        owner,
        &[&attester],
    )
}

fn effective(e: &Env, owner: &Pubkey) -> u8 {
    let id: Identity = read_account(&e.svm, &identity_pda(owner));
    let cfg: IdentityConfig = read_account(&e.svm, &config_pda());
    id.effective_level(now(&e.svm), &cfg)
}

#[test]
fn usernames_are_unique_validated_and_freed_on_close() {
    let mut e = env();
    let alice = user_with_identity(&mut e, "alice");
    let bob = funded_keypair(&mut e.svm, 5);
    assert!(
        send(&mut e.svm, &[create_ix(&bob.pubkey(), "alice")], &bob, &[]).is_err(),
        "taken"
    );
    for bad in ["Al", "al-ice", &"x".repeat(33)] {
        let r = send(&mut e.svm, &[create_ix(&bob.pubkey(), bad)], &bob, &[]);
        assert_anchor_error(r, err(IdentityError::InvalidUsername));
    }

    // An unverified owner can fix its DID (ID-L1).
    let update = ix(
        identity_protocol::ID,
        identity_protocol::accounts::UpdateIdentity {
            config: config_pda(),
            identity: identity_pda(&alice.pubkey()),
            owner: alice.pubkey(),
        },
        identity_protocol::instruction::UpdateDidHash { did_hash: [2; 32] },
    );
    send(&mut e.svm, &[update], &alice, &[]).expect("update did");
    assert_eq!(
        read_account::<Identity>(&e.svm, &identity_pda(&alice.pubkey())).did_hash,
        [2; 32]
    );

    let close = ix(
        identity_protocol::ID,
        identity_protocol::accounts::CloseIdentity {
            identity: identity_pda(&alice.pubkey()),
            username_record: pda(&[USERNAME_SEED, b"alice"]),
            owner: alice.pubkey(),
        },
        identity_protocol::instruction::CloseIdentity {},
    );
    send(&mut e.svm, &[close], &alice, &[]).expect("close");
    send(&mut e.svm, &[create_ix(&bob.pubkey(), "alice")], &bob, &[]).expect("username freed");

    let admin = e.admin.insecure_clone();
    let pause = admin_ix(
        admin.pubkey(),
        identity_protocol::instruction::SetPaused { paused: true },
    );
    send(&mut e.svm, &[pause], &admin, &[]).expect("pause");
    let carol = funded_keypair(&mut e.svm, 5);
    let r = send(
        &mut e.svm,
        &[create_ix(&carol.pubkey(), "carol")],
        &carol,
        &[],
    );
    assert_anchor_error(r, err(IdentityError::Paused));
}

#[test]
fn levels_move_both_ways_expire_and_can_be_revoked() {
    let mut e = env();
    let alice = user_with_identity(&mut e, "alice");
    let year = now(&e.svm) + 365 * DAY;
    let id = [7u8; 32];

    link(&mut e, &alice, id, CLAIM_EMAIL | CLAIM_PHONE, year).expect("basic");
    assert_eq!(effective(&e, &alice.pubkey()), LEVEL_BASIC);
    link(&mut e, &alice, id, CLAIM_KYC | CLAIM_BIOMETRIC, year).expect("upgrade (ID-01)");
    assert_eq!(effective(&e, &alice.pubkey()), LEVEL_FULL);
    link(
        &mut e,
        &alice,
        id,
        CLAIM_EMAIL | CLAIM_PHONE | CLAIM_SOCIAL,
        year,
    )
    .expect("refresh lowers too");
    assert_eq!(
        effective(&e, &alice.pubkey()),
        LEVEL_BASIC,
        "no documents, no KYC (ID-04)"
    );

    // One attestation, one wallet (ID-03); attesters cannot attest themselves.
    let bob = user_with_identity(&mut e, "bob");
    assert_anchor_error(
        link(&mut e, &bob, id, CLAIM_KYC, year),
        err(IdentityError::AttestationAlreadyLinked),
    );
    let attester = e.attester.insecure_clone();
    send(
        &mut e.svm,
        &[create_ix(&attester.pubkey(), "attester")],
        &attester,
        &[],
    )
    .expect("attester identity");
    let selfie = link_ix(
        &attester.pubkey(),
        &attester.pubkey(),
        [8; 32],
        CLAIM_KYC,
        year,
    );
    assert_anchor_error(
        send(&mut e.svm, &[selfie], &attester, &[]),
        err(IdentityError::SelfAttestation),
    );
    let rogue = funded_keypair(&mut e.svm, 1);
    let untrusted = link_ix(&bob.pubkey(), &rogue.pubkey(), [9; 32], CLAIM_KYC, year);
    assert_anchor_error(
        send(&mut e.svm, &[untrusted], &bob, &[&rogue]),
        err(IdentityError::NotTrustedAttester),
    );
    let far = now(&e.svm) + MAX_ATTESTATION_VALIDITY + 1;
    assert_anchor_error(
        link(&mut e, &bob, [9; 32], CLAIM_KYC, far),
        err(IdentityError::InvalidExpiry),
    );
    assert_anchor_error(
        link(&mut e, &bob, [9; 32], 1 << 6, year),
        err(IdentityError::InvalidClaims),
    );

    // Expiry and attester removal void the level for consumers (ID-02).
    let soon = now(&e.svm) + DAY;
    link(&mut e, &bob, [9; 32], CLAIM_KYC, soon).expect("short attestation");
    assert_eq!(effective(&e, &bob.pubkey()), LEVEL_KYC);
    warp_by(&mut e.svm, DAY);
    assert_eq!(effective(&e, &bob.pubkey()), LEVEL_NONE, "expired");
    assert_eq!(effective(&e, &alice.pubkey()), LEVEL_BASIC);
    let admin = e.admin.insecure_clone();
    let remove = admin_ix(
        admin.pubkey(),
        identity_protocol::instruction::RemoveAttester {
            attester: attester.pubkey(),
        },
    );
    send(&mut e.svm, &[remove], &admin, &[]).expect("remove attester");
    assert_eq!(
        effective(&e, &alice.pubkey()),
        LEVEL_NONE,
        "attester no longer trusted"
    );

    // Revocation by the issuing attester (even after removal); strangers cannot.
    let revoke = |revoker: Pubkey| {
        ix(
            identity_protocol::ID,
            identity_protocol::accounts::RevokeAttestation {
                config: config_pda(),
                attestation_link: pda(&[ATTESTATION_SEED, id.as_ref()]),
                identity: identity_pda(&alice.pubkey()),
                rent_payer: alice.pubkey(),
                revoker,
            },
            identity_protocol::instruction::RevokeAttestation {},
        )
    };
    assert_anchor_error(
        send(&mut e.svm, &[revoke(rogue.pubkey())], &rogue, &[]),
        err(IdentityError::NotRevoker),
    );
    send(&mut e.svm, &[revoke(attester.pubkey())], &attester, &[]).expect("revoke");
    let a: Identity = read_account(&e.svm, &identity_pda(&alice.pubkey()));
    assert_eq!((a.level, a.attester), (LEVEL_NONE, Pubkey::default()));
    assert!(e
        .svm
        .get_account(&pda(&[ATTESTATION_SEED, id.as_ref()]))
        .is_none_or(|x| x.lamports == 0));

    // The authority may lower but never grant.
    let add = admin_ix(
        admin.pubkey(),
        identity_protocol::instruction::AddAttester {
            attester: attester.pubkey(),
        },
    );
    send(&mut e.svm, &[add], &admin, &[]).expect("re-add attester");
    link(&mut e, &alice, [10; 32], CLAIM_KYC, year).expect("kyc");
    let lower = |level: u8| {
        ix(
            identity_protocol::ID,
            identity_protocol::accounts::AdminLowerLevel {
                config: config_pda(),
                identity: identity_pda(&alice.pubkey()),
                authority: admin.pubkey(),
            },
            identity_protocol::instruction::AdminLowerLevel { new_level: level },
        )
    };
    assert_anchor_error(
        send(&mut e.svm, &[lower(LEVEL_FULL)], &admin, &[]),
        err(IdentityError::NotADowngrade),
    );
    send(&mut e.svm, &[lower(LEVEL_BASIC)], &admin, &[]).expect("lower");
    assert_eq!(effective(&e, &alice.pubkey()), LEVEL_BASIC);
}

fn subscribe_ix(
    e: &Env,
    user: &Pubkey,
    user_usdc: Pubkey,
    tier: u8,
    max_payment: u64,
) -> Instruction {
    ix(
        identity_protocol::ID,
        identity_protocol::accounts::Subscribe {
            config: config_pda(),
            subscription: pda(&[SUBSCRIPTION_SEED, user.as_ref()]),
            user: *user,
            user_usdc,
            treasury_token: e.treasury,
            usdc_mint: e.usdc,
            token_program: spl_token::ID,
            system_program: system_program::ID,
        },
        identity_protocol::instruction::Subscribe { tier, max_payment },
    )
}

fn usdc_balance(svm: &LiteSVM, account: &Pubkey) -> u64 {
    let data = svm.get_account(account).unwrap().data;
    u64::from_le_bytes(data[64..72].try_into().unwrap())
}

#[test]
fn subscriptions_extend_prorate_and_refuse_active_downgrades() {
    let mut e = env();
    let admin = e.admin.insecure_clone();
    let user = funded_keypair(&mut e.svm, 5);
    let usdc = e.usdc;
    let wallet = ata(&mut e.svm, &admin, &usdc, &user.pubkey());
    let mint_to = spl_token::instruction::mint_to(
        &spl_token::ID,
        &e.usdc,
        &wallet,
        &admin.pubkey(),
        &[],
        500 * USDC,
    )
    .unwrap();
    send(&mut e.svm, &[mint_to], &admin, &[]).expect("fund user");
    let sub = pda(&[SUBSCRIPTION_SEED, user.pubkey().as_ref()]);
    let buy = |e: &mut Env, tier: u8, max: u64| {
        let i = subscribe_ix(e, &user.pubkey(), wallet, tier, max);
        send(&mut e.svm, &[i], &user, &[])
    };

    for bad in [0u8, 4] {
        assert_anchor_error(buy(&mut e, bad, u64::MAX), err(IdentityError::InvalidTier));
    }
    assert_anchor_error(
        buy(&mut e, 1, 4 * USDC - 1),
        err(IdentityError::PriceAboveMax),
    );
    let start = now(&e.svm);
    buy(&mut e, 1, 4 * USDC).expect("tier 1");
    assert_eq!(
        read_account::<Subscription>(&e.svm, &sub).expires_at,
        start + SUBSCRIPTION_DURATION
    );

    // Renewing 5 days early keeps the remaining time.
    warp_by(&mut e.svm, SUBSCRIPTION_DURATION - 5 * DAY);
    warp_by(&mut e.svm, 0);
    buy(&mut e, 1, 4 * USDC).expect("renew");
    assert_eq!(
        read_account::<Subscription>(&e.svm, &sub).expires_at,
        start + 2 * SUBSCRIPTION_DURATION
    );

    // Upgrade with 35 days of tier 1 left: their value (35/30 of 4 USDC) is credited.
    let before = usdc_balance(&e.svm, &e.treasury);
    buy(&mut e, 3, 59 * USDC).expect("upgrade");
    let s: Subscription = read_account(&e.svm, &sub);
    let paid = usdc_balance(&e.svm, &e.treasury) - before;
    assert!(paid < 59 * USDC && s.tier == 3, "prorated: paid {paid}");
    assert_eq!(s.expires_at, now(&e.svm) + SUBSCRIPTION_DURATION);

    assert_anchor_error(
        buy(&mut e, 1, u64::MAX),
        err(IdentityError::CannotDowngradeActive),
    );
    warp_by(&mut e.svm, SUBSCRIPTION_DURATION);
    buy(&mut e, 1, 4 * USDC).expect("downgrade after expiry");
    assert_eq!(read_account::<Subscription>(&e.svm, &sub).tier, 1);
}

#[test]
fn initialization_is_gated_and_payment_changes_are_timelocked() {
    let mut svm = LiteSVM::new();
    let admin = funded_keypair(&mut svm, 100);
    let stranger = funded_keypair(&mut svm, 10);
    deploy_upgradeable(
        &mut svm,
        identity_protocol::ID,
        &program_so("identity_protocol"),
        Some(admin.pubkey()),
    );
    let usdc = create_mint(&mut svm, &admin, 6);
    let wrong = create_mint(&mut svm, &admin, 9);
    let treasury = ata(&mut svm, &admin, &usdc, &admin.pubkey());
    let wrong_treasury = ata(&mut svm, &admin, &wrong, &admin.pubkey());
    let r = send(
        &mut svm,
        &[init_ix(stranger.pubkey(), usdc, treasury)],
        &stranger,
        &[],
    );
    assert_anchor_error(r, u32::from(CommonError::NotUpgradeAuthority));
    let r = send(
        &mut svm,
        &[init_ix(admin.pubkey(), wrong, wrong_treasury)],
        &admin,
        &[],
    );
    assert_anchor_error(r, err(IdentityError::InvalidUsdcMint));
    send(
        &mut svm,
        &[init_ix(admin.pubkey(), usdc, treasury)],
        &admin,
        &[],
    )
    .expect("init");

    let new_treasury = ata(&mut svm, &admin, &usdc, &stranger.pubkey());
    let propose = ix(
        identity_protocol::ID,
        identity_protocol::accounts::ProposePayment {
            config: config_pda(),
            authority: admin.pubkey(),
            usdc_mint: usdc,
            treasury_token: new_treasury,
        },
        identity_protocol::instruction::ProposePayment {
            prices: [USDC, 2 * USDC, 3 * USDC],
        },
    );
    send(&mut svm, &[propose], &admin, &[]).expect("propose");
    let apply = ix(
        identity_protocol::ID,
        identity_protocol::accounts::ApplyPayment {
            config: config_pda(),
        },
        identity_protocol::instruction::ApplyPayment {},
    );
    assert_anchor_error(
        send(&mut svm, std::slice::from_ref(&apply), &stranger, &[]),
        err(IdentityError::TimelockActive),
    );
    warp_by(&mut svm, PAYMENT_CONFIG_DELAY);
    send(&mut svm, &[apply], &stranger, &[]).expect("apply");
    let cfg: IdentityConfig = read_account(&svm, &config_pda());
    assert_eq!(
        (cfg.prices, cfg.treasury_token),
        ([USDC, 2 * USDC, 3 * USDC], new_treasury)
    );
}

/// Re-review C3/C5: only the attester that issued an id may refresh it (so it keeps the
/// right to revoke), and a revocation always reaches the identity, or works without one.
#[test]
fn only_the_issuer_refreshes_a_link_and_revocation_always_clears_the_level() {
    let mut e = env();
    let admin = e.admin.insecure_clone();
    let issuer = e.attester.insecure_clone();
    let alice = user_with_identity(&mut e, "alice");
    let bob = user_with_identity(&mut e, "bob");
    let year = now(&e.svm) + 365 * DAY;
    let id = [42u8; 32];
    link(&mut e, &alice, id, CLAIM_KYC, year).expect("issued");

    let lenient = funded_keypair(&mut e.svm, 5);
    let add = admin_ix(
        admin.pubkey(),
        identity_protocol::instruction::AddAttester {
            attester: lenient.pubkey(),
        },
    );
    send(&mut e.svm, &[add], &admin, &[]).expect("second attester");
    let takeover = link_ix(
        &alice.pubkey(),
        &lenient.pubkey(),
        id,
        CLAIM_KYC | CLAIM_BIOMETRIC | CLAIM_UNIQUE_HUMAN,
        year,
    );
    assert_anchor_error(
        send(&mut e.svm, &[takeover], &alice, &[&lenient]),
        err(IdentityError::AttesterMismatch),
    );
    link(&mut e, &alice, id, CLAIM_KYC | CLAIM_BIOMETRIC, year).expect("issuer refreshes");
    assert_eq!(effective(&e, &alice.pubkey()), LEVEL_FULL);

    let revoke = |identity: Pubkey| {
        ix(
            identity_protocol::ID,
            identity_protocol::accounts::RevokeAttestation {
                config: config_pda(),
                attestation_link: pda(&[ATTESTATION_SEED, id.as_ref()]),
                identity,
                rent_payer: alice.pubkey(),
                revoker: issuer.pubkey(),
            },
            identity_protocol::instruction::RevokeAttestation {},
        )
    };
    // The identity is derived from the link: it cannot be left out or swapped.
    let wrong = revoke(identity_pda(&bob.pubkey()));
    assert_anchor_error(
        send(&mut e.svm, &[wrong], &issuer, &[]),
        anchor_lang::error::ErrorCode::ConstraintSeeds as u32,
    );
    send(
        &mut e.svm,
        &[revoke(identity_pda(&alice.pubkey()))],
        &issuer,
        &[],
    )
    .expect("revoke");
    let a: Identity = read_account(&e.svm, &identity_pda(&alice.pubkey()));
    assert_eq!((a.level, a.level_expires_at), (LEVEL_NONE, 0));

    // An attestation outlives a closed identity; it can still be revoked.
    link(&mut e, &alice, id, CLAIM_KYC, year).expect("issued again");
    let close = ix(
        identity_protocol::ID,
        identity_protocol::accounts::CloseIdentity {
            identity: identity_pda(&alice.pubkey()),
            username_record: pda(&[USERNAME_SEED, b"alice"]),
            owner: alice.pubkey(),
        },
        identity_protocol::instruction::CloseIdentity {},
    );
    send(&mut e.svm, &[close], &alice, &[]).expect("close identity");
    warp_by(&mut e.svm, 1);
    send(
        &mut e.svm,
        &[revoke(identity_pda(&alice.pubkey()))],
        &issuer,
        &[],
    )
    .expect("revoke without an identity");
    assert!(e
        .svm
        .get_account(&pda(&[ATTESTATION_SEED, id.as_ref()]))
        .is_none_or(|x| x.lamports == 0));
}

/// Re-review C4: prepaid time worth more than the new tier's price is carried over at the
/// price ratio instead of being cut to one period.
#[test]
fn an_upgrade_never_drops_prepaid_time() {
    let mut e = env();
    let admin = e.admin.insecure_clone();
    let user = funded_keypair(&mut e.svm, 5);
    let usdc = e.usdc;
    let wallet = ata(&mut e.svm, &admin, &usdc, &user.pubkey());
    let mint_to = spl_token::instruction::mint_to(
        &spl_token::ID,
        &e.usdc,
        &wallet,
        &admin.pubkey(),
        &[],
        100 * USDC,
    )
    .unwrap();
    send(&mut e.svm, &[mint_to], &admin, &[]).expect("fund user");
    let sub = pda(&[SUBSCRIPTION_SEED, user.pubkey().as_ref()]);

    let start = now(&e.svm);
    for _ in 0..4 {
        let i = subscribe_ix(&e, &user.pubkey(), wallet, 1, 4 * USDC);
        send(&mut e.svm, &[i], &user, &[]).expect("tier 1");
        warp_by(&mut e.svm, 1);
    }
    assert_eq!(
        read_account::<Subscription>(&e.svm, &sub).expires_at,
        start + 4 * SUBSCRIPTION_DURATION
    );

    // 16 USDC of tier 1 left; tier 2 costs 12. Free upgrade, about 40 days of tier 2.
    let before = usdc_balance(&e.svm, &e.treasury);
    let i = subscribe_ix(&e, &user.pubkey(), wallet, 2, 0);
    send(&mut e.svm, &[i], &user, &[]).expect("free upgrade");
    assert_eq!(usdc_balance(&e.svm, &e.treasury), before);
    let s: Subscription = read_account(&e.svm, &sub);
    let t = now(&e.svm);
    let remaining = (start + 4 * SUBSCRIPTION_DURATION - t) as u128;
    let carried = (remaining * 4 / 12) as i64;
    assert_eq!((s.tier, s.expires_at), (2, t + carried));
    assert!(carried > SUBSCRIPTION_DURATION);
}
