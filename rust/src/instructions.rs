use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use spl_token;

use crate::consts;

#[repr(packed)]
pub struct UpdateUserAssetConfigParam {
    pub use_as_collateral: u8,
    pub pool_id: u8,
}

#[repr(packed)]
pub struct DepositParam {
    pub amount: u64,
    pub pool_id: u8,
}

#[repr(packed)]
pub struct WithdrawParam {
    pub withdraw_all: u8,
    pub amount: u64,
    pub pool_id: u8,
}

#[repr(packed)]
pub struct BorrowParam {
    pub amount: u64,
    pub pool_id: u8,
}

#[repr(packed)]
pub struct RepayParam {
    pub repay_all: u8,
    pub amount: u64,
    pub pool_id: u8,
}

#[repr(packed)]
pub struct ExternLiquidateParam {
    // how much collateral liquidator wants to receive
    pub min_collateral_amount: u64,
    // how much debt liquidator wants to help repay
    pub repaid_borrow_amount: u64,
    pub collateral_pool_id: u8,
    pub borrowed_pool_id: u8,
}

#[repr(packed)]
pub struct SelfLiquidateParam {
    pub need_to_sell: u8,
    pub need_to_buy: u8,
    pub sell_collateral_amount: u64,
    pub buy_borrowed_amount: u64,
    pub collateral_pool_id: u8,
    pub borrowed_pool_id: u8,
}

#[repr(packed)]
pub struct MarginSwapParam {
    // about need_to_sell and need_to_buy
    // if we are swapping USDT/C to another asset, a single BID order will do so no need to sell
    // if we are swapping some asset to USDT/C, a single ASK order will do so no need to buy
    pub need_to_sell: u8,
    pub need_to_buy: u8,
    pub sell_amount: u64,
    pub min_buy_amount: u64,
    pub sell_pool_id: u8,
    pub buy_pool_id: u8,
}

#[inline(always)]
pub fn mut_cast<T>(data: &mut [u8] ) -> &mut T {
    assert!(data.len() >= std::mem::size_of::<T>());
    return unsafe{std::mem::transmute(data.as_ptr())};
}

pub fn deposit(
    user_wallet: &Pubkey,       // user wallet account, needs to be signer
    user_spl: &Pubkey,          // user's SPL token account
    amount: u64,
    pool_id: u8,
) -> Instruction {
    deposit_full(
        user_wallet,
        user_spl,
        &consts::get_user_info_k(user_wallet),
        &consts::get_asset_pool_k(pool_id),
        &consts::get_asset_pool_spl_k(&spl_token::ID, pool_id),
        &consts::get_pool_summaries_k(),
        &spl_token::ID,
        &consts::program::ID,
        amount,
        pool_id,
    )
}

pub fn deposit_full(
    user_wallet: &Pubkey,       // user wallet account, needs to be signer
    user_spl: &Pubkey,          // user's SPL token account
    user_info: &Pubkey,         // consts::get_user_info_k(user_wallet_key)
    asset_pool: &Pubkey,        // consts::get_asset_pool_k(pool_id)
    asset_pool_spl: &Pubkey,    // consts::get_asset_pool_spl_k(token_program, pool_id)
    pool_summaries: &Pubkey,    // consts::get_pool_summaries_k()
    token_program: &Pubkey,
    program_id: &Pubkey,        // consts::program::ID

    amount: u64,
    pool_id: u8,

) -> Instruction {
    let data_size = 1 + std::mem::size_of::<DepositParam>();
    let mut buffer = vec![0; data_size];

    buffer[0] = consts::CMD_DEPOSIT;
    let mut param = mut_cast::<DepositParam>(&mut buffer[1..]);
    param.amount = amount;
    param.pool_id = pool_id;

    Instruction{
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*user_wallet, true),
            AccountMeta::new(*user_spl, false),
            AccountMeta::new(*user_info, false),
            AccountMeta::new(*asset_pool, false),
            AccountMeta::new(*asset_pool_spl, false),
            AccountMeta::new(*pool_summaries, false),
            AccountMeta::new_readonly(*token_program, false),
        ],
        data: buffer,
    }
}

pub fn withdraw(
    user_wallet: &Pubkey,       // user wallet account, needs to be signer
    user_spl: &Pubkey,          // user's SPL token account
    withdraw_all: bool,
    amount: u64,
    pool_id: u8,
) -> Instruction {
    withdraw_full(
        user_wallet,
        user_spl,
        &consts::get_user_info_k(user_wallet),
        &consts::get_asset_pool_k(pool_id),
        &consts::get_asset_pool_spl_k(&spl_token::ID, pool_id),
        &consts::get_pool_summaries_k(),
        &consts::get_price_summaries_k(),
        &consts::get_base_pda(),
        &spl_token::ID,
        &consts::program::ID,
        withdraw_all,
        amount,
        pool_id,
    )
}

