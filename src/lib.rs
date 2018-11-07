// Copyright (c) 2018 The rust-bitcoin developers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! Constants for various bitcoin-like cryptocurrencie networks.
//!
//! The data provided for each currency includes:
//! * the human readable part as authoritatively maintained in [SLIP-0173](https://github.com/satoshilabs/slips/blob/master/slip-0173.md)
//! * the network's magic bytes as defined in their respective git repository
//!
//! The data provided for bitcoin only for now includes (other currencies may panic):
//! * chain parameters
//!
//! Please check if all constants you want to use are actually implemented by this library to avoid
//! panics.
//!
//! PRs adding new networks for the existing currencies (e.g. regtest) and constants not yet
//! included are very welcome. Please provide credible sources for magic bytes etc. in comments
//! to make review easier.

#![deny(missing_docs)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]

#[cfg(feature = "serde-support")]
extern crate serde;

#[cfg(all(feature = "serde-support", test))]
extern crate serde_json;

use std::fmt;

/// Provides network constants for one or more possible p2p networks. This trait is intended to be
/// implemented for enums representing sub- or supersets of the networks included in `Networks`.
/// When taking network-enums as arguments for functions please try to implement these generically
/// for this trait, e.g.:
///
/// ```ignore
/// use NetworkConstants;
///
/// fn new_address(network: &NetworkConstants) -> String {
///     let bech32_prefix = network.hrp();
///     // more bech32 magic
///     unimplemented!()
/// }
///
/// fn network_from_address<N: NetworkConstants>(bech32_addr: &str) -> Option<N> {
///     // bech32 parsing magic
///     let hrp = "bc";
///     N::from_hrp(hrp).ok()
/// }
/// ```
///
/// If you feel the urge to do matching over an enum implementing `NetworkConstants` please
/// consider opening a PR instead if your problem/solution can be generalized.
pub trait NetworkConstants : Sized {
    /// Returns the Human-readable part for the given network
    fn hrp(&self) -> &'static str;

    /// Tries to find a network with maching hrp
    fn from_hrp(hrp: &str) -> Result<Self, Error>;

    /// Returns the network's magic bytes
    fn magic(&self) -> u32;

    /// Tries to find a network with matching magic bytes
    fn from_magic(magic: u32) -> Result<Self, Error>;

    /// Returns a string representation of the networks identity (a.k.a. name)
    fn name(&self) -> &'static str;

    /// Tries to find a network with a matching name
    fn from_name(name: &str) -> Result<Self, Error>;

    /// Describes the nature of the network (production/testing)
    fn network_type(&self) -> NetworkType;

    /// Returns parameters for the chain's consensus
    fn chain_params(&self) -> ChainParams<Self>;
}

/// Errors that can happen in the `from_` functions
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Error {
    /// Not network with the specified properties (e.g. matching name) could be found.
    UnknownNetwork,
}

/// Describes the nature of the network
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NetworkType {
    /// Public production network with real economic activity
    Mainnet,

    /// Public network without real economic activity, for testing purposes only
    Testnet,

    /// Private testnet, typically created and controlled by a single actor
    Regtest,
}

#[derive(Debug, Clone)]
/// Parameters that influence chain consensus.
pub struct ChainParams<N: NetworkConstants> {
    /// Network for which parameters are valid.
    pub network: N,

    /// Time when BIP16 becomes active.
    pub bip16_time: u32,

    /// Block height at which BIP34 becomes active.
    pub bip34_height: u32,

    /// Block height at which BIP65 becomes active.
    pub bip65_height: u32,

    /// Block height at which BIP66 becomes active.
    pub bip66_height: u32,

    /// Minimum blocks including miner confirmation of the total of 2016 blocks in a retargeting period,
    /// (nPowTargetTimespan / nPowTargetSpacing) which is also used for BIP9 deployments.
    /// Examples: 1916 for 95%, 1512 for testchains.
    pub rule_change_activation_threshold: u32,

    /// Number of blocks with the same set of rules.
    pub miner_confirmation_window: u32,

