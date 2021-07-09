use solana_program::pubkey::Pubkey;

pub mod program {
    use solana_program::declare_id;
    // program_id
    declare_id!("HidHf4DzeZj6F7BL37WP6YnTuhh4c4DTsdSTmiFaDtSf");
}

pub mod base_pda {
    use solana_program::declare_id;
    declare_id!("JBSGCV1hPY3CTfpqDQqB4TzwnL9Mjv9ahrSGkpvnxSiM");
    pub const BUMP:u8 = 255;
}

pub mod price_pda {
    use solana_program::declare_id;
    declare_id!("BPLk2Nd5B9pggzD6i6upRqPFptLBCjQSwfKHjjLjFYNp");
    pub const BUMP:u8 = 254;
}

pub mod pool_summaries {
    use solana_program::declare_id;
    declare_id!("vmw4aLng87nsu7adSGvjzsdrN8BixFnSwtfttXx7N6T");
}

pub mod price_summaries {
    use solana_program::declare_id;
    declare_id!("G1cmF3D5PAEAjnwdMFbcGQbnBmWNH7t4hv8cpmfHzS2V");
}


// commands
pub const CMD_ADD_USER_AND_DEPOSIT: u8 = 0x10;
pub const CMD_DEPOSIT: u8 = 0x11;
pub const CMD_WITHDRAW: u8 = 0x12;
pub const CMD_BORROW: u8 = 0x13;
pub const CMD_REPAY: u8 = 0x14;
pub const CMD_EXTERN_LIQUIDATE: u8 = 0x15;
pub const CMD_SELF_LIQUIDATE: u8 = 0x16;
pub const CMD_UPDATE_USER_CONFIG: u8 = 0x17;
pub const CMD_MARGIN_SWAP: u8 = 0x18;
pub const CMD_UPDATE_USER_ASSET_CONFIG: u8 = 0x19;
pub const CMD_WITHDRAW_AND_REMOVE_USER: u8 = 0x1a;


// errors
pub const ERR_INCORRECT_BASE_PDA:u32 = 0x1000;
pub const ERR_INCORRECT_USER_PAGES_STATS:u32 = 0x1001;
pub const ERR_INCORRECT_USERS_PAGE:u32 = 0x1002;
pub const ERR_INCORRECT_USER_INFO:u32 = 0x1003;
pub const ERR_INCORRECT_ASSET_POOL:u32 = 0x1004;
pub const ERR_INCORRECT_ASSET_PRICE:u32 = 0x1005;
pub const ERR_INCORRECT_ASSET_POOL_SPL:u32 = 0x1006;
pub const ERR_INCORRECT_USER_ASSET_INFO:u32 = 0x1007;
pub const ERR_MISSING_ACTIVE_ACCOUNTS:u32 = 0x1008;
pub const ERR_INCORRECT_INTERMEDIATE_SPL:u32 = 0x1009;
pub const ERR_INCORRECT_SELL_MARKET:u32 = 0x100a;
pub const ERR_INCORRECT_BUY_MARKET:u32 = 0x100b;
pub const ERR_INCORRECT_SERUM_PROGRAM:u32 = 0x100c;
pub const ERR_INCORRECT_ADMIN:u32 = 0x100d;
pub const ERR_INCORRECT_INTERMEDIATE_SPL_OWNER:u32 = 0x100e;
pub const ERR_INCORRECT_POOL_LIST:u32 = 0x100f;
pub const ERR_INCORRECT_POOL_SUMMARIES:u32 = 0x1010;
pub const ERR_INCORRECT_PRICE_SUMMARIES:u32 = 0x1011;
pub const ERR_INCORRECT_PRICE_PDA:u32 = 0x1012;

// errors about data
pub const ERR_MISSING_PAGE_ID:u32 = 0x2000;
pub const ERR_PAGE_ID_TOO_LARGE:u32 = 0x2001;
pub const ERR_MISSING_AMOUNT:u32 = 0x2002;
pub const ERR_MISSING_MINT_SEED_STR:u32 = 0x2003;
pub const ERR_MISSING_ACTIVE_MINT_SEED_STR:u32 = 0x2004;
pub const ERR_WRONG_DATA_SIZE:u32 = 0x2005;


// internal logic error
pub const ERR_ACCOUNT_ALREADY_ADDED:u32 = 0x3000;
pub const ERR_NO_AVAILABLE_SLOTS:u32 = 0x3001;
pub const ERR_ACCOUNT_NOT_ADDED:u32 = 0x3002;
pub const ERR_WALLET_DID_NOT_SIGN:u32 = 0x3003;
pub const ERR_MAXIMUM_NUM_POOLS_REACHED:u32 = 0x3004;
pub const ERR_USER_HAS_NO_SUCH_ASSET:u32 = 0x3005;
pub const ERR_NEED_AT_LEAST_BUY_OR_SELL:u32 = 0x3006;
pub const ERR_INSUFFICIENT_FEES:u32 = 0x3007;