pub fn withdraw_full(
    user_wallet: &Pubkey,       // user wallet account, needs to be signer
    user_spl: &Pubkey,          // user's SPL token account
    user_info: &Pubkey,         // consts::get_user_info_k(user_wallet_key)
    asset_pool: &Pubkey,        // consts::get_asset_pool_k(pool_id)
    asset_pool_spl: &Pubkey,    // consts::get_asset_pool_spl_k(token_program, pool_id)
    pool_summaries: &Pubkey,    // consts::get_pool_summaries_k()
    price_summaries: &Pubkey,   // consts::get_price_summaries_k()
    base_pda: &Pubkey,          // consts::get_base_pda()
    token_program: &Pubkey,
    program_id: &Pubkey,        // consts::program::ID

    withdraw_all: bool,
    amount: u64,
    pool_id: u8,

) -> Instruction {
    let data_size = 1 + std::mem::size_of::<WithdrawParam>();
    let mut buffer = vec![0; data_size];

    buffer[0] = consts::CMD_WITHDRAW;
    let mut param = mut_cast::<WithdrawParam>(&mut buffer[1..]);
    param.withdraw_all = if withdraw_all {1} else {0};
    param.amount = amount;
    param.pool_id = pool_id;

    Instruction{
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*user_wallet, true),
            AccountMeta::new(*user_spl, false),
            AccountMeta::new(*user_info, false),
            AccountMeta::new(*asset_pool, false),
            AccountMeta::new(*asset_pool_spl, false),
            AccountMeta::new(*pool_summaries, false),
            AccountMeta::new_readonly(*price_summaries, false),
            AccountMeta::new_readonly(*base_pda, false),
            AccountMeta::new_readonly(*token_program, false),
        ],
        data: buffer,
    }
}

pub fn borrow(
    user_wallet: &Pubkey,       // user wallet account, needs to be signer
    user_spl: &Pubkey,          // user's SPL token account
    amount: u64,
    pool_id: u8,
) -> Instruction {
    borrow_full(
        user_wallet,
        user_spl,
        &consts::get_user_info_k(user_wallet),
        &consts::get_asset_pool_k(pool_id),
        &consts::get_asset_pool_spl_k(&spl_token::ID, pool_id),
        &consts::get_pool_summaries_k(),
        &consts::get_price_summaries_k(),
        &consts::get_base_pda(),
        &spl_token::ID,
        &consts::program::ID,
        amount,
        pool_id,
    )
}

pub fn borrow_full(
    user_wallet: &Pubkey,       // user wallet account, needs to be signer
    user_spl: &Pubkey,          // user's SPL token account
    user_info: &Pubkey,         // consts::get_user_info_k(user_wallet_key)
    asset_pool: &Pubkey,        // consts::get_asset_pool_k(pool_id)
    asset_pool_spl: &Pubkey,    // consts::get_asset_pool_spl_k(token_program, pool_id)
    pool_summaries: &Pubkey,    // consts::get_pool_summaries_k()
    price_summaries: &Pubkey,   // consts::get_price_summaries_k()
    base_pda: &Pubkey,          // consts::get_base_pda()
    token_program: &Pubkey,
    program_id: &Pubkey,        // consts::program::ID

    amount: u64,
    pool_id: u8,

) -> Instruction {
    let data_size = 1 + std::mem::size_of::<BorrowParam>();
    let mut buffer = vec![0; data_size];

    buffer[0] = consts::CMD_BORROW;
    let mut param = mut_cast::<BorrowParam>(&mut buffer[1..]);
    param.amount = amount;
    param.pool_id = pool_id;

    Instruction{
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*user_wallet, true),
            AccountMeta::new(*user_spl, false),
            AccountMeta::new(*user_info, false),
            AccountMeta::new(*asset_pool, false),
            AccountMeta::new(*asset_pool_spl, false),
            AccountMeta::new(*pool_summaries, false),
            AccountMeta::new_readonly(*price_summaries, false),
            AccountMeta::new_readonly(*base_pda, false),
            AccountMeta::new_readonly(*token_program, false),
        ],
        data: buffer,
    }
}

pub fn repay(
    user_wallet: &Pubkey,       // user wallet account, needs to be signer
    user_spl: &Pubkey,          // user's SPL token account
    repay_all: bool,
    amount: u64,
    pool_id: u8,
) -> Instruction {
    repay_full(
        user_wallet,
        user_spl,
        &consts::get_user_info_k(user_wallet),
        &consts::get_asset_pool_k(pool_id),
        &consts::get_asset_pool_spl_k(&spl_token::ID, pool_id),
        &consts::get_pool_summaries_k(),
        &spl_token::ID,
        &consts::program::ID,
        repay_all,
        amount,
        pool_id,
    )
}