    /// Proof of work limit value. It cointans the lowest possible difficulty.
    pub pow_limit: [u64; 4],

    /// Expected amount of time to mine one block.
    pub pow_target_spacing: u64,

    /// Difficulty recalculation interval.
    pub pow_target_timespan: u64,

    /// Determines whether minimal difficulty may be used for blocks or not.
    pub allow_min_difficulty_blocks: bool,

    /// Determines whether retargeting is disabled for this network or not.
    pub no_pow_retargeting: bool,
}

/// The cryptocurrency to act on
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Networks {
    /// Bitcoin mainnet
    Bitcoin,
    /// Bitcoin testnet
    Testnet,
    /// Bitcoin regtest
    Regtest,
    /// Litecoin mainnet
    Litecoin,
    /// Litecoin testnet
    LitecoinTestnet,
    /// Vertcoin mainnet
    Vertcoin,
    /// Vertcoin testnet
    VertcoinTestnet,

    // if you add networks please also include them in the ALL_NETWORKS list
}

/// List of all networks included in this crate
//
// Given a list of all networks and any `network -> x` mapping, the inverse `x -> network` mapping
// can be calculated. This might not be the most performant solution, but surely the easiest to
// maintain. Since the match statements for the `network -> x` mapping have to be complete the
// compiler will complain if one of the mappings was forgotten after adding a new currency which is
// not possible for `x -> network` mappings.
pub const ALL_NETWORKS: &'static [Networks] = &[
    Networks::Bitcoin,
    Networks::Testnet,
    Networks::Regtest,
    Networks::Litecoin,
    Networks::LitecoinTestnet,
    Networks::Vertcoin,
    Networks::VertcoinTestnet
];

impl Networks {
    fn find_net_with_property<P>(predicate: P) -> Result<Networks, Error>
        where for<'r> P: FnMut(&'r &Networks) -> bool
    {
        match ALL_NETWORKS.iter().find::<P>(predicate).map(|n| *n) {
            Some(network) => Ok(network),
            None => Err(Error::UnknownNetwork)
        }
    }
}

impl NetworkConstants for Networks {
    fn hrp(&self) -> &'static str {
        match *self {
            Networks::Bitcoin => "bc",
            Networks::Testnet => "tb",
            Networks::Regtest => "bcrt",
            Networks::Litecoin => "ltc",
            Networks::LitecoinTestnet => "tltc",
            Networks::Vertcoin => "vtc",
            Networks::VertcoinTestnet => "tvtc",
        }
    }

    fn from_hrp(hrp: &str) -> Result<Networks, Error> {
        Networks::find_net_with_property(|n| n.hrp() == hrp)
    }

    fn magic(&self) -> u32 {
        match *self {
            // https://github.com/bitcoin/bitcoin/blob/ce650182f4d9847423202789856e6e5f499151f8/src/chainparams.cpp#L115
            Networks::Bitcoin => 0xD9B4BEF9,
            Networks::Testnet => 0x0709110B,
            Networks::Regtest => 0xDAB5BFFA,

            // https://github.com/litecoin-project/litecoin/blob/42dddc2f9ef5bdc8369a3c7552e70b974b9d1764/src/chainparams.cpp#L114
            Networks::Litecoin => 0xDBB6C0FB,
            Networks::LitecoinTestnet => 0xF1C8D2FD,

            // https://github.com/vertcoin-project/vertcoin-core/blob/3b3701e7a76d4fe6d2d7459b6f39a9570ca65b19/src/chainparams.cpp#L114
            Networks::Vertcoin => 0xDAB5BFFA,
            Networks::VertcoinTestnet => 0x74726576,
        }
    }

    /// Constructs a network from magic bytes if possible
    fn from_magic(magic: u32) -> Result<Networks, Error> {
        Networks::find_net_with_property(|n| n.magic() == magic)
    }

    fn name(&self) -> &'static str {
        match *self {
            Networks::Bitcoin => "bitcoin",
            Networks::Testnet => "testnet",
            Networks::Regtest => "regtest",
            Networks::Litecoin => "litecoin",
            Networks::LitecoinTestnet => "litecoin-testnet",
            Networks::Vertcoin => "vertcoin",
            Networks::VertcoinTestnet => "vertcoin-testnet",
        }
    }

    fn from_name(name: &str) -> Result<Self, Error> {
        Networks::find_net_with_property(|n| n.name() == name)
    }

    fn network_type(&self) -> NetworkType {
        match *self {
            Networks::Bitcoin => NetworkType::Mainnet,
            Networks::Testnet => NetworkType::Testnet,
            Networks::Regtest => NetworkType::Regtest,
            Networks::Litecoin => NetworkType::Mainnet,
            Networks::LitecoinTestnet => NetworkType::Testnet,
            Networks::Vertcoin => NetworkType::Mainnet,
            Networks::VertcoinTestnet => NetworkType::Testnet,
        }
    }

    fn chain_params(&self) -> ChainParams<Self> {
        match *self {
            Networks::Bitcoin => ChainParams {
                network: Networks::Bitcoin,
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
            },
            Networks::Testnet => ChainParams {
                network: Networks::Testnet,
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
            },
            Networks::Regtest => ChainParams {
                network: Networks::Regtest,
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
            },
            _ => unimplemented!(),
        }
    }
}

