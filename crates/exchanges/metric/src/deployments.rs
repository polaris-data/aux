use crate::{Address, Error};
/// Current v1 defaults from Metric's deployment page, rather than SDK address constants.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Addresses {
    pub factory: Address,
    pub router: Address,
    pub external_swap_executor: Address,
    pub quoter: Address,
    pub data_provider: Address,
    pub liquidity_adder: Address,
    pub compressed_oracle: Address,
    pub price_provider_factory: Address,
    pub anchored_provider_factory: Address,
}
impl Addresses {
    pub fn current(chain_id: u64) -> Result<Self, Error> {
        if ![1, 56, 4663, 8453, 42161, 999, 137, 43114, 143, 4326].contains(&chain_id) {
            return Err(Error::InvalidRequest(
                "no pinned deployment for chain; supply explicit contract addresses",
            ));
        }
        let a = |s: &str| s.parse::<Address>().expect("pinned deployment address");
        Ok(Self {
            factory: a("0x2a53833cc95548cf52c7b159110e22D3a9018f32"),
            router: a("0x02201E6f13c41506c7Bb635d7543F98826229aB7"),
            external_swap_executor: a("0x578C80DeC5D4Aa1F2879985e6e8d91Fc268e4AC2"),
            quoter: a("0x3B9ea79f482094323d06064E637a7F4528560fcb"),
            data_provider: a("0x83d66c5934b1525ee82e6E772c76c7acC778bBB7"),
            liquidity_adder: a("0x49A0dcb8dBD304B80aA475026cC558d4baE54382"),
            compressed_oracle: a("0xE844701934D9C6085B292e428F41b911cC0b3872"),
            price_provider_factory: a("0x050502eE4Da0eE2a9D44eE6046310F9f1532aCF2"),
            anchored_provider_factory: a("0xb0eAE19259500Ac4Ca59b9a5cb8cDB2Da8076F0A"),
        })
    }
}
