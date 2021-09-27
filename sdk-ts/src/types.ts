import { Decimal } from "decimal.js";
import { PublicKey } from "@solana/web3.js";
import { assert } from "./utils";

export enum TokenType {
  BTC = "BTC",
  ETH = "ETH",
  USDT = "USDT",
  USDC = "USDC",
  UST = "UST",
  SOL = "SOL",
  USDT_USDC_SABER = "USDT_USDC_SABER",
  USDC_USDT_ORCA = "USDC_USDT_ORCA",
  UST_USDC_SABER = "UST_USDC_SABER"
}

export class AppConfig {
    programPubkey: PublicKey;
    adminPubkey: PublicKey;
    mints: { [key in TokenType]?: PublicKey };
    poolIds: { [key in TokenType]?: number };
    constructor(
      programPubkey: PublicKey,
      adminPubkey: PublicKey,
      mints: { [key in TokenType]?: PublicKey; },
      poolIds: { [key in TokenType]?: number; }
    ) {
      this.programPubkey = programPubkey;
      this.adminPubkey = adminPubkey;
      this.mints = mints;
      this.poolIds = poolIds;
      Object.keys(mints).map(tokType => { assert(tokType in poolIds); });
    }
    mintKeyStrToPoolId(mint_key_str: string): number {
      for(const [tokenType, pubkey] of Object.entries(this.mints)) {
        if(pubkey.toString() === mint_key_str) {
          const result = this.poolIds[tokenType as TokenType];
          assert(result !== undefined);
          return result;
        }
      }
      assert(false);
    }
}

export interface AssetPool {
  coin_name         : string;

  mint_key          : PublicKey;
  mint_decimal_mult : Decimal;
  pool_id           : number;

  deposit_amount    : Decimal;
  deposit_index     : Decimal;

  borrow_amount     : Decimal;
  borrow_index      : Decimal;

  reserve_factor    : Decimal;
  fee_amount        : Decimal;
  fee_withdrawn_amt : Decimal;
  fee_rate          : Decimal;

  last_update_time  : Decimal;

  spl_key           : PublicKey;
  atoken_mint_key   : PublicKey;
  price_key         : PublicKey;
  pyth_price_key    : PublicKey;

  serum_next_cl_id  : Decimal;
  ltv               : Decimal;
  safe_factor       : Decimal;
  flags             : number;

  base_rate         : Decimal;
  multiplier1       : Decimal;
  multiplier2       : Decimal;
  kink              : Decimal;
  borrow_rate       : Decimal;
  deposit_rate      : Decimal;

  reward_multiplier       : Decimal;
  reward_deposit_intra    : Decimal;

  reward_per_year         : Decimal;
  reward_per_year_deposit : Decimal;
  reward_per_year_borrow  : Decimal;
  reward_per_year_per_d   : Decimal;
  reward_per_year_per_b   : Decimal;

  reward_deposit_index    : Decimal;
  reward_borrow_index     : Decimal;

  deposit_cap       : Decimal;
}

export interface UserInfo {
  page_id         : number;
  num_assets      : number;
  user_asset_info : UserAssetInfo[];
  reward          : unknown;
  cap             : unknown;
  assist          : Assist;
}

export interface UserAssetInfo {
  pool_id               : number;
  use_as_collateral     : number;

  deposit_amount        : Decimal;
  deposit_interests     : Decimal;
  deposit_index         : Decimal;
  reward_deposit_amount : Decimal;
  reward_deposit_index  : Decimal;

  borrow_amount         : Decimal;
  borrow_interests      : Decimal;
  borrow_index          : Decimal;
  reward_borrow_amount  : Decimal;
  reward_borrow_index   : Decimal;
}

export interface Assist {
  assist_mode           : number;
  self_deleverage_factor: number;
  post_deleverage_factor: number;
  sell_sequence         : Uint8Array;
  buy_sequence          : Uint8Array;
  // skip tprice triggered actions
  num_actions           : number;
  num_executed          : number;
  //actions: unknown[];
}