// user logic error
pub const ERR_DEPOSIT_LESS_THAN_MINIMUM:u32 = 0x4000;
pub const ERR_INSUFFICIENT_DEPOSIT:u32 = 0x4001;
pub const ERR_POOL_NO_FREE_FUND:u32 = 0x4002;
pub const ERR_PLEASE_WITHDRAW_ALL:u32 = 0x4003;
pub const ERR_INSUFFICIENT_BORROW_POWER:u32 = 0x4004;
pub const ERR_CANNOT_REPAY_MORE_THAN_DEBT:u32 = 0x4005;
pub const ERR_WITHDRAWL_BELOW_MIN_COLLATERAL_RATIO:u32 = 0x4006;
pub const ERR_LIQUIDATION_NOT_REACHED:u32 = 0x4007;
pub const ERR_LIQUIDATOR_ASKED_TOO_MUCH_COLLATERAL:u32 = 0x4008;
pub const ERR_ACCOUNT_NOT_ENOUGH_DEBT_FOR_LIQUIDATION:u32 = 0x4009;
pub const ERR_ACCOUNT_NOT_ENOUGH_COLLATERAL_FOR_LIQUIDATION:u32 = 0x400a;
pub const ERR_EXCEEDS_LIQUIDATION_LIMIT:u32 = 0x400b;
pub const ERR_SELF_LIQUIDATION_THRESHOLD_TOO_SMALL:u32 = 0x400c;
pub const ERR_POST_SELF_LIQUIDATION_TARGET_TOO_SMALL:u32 = 0x400d;
pub const ERR_POST_EXTERN_LIQUIDATION_TARGET_TOO_SMALL:u32 = 0x400e;
pub const ERR_SELF_LIQUIDATION_NOT_REACHED:u32 = 0x400f;
pub const ERR_SELF_LIQUIDATION_TARGET_EXCEEDED:u32 = 0x4010;
pub const ERR_SELF_LIQUIDATION_HIGH_SLIPPAGE:u32 = 0x4011;
pub const ERR_MAX_NUM_ASSETS_REACHED:u32 = 0x4012;
pub const ERR_SWAP_BOUGHT_LESS_THAN_MIN:u32 = 0x4013;
pub const ERR_ASSET_NOT_USED_AS_COLLATERAL:u32 = 0x4014;


pub const USER_INFO_SEED: &'static str = "UserInfo";
pub const USER_PAGES_STATS_SEED: &'static str = "UserPagesStats";



#[inline(always)]
pub fn bytes_to_str(key_bytes:&[u8]) -> &str {
    unsafe { std::mem::transmute(key_bytes) }
}

#[inline(always)]
pub fn pool_id_to_seed_str<'a>(pool_id: u8, buffer: &'a mut [u8;8]) -> &'a str {
    *buffer = *b"POOL____";
    buffer[6] = (pool_id / 16) + b'a';
    buffer[7] = (pool_id % 16) + b'a';
    bytes_to_str(buffer)
}

#[inline(always)]
pub fn get_base_pda() -> Pubkey {
    base_pda::ID
}

#[inline(always)]
pub fn get_pool_summaries_k() -> Pubkey  {
    pool_summaries::ID
}

#[inline(always)]
pub fn get_price_summaries_k() -> Pubkey  {
    price_summaries::ID
}

#[inline(always)]
pub fn get_user_pages_stats_k() -> Pubkey {
    Pubkey::create_with_seed(&base_pda::ID, USER_PAGES_STATS_SEED, &program::ID).unwrap()
}

#[inline(always)]
pub fn get_users_page_k(page_id: usize) -> Pubkey {
    let seed = format!("UsersPage_{}", page_id);
    Pubkey::create_with_seed(&base_pda::ID, &seed, &program::ID).unwrap()
}

#[inline(always)]
pub fn get_asset_pool_k(pool_id: u8) -> Pubkey {
    let mut mint_seed_buffer = [0 as u8; 8];
    let pool_seed_str = pool_id_to_seed_str(pool_id, &mut mint_seed_buffer);
    Pubkey::create_with_seed(&base_pda::ID, pool_seed_str, &program::ID).unwrap()
}

#[inline(always)]
pub fn get_asset_pool_spl_k(token_program_id: &Pubkey, pool_id: u8) -> Pubkey {
    let mut mint_seed_buffer = [0 as u8; 8];
    let pool_seed_str = pool_id_to_seed_str(pool_id, &mut mint_seed_buffer);
    Pubkey::create_with_seed(&base_pda::ID, pool_seed_str, token_program_id).unwrap()
}

#[inline(always)]
pub fn get_user_info_k(user_wallet_key: &Pubkey) -> Pubkey {
    Pubkey::create_with_seed(user_wallet_key, USER_INFO_SEED, &program::ID).unwrap()
}