impl fmt::Debug for Networks {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name())
    }
}

#[cfg(feature = "serde-support")]
impl serde::Deserialize for Networks {
    #[inline]
    fn deserialize<D>(d: &mut D) -> Result<Networks, D::Error>
        where D: serde::Deserializer
    {
        struct Visitor;
        impl serde::de::Visitor for Visitor {
            type Value = Networks;

            fn visit_string<E>(&mut self, v: String) -> Result<Networks, E>
                where E: serde::de::Error
            {
                self.visit_str(&v)
            }

            fn visit_str<E>(&mut self, s: &str) -> Result<Networks, E>
                where E: serde::de::Error
            {
                match Networks::from_name(s) {
                    Ok(network) => Ok(network),
                    Err(Error::UnknownNetwork) => Err(serde::de::Error::syntax("Network")),
                }
            }
        }

        d.visit(Visitor)
    }
}

#[cfg(feature = "serde-support")]
impl serde::Serialize for Networks {
    fn serialize<S>(&self, s: &mut S) -> Result<(), S::Error>
        where S: ::serde::Serializer
    {
        s.visit_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use {Networks, Error, NetworkConstants};

    #[test]
    fn hrp_conversion() {
        assert_eq!(Networks::Bitcoin.hrp(), "bc");
        assert_eq!(Networks::from_hrp("tvtc"), Ok(Networks::VertcoinTestnet));
        assert_eq!(Networks::from_hrp("test"), Err(Error::UnknownNetwork));
    }

    #[test]
    fn magic_conversion() {
        assert_eq!(Networks::Bitcoin.magic(), 0xD9B4BEF9);
        assert_eq!(Networks::from_magic(0xD9B4BEF9), Ok(Networks::Bitcoin));
        assert_eq!(Networks::from_magic(0xABCDEF01), Err(Error::UnknownNetwork));
    }

    #[test]
    fn enum_name_conversion() {
        assert_eq!(Networks::Bitcoin.name(), "bitcoin".to_string());
        assert_eq!(Networks::from_name("testnet"), Ok(Networks::Testnet));
        assert_eq!(Networks::from_name("foobar"), Err(Error::UnknownNetwork));
    }

    #[cfg(feature = "serde-support")]
    #[test]
    fn test_serde() {
        let from = vec![Networks::Bitcoin, Networks::LitecoinTestnet, Networks::Vertcoin];
        let enc = ::serde_json::to_string(&from).unwrap();
        assert!(enc.contains("bitcoin"));
        assert!(enc.contains("litecoin-testnet"));
        assert!(enc.contains("vertcoin"));
        assert_eq!(::serde_json::from_str::<Vec<Networks>>(&enc).unwrap(), from);
    }
}