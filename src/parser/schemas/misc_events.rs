// src/parser/schemas/misc_events.rs

use sonic_rs::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MiscEvent {
    pub time: String,
    pub hash: String,
    pub inner: MiscEventInner,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MiscEventInner {
    CDeposit(CDeposit),
    Delegation(Delegation),
    CWithdrawal(CWithdrawal),
    ValidatorRewards(ValidatorRewards),
    Funding(Funding),
    LedgerUpdate(LedgerUpdate),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CDeposit {
    pub user: String,
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Delegation {
    pub user: String,
    pub validator: String,
    pub amount: String,
    pub is_undelegate: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CWithdrawal {
    pub user: String,
    pub amount: String,
    pub is_finalized: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidatorRewards {
    pub validator_to_reward: Vec<(String, String)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Funding {
    pub deltas: Vec<FundingDelta>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FundingDelta {
    pub user: String,
    pub coin: String,
    pub funding_amount: String,
    pub szi: String,
    pub funding_rate: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LedgerUpdate {
    pub users: Vec<String>,
    pub delta: LedgerDelta,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LedgerDelta {
    Withdraw(Withdraw),
    Deposit(Deposit),
    VaultCreate(VaultCreate),
    VaultDeposit(VaultDeposit),
    VaultWithdraw(VaultWithdraw),
    VaultDistribution(VaultDistribution),
    VaultLeaderCommission(VaultLeaderCommission),
    Liquidation(Liquidation),
    InternalTransfer(InternalTransfer),
    SubAccountTransfer(SubAccountTransfer),
    SpotTransfer(SpotTransfer),
    SpotGenesis(SpotGenesis),
    RewardsClaim(RewardsClaim),
    AccountActivationGas(AccountActivationGas),
    PerpDexClassTransfer(PerpDexClassTransfer),
    DeployGasAuction(DeployGasAuction),
    AccountClassTransfer(AccountClassTransfer),
    Send(Send),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Withdraw {
    pub usdc: String,
    pub nonce: u64,
    pub fee: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Deposit {
    pub usdc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VaultCreate {
    pub vault: String,
    pub usdc: String,
    pub fee: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VaultDeposit {
    pub vault: String,
    pub user: Option<String>,
    pub usdc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VaultWithdraw {
    pub vault: String,
    pub user: String,
    #[serde(rename = "requestedUsd")]
    pub requested_usd: String,
    pub commission: String,
    #[serde(rename = "closingCost")]
    pub closing_cost: String,
    pub basis: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VaultDistribution {
    pub vault: String,
    pub usdc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VaultLeaderCommission {
    pub user: String,
    pub usdc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Liquidation {
    #[serde(rename = "liquidatedNtlPos")]
    pub liquidated_ntl_pos: String,
    #[serde(rename = "accountValue")]
    pub account_value: String,
    #[serde(rename = "leverageType")]
    pub leverage_type: String,
    #[serde(rename = "liquidatedPositions")]
    pub liquidated_positions: Vec<LiquidatedPosition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LiquidatedPosition {
    pub coin: String,
    pub szi: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InternalTransfer {
    pub usdc: String,
    pub user: String,
    pub destination: String,
    pub fee: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubAccountTransfer {
    pub usdc: String,
    pub user: String,
    pub destination: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpotTransfer {
    pub token: String,
    pub amount: String,
    #[serde(rename = "usdcValue")]
    pub usdc_value: String,
    pub user: String,
    pub destination: String,
    pub fee: String,
    #[serde(rename = "nativeTokenFee")]
    pub native_token_fee: String,
    pub nonce: Option<u64>,
    #[serde(rename = "feeToken")]
    pub fee_token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpotGenesis {
    pub token: String,
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RewardsClaim {
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountActivationGas {
    pub amount: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PerpDexClassTransfer {
    pub amount: String,
    pub token: String,
    pub dex: String,
    #[serde(rename = "toPerp")]
    pub to_perp: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeployGasAuction {
    pub token: String,
    pub amount: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountClassTransfer {
    pub usdc: String,
    #[serde(rename = "toPerp")]
    pub to_perp: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Send {
    pub user: String,
    pub destination: String,
    #[serde(rename = "sourceDex")]
    pub source_dex: String,
    #[serde(rename = "destinationDex")]
    pub destination_dex: String,
    pub token: String,
    pub amount: String,
    #[serde(rename = "usdcValue")]
    pub usdc_value: String,
    pub fee: String,
    #[serde(rename = "nativeTokenFee")]
    pub native_token_fee: String,
    pub nonce: u64,
    #[serde(rename = "feeToken")]
    pub fee_token: String,
}