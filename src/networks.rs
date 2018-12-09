    //! Contains the networks supported by this crate.
    //!
    //! A network is represented by a zero sized struct which implements `NetworkConstants`. When used
    //! as function argument they should be passed as `&NetworkConstants` and if returned as result
    //! they should be boxed `Box<NetworkConstants>`. Since they are zero-sized `Box` will not allocate.
    //!
    //! The reason for this design decision is to avoid complete matching over some enum implementing
    //! `NetworkConstants` which would make any expansion of the set of supported networks a breaking
    //! change.

use ::{ChainParams, NetworkConstants, NetworkType};
use bitcoin_hashes::hex::FromHex;
use bitcoin_hashes::sha256d;

/// Represents the Bitcoin Mainnet
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Bitcoin {}

/// Represents the Bitcoin Testnet
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BitcoinTestnet {}

/// Represents the Bitcoin Regtest network
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BitcoinRegtest {}

impl Bitcoin {
    /// Create a new `Network` object representing Bitcoin
    pub fn new() -> Box<NetworkConstants> {
        Box::new(Bitcoin {})
    }
}

impl BitcoinTestnet {
    /// Create a new `Network` object representing BitcoinTestnet
    pub fn new() -> Box<NetworkConstants> {
        Box::new(BitcoinTestnet {})
    }
}

impl BitcoinRegtest {
    /// Create a new `Network` object representing BitcoinRegtest
    pub fn new() -> Box<NetworkConstants> {
        Box::new(BitcoinRegtest {})
    }
}

impl NetworkConstants for Bitcoin {
    fn hrp(&self) -> &'static str {
        "bc"
    }

    fn p2pk_prefix(&self) -> u8 {
        0
    }

    fn p2pkh_prefix(&self) -> u8 {
        0
    }

    fn p2sh_prefix(&self) -> u8 {
        5
    }

    fn xpub_prefix(&self) -> &'static [u8; 4] {
        static PREFIX: [u8; 4] = [0x04u8, 0x88, 0xB2, 0x1E];
        &PREFIX
    }

    fn xpriv_prefix(&self) -> &'static [u8; 4] {
        static PREFIX: [u8; 4] = [0x04, 0x88, 0xAD, 0xE4];
        &PREFIX
    }

    fn wif_prefix(&self) -> u8 {
        128
    }

    fn magic(&self) -> u32 {
        0xD9B4BEF9
    }

    fn name(&self) -> &'static str {
        "bitcoin"
    }

    fn network_type(&self) -> NetworkType {
        NetworkType::Mainnet
    }

    fn chain_params(&self) -> ChainParams {
        ChainParams {
            bip16_time: 1333238400,                 // Apr 1 2012
            bip34_height: 227931, // 000000000000024b89b42a942fe0d9fea3bb44ab7bd1b19115dd6a759c0808b8
            bip65_height: 388381, // 000000000000000004c2b624ed5d7756c508d90fd0da2c7c679febfa6c4735f0
            bip66_height: 363725, // 00000000000000000379eaa19dce8c9b722d46ae6a57c2f1a988119488b50931
            rule_change_activation_threshold: 1916, // 95%
            miner_confirmation_window: 2016,
            pow_limit: [
                0xffffffffffffffffu64,
                0xffffffffffffffffu64,
                0xffffffffffffffffu64,
                0x00000000ffffffffu64,
            ],
            pow_target_spacing: 10 * 60,            // 10 minutes.
            pow_target_timespan: 14 * 24 * 60 * 60, // 2 weeks.
            allow_min_difficulty_blocks: false,
            no_pow_retargeting: false,
        }
    }

    fn genesis_block(&self) -> sha256d::Hash {
        sha256d::Hash::from_hex(
            "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
        ).expect("static hex string, tested")
    }

    fn clone_boxed(&self) -> Box<NetworkConstants> {
        Self::new()
    }
}

