use sonic_rs::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MiscEvent {
    pub time: String,
    pub hash: String,
    #[serde(flatten)]
    pub inner: MiscEventInner,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
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
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Delegation {
    pub user: String,
    pub validator: String,
    pub amount: f64,
    pub is_undelegate: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CWithdrawal {
    pub user: String,
    pub amount: f64,
    pub is_finalized: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidatorRewards {
    pub validator_to_reward: Vec<(String, f64)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Funding {
    pub coin: String,
    pub usdc: f64,
    pub szi: f64,
    #[serde(rename = "fundingRate")]
    pub funding_rate: f64,
    #[serde(rename = "nSamples")]
    pub n_samples: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LedgerUpdate {
    pub users: Vec<String>,
    pub delta: LedgerDelta,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Withdraw {
    pub usdc: f64,
    pub nonce: u64,
    pub fee: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Deposit {
    pub usdc: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VaultCreate {
    pub vault: String,
    pub usdc: f64,
    pub fee: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VaultDeposit {
    pub vault: String,
    pub user: String,
    pub usdc: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VaultWithdraw {
    pub vault: String,
    pub user: String,
    #[serde(rename = "requestedUsd")]
    pub requested_usd: f64,
    pub commission: f64,
    #[serde(rename = "closingCost")]
    pub closing_cost: f64,
    pub basis: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VaultDistribution {
    pub vault: String,
    pub usdc: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VaultLeaderCommission {
    pub vault: String,
    pub commission: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Liquidation {
    #[serde(rename = "liquidatedNtlPos")]
    pub liquidated_ntl_pos: f64,
    #[serde(rename = "accountValue")]
    pub account_value: f64,
    #[serde(rename = "leverageType")]
    pub leverage_type: String,
    #[serde(rename = "liquidatedPositions")]
    pub liquidated_positions: Vec<LiquidatedPosition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LiquidatedPosition {
    pub coin: String,
    pub szi: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InternalTransfer {
    pub usdc: f64,
    pub user: String,
    pub destination: String,
    pub fee: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubAccountTransfer {
    pub usdc: f64,
    pub user: String,
    pub destination: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpotTransfer {
    pub token: String,
    pub amount: f64,
    #[serde(rename = "usdcValue")]
    pub usdc_value: f64,
    pub user: String,
    pub destination: String,
    pub fee: f64,
    #[serde(rename = "nativeTokenFee")]
    pub native_token_fee: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpotGenesis {
    pub token: String,
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RewardsClaim {
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountActivationGas {
    pub amount: f64,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PerpDexClassTransfer {
    pub amount: f64,
    pub token: String,
    pub dex: String,
    #[serde(rename = "toPerp")]
    pub to_perp: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeployGasAuction {
    pub token: String,
    pub amount: f64,
}