pub fn repay_full(
    user_wallet: &Pubkey,       // user wallet account, needs to be signer
    user_spl: &Pubkey,          // user's SPL token account
    user_info: &Pubkey,         // consts::get_user_info_k(user_wallet_key)
    asset_pool: &Pubkey,        // consts::get_asset_pool_k(pool_id)
    asset_pool_spl: &Pubkey,    // consts::get_asset_pool_spl_k(token_program, pool_id)
    pool_summaries: &Pubkey,    // consts::get_pool_summaries_k()
    token_program: &Pubkey,
    program_id: &Pubkey,        // consts::program::ID

    repay_all: bool,
    amount: u64,
    pool_id: u8,

) -> Instruction {
    let data_size = 1 + std::mem::size_of::<RepayParam>();
    let mut buffer = vec![0; data_size];

    buffer[0] = consts::CMD_REPAY;
    let mut param = mut_cast::<RepayParam>(&mut buffer[1..]);
    param.repay_all = if repay_all {1} else {0};
    param.amount = amount;
    param.pool_id = pool_id;

    Instruction{
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*user_wallet, true),
            AccountMeta::new(*user_spl, false),
            AccountMeta::new(*user_info, false),
            AccountMeta::new(*asset_pool, false),
            AccountMeta::new(*asset_pool_spl, false),
            AccountMeta::new(*pool_summaries, false),
            AccountMeta::new_readonly(*token_program, false),
        ],
        data: buffer,
    }
}

pub fn extern_liquidate(
    liquidated_wallet: &Pubkey,         // wallet key for account to be liquidated
    liquidator_wallet: &Pubkey,         // wallet key for liquidator, signer
    liquidator_collateral_spl: &Pubkey, // liquidator's SPL token account for collateral asset
    liquidator_borrowed_spl: &Pubkey,   // liquidator's SPL token account for repaid asset
    repaid_borrow_amount: u64,
    min_collateral_amount: u64,
    borrowed_pool_id: u8,
    collateral_pool_id: u8,
) -> Instruction {
    extern_liquidate_full(
        liquidated_wallet,
        liquidator_wallet,

        &consts::get_user_info_k(liquidated_wallet),
        &consts::get_base_pda(),

        liquidator_collateral_spl,
        liquidator_borrowed_spl,

        &consts::get_asset_pool_k(collateral_pool_id),
        &consts::get_asset_pool_spl_k(&spl_token::ID, collateral_pool_id),

        &consts::get_asset_pool_k(borrowed_pool_id),
        &consts::get_asset_pool_spl_k(&spl_token::ID, borrowed_pool_id),

        &consts::get_pool_summaries_k(),
        &spl_token::ID,
        &consts::program::ID,

        repaid_borrow_amount,
        min_collateral_amount,
        borrowed_pool_id,
        collateral_pool_id,
    )
}

// liquidator will help repay "borrowed" and in exchange receive "collateral" asset at 1% discount
pub fn extern_liquidate_full(
    liquidated_wallet: &Pubkey,         // wallet key for account to be liquidated
    liquidator_wallet: &Pubkey,         // wallet key for liquidator, signer

    user_info: &Pubkey,                 // consts::get_user_info_k(liquidated_wallet)
    base_pda: &Pubkey,                  // consts::get_base_pda()

    liquidator_collateral_spl: &Pubkey, // liquidator's SPL token account for collateral asset
    liquidator_borrowed_spl: &Pubkey,   // liquidator's SPL token account for repaid asset

    collateral_asset_pool: &Pubkey,     // consts::get_asset_pool_k(collateral_pool_id)
    collateral_asset_pool_spl: &Pubkey, // consts::get_asset_pool_spl_k(token_program, collateral_pool_id)

    borrowed_asset_pool: &Pubkey,       // consts::get_asset_pool_k(borrowed_pool_id)
    borrowed_asset_pool_spl: &Pubkey,   // consts::get_asset_pool_spl_k(token_program, borrowed_pool_id)

    pool_summaries: &Pubkey,            // consts::get_pool_summaries_k()
    token_program: &Pubkey,
    program_id: &Pubkey,                // consts::program::ID

    repaid_borrow_amount: u64,
    min_collateral_amount: u64,
    borrowed_pool_id: u8,
    collateral_pool_id: u8,
) -> Instruction {
    let data_size = 1 + std::mem::size_of::<ExternLiquidateParam>();
    let mut buffer = vec![0; data_size];
    buffer[0] = consts::CMD_EXTERN_LIQUIDATE;

    let mut param = mut_cast::<ExternLiquidateParam>(&mut buffer[1..]);
    param.repaid_borrow_amount = repaid_borrow_amount;
    param.min_collateral_amount = min_collateral_amount;
    param.borrowed_pool_id = borrowed_pool_id;
    param.collateral_pool_id = collateral_pool_id;

    Instruction{
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*liquidated_wallet, false),
            AccountMeta::new_readonly(*liquidator_wallet, true),
            AccountMeta::new(*user_info, false),
            AccountMeta::new_readonly(*base_pda, false),
            AccountMeta::new(*liquidator_collateral_spl, false),
            AccountMeta::new(*liquidator_borrowed_spl, false),
            AccountMeta::new(*collateral_asset_pool, false),
            AccountMeta::new(*collateral_asset_pool_spl, false),
            AccountMeta::new(*borrowed_asset_pool, false),
            AccountMeta::new(*borrowed_asset_pool_spl, false),
            AccountMeta::new(*pool_summaries, false),
            AccountMeta::new_readonly(*token_program, false),
        ],
        data: buffer,
    }
}

// TODO: add self_liquidate
// TODO: add margin_swap