impl NetworkConstants for BitcoinTestnet {
    fn hrp(&self) -> &'static str {
        "tb"
    }

    fn p2pk_prefix(&self) -> u8 {
        111
    }

    fn p2pkh_prefix(&self) -> u8 {
        111
    }

    fn p2sh_prefix(&self) -> u8 {
        196
    }

    fn xpub_prefix(&self) -> &'static [u8; 4] {
        static PREFIX: [u8; 4] = [0x04u8, 0x35, 0x87, 0xCF];
        &PREFIX
    }

    fn xpriv_prefix(&self) -> &'static [u8; 4] {
        static PREFIX: [u8; 4] = [0x04, 0x35, 0x83, 0x94];
        &PREFIX
    }

    fn wif_prefix(&self) -> u8 {
        239
    }

    fn magic(&self) -> u32 {
        0x0709110B
    }

    fn name(&self) -> &'static str {
        "bitcoin-testnet"
    }

    fn network_type(&self) -> NetworkType {
        NetworkType::Testnet
    }

    fn chain_params(&self) -> ChainParams {
        ChainParams {
            bip16_time: 1333238400,                 // Apr 1 2012
            bip34_height: 21111, // 0000000023b3a96d3484e5abb3755c413e7d41500f8e2a5c3f0dd01299cd8ef8
            bip65_height: 581885, // 00000000007f6655f22f98e72ed80d8b06dc761d5da09df0fa1dc4be4f861eb6
            bip66_height: 330776, // 000000002104c8c45e99a8853285a3b592602a3ccde2b832481da85e9e4ba182
            rule_change_activation_threshold: 1512, // 75%
            miner_confirmation_window: 2016,
            pow_limit: [
                0xffffffffffffffffu64,
                0xffffffffffffffffu64,
                0xffffffffffffffffu64,
                0x00000000ffffffffu64,
            ],
            pow_target_spacing: 10 * 60,            // 10 minutes.
            pow_target_timespan: 14 * 24 * 60 * 60, // 2 weeks.
            allow_min_difficulty_blocks: true,
            no_pow_retargeting: false,
        }
    }

    fn genesis_block(&self) -> sha256d::Hash {
        sha256d::Hash::from_hex(
            "000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943"
        ).expect("static hex string, tested")
    }

    fn clone_boxed(&self) -> Box<NetworkConstants> {
        Self::new()
    }
}

impl NetworkConstants for BitcoinRegtest {
    fn hrp(&self) -> &'static str {
        "bcrt"
    }

    fn p2pk_prefix(&self) -> u8 {
        111
    }

    fn p2pkh_prefix(&self) -> u8 {
        111
    }

    fn p2sh_prefix(&self) -> u8 {
        196
    }

    fn xpub_prefix(&self) -> &'static [u8; 4] {
        static PREFIX: [u8; 4] = [0x04u8, 0x35, 0x87, 0xCF];
        &PREFIX
    }

    fn xpriv_prefix(&self) -> &'static [u8; 4] {
        static PREFIX: [u8; 4] = [0x04, 0x35, 0x83, 0x94];
        &PREFIX
    }

    fn wif_prefix(&self) -> u8 {
        239
    }

    fn magic(&self) -> u32 {
        0xDAB5BFFA
    }

    fn name(&self) -> &'static str {
        "bitcoin-regtest"
    }

    fn network_type(&self) -> NetworkType {
        NetworkType::Regtest
    }

    fn chain_params(&self) -> ChainParams {
        ChainParams {
            bip16_time: 1333238400,  // Apr 1 2012
            bip34_height: 100000000, // not activated on regtest
            bip65_height: 1351,
            bip66_height: 1251,                    // used only in rpc tests
            rule_change_activation_threshold: 108, // 75%
            miner_confirmation_window: 144,
            pow_limit: [
                0xffffffffffffffffu64,
                0xffffffffffffffffu64,
                0xffffffffffffffffu64,
                0x7fffffffffffffffu64,
            ],
            pow_target_spacing: 10 * 60,            // 10 minutes.
            pow_target_timespan: 14 * 24 * 60 * 60, // 2 weeks.
            allow_min_difficulty_blocks: true,
            no_pow_retargeting: true,
        }
    }

    fn genesis_block(&self) -> sha256d::Hash {
        sha256d::Hash::from_hex(
            "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206"
        ).expect("static hex string, tested")
    }

    fn clone_boxed(&self) -> Box<NetworkConstants> {
        Self::new()
    }
}
