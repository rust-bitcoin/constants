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

extern crate bitcoin_hashes;

use bitcoin_hashes::sha256d;
use std::{fmt, ops};

pub mod networks;

/// Represents a bitcoin-like network for which it can provide encoding , network and consensus
/// constants.
pub struct Network(Box<NetworkConstants>);

impl Network {
    /// Create a net `Network` object from a trait object that provides network constants
    pub fn from_box(trait_obj: Box<NetworkConstants>) -> Network {
        Network(trait_obj)
    }

    /// Creates a `Network` object representing the bitcoin mainnet
    pub fn bitcoin() -> Network {
        Self::from_box(networks::Bitcoin::new())
    }

    /// Creates a `Network` object representing the bitcoin testnet
    pub fn bitcoin_testnet() -> Network {
        Self::from_box(networks::BitcoinTestnet::new())
    }

    /// Creates a `Network` object representing the bitcoin signet
    pub fn bitcoin_signet() -> Network {
        Self::from_box(networks::BitcoinSignet::new())
    }

    /// Creates a `Network` object representing the bitcoin regtest
    pub fn bitcoin_regtest() -> Network {
        Self::from_box(networks::BitcoinRegtest::new())
    }
}

impl Clone for Network {
    fn clone(&self) -> Self {
        Self::from_box(self.0.clone_boxed())
    }
}

impl ops::Deref for Network {
    type Target = Box<NetworkConstants>;

    fn deref(&self) -> &Box<NetworkConstants> {
        &self.0
    }
}

impl fmt::Debug for Network {
    fn fmt(&self, f: & mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Network{{name: '{}', ...}}", self.name())
    }
}

/// Provides network constants for a bitcoin-like crypto currency
pub trait NetworkConstants {
    /// Returns the Human-readable part for the given network
    fn hrp(&self) -> &'static str;

    /// Returns the prefix byte for legacy p2pk addresses
    fn p2pk_prefix(&self) -> u8;

    /// Returns the prefix byte for legacy p2pkh addresses
    fn p2pkh_prefix(&self) -> u8;

    /// Returns the prefix byte for legacy p2sh addresses
    fn p2sh_prefix(&self) -> u8;

    /// Returns the prefix bytes for encoding xpub keys
    fn xpub_prefix(&self) -> &'static [u8; 4];

    /// Returns the prefix bytes for encoding xpriv keys
    fn xpriv_prefix(&self) -> &'static [u8; 4];

    /// Returns the prefix byte for encoding private keys as WIF
    fn wif_prefix(&self) -> u8;

    /// Returns the network's magic bytes
    fn magic(&self) -> u32;

    /// Returns a string representation of the networks identity (a.k.a. name)
    fn name(&self) -> &'static str;

    /// Describes the nature of the network (production/testing)
    fn network_type(&self) -> NetworkType;

    /// Returns parameters for the chain's consensus
    fn chain_params(&self) -> ChainParams;

    /// Returns the hash of the genesis block
    fn genesis_block(&self) -> sha256d::Hash;

    /// Creates a boxed copy of `self`
    fn clone_boxed(&self) -> Box<NetworkConstants>;
}

/// Describes the nature of the network
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NetworkType {
    /// Public production network with real economic activity
    Mainnet,

    /// Public network without real economic activity, for testing purposes only
    Testnet,

    /// Testnet-like network with an added signature to block verification
    Signet,

    /// Private testnet, typically created and controlled by a single actor
    Regtest,
}

/// Parameters that influence chain consensus.
#[derive(Debug, Clone)]
pub struct ChainParams {
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

#[cfg(test)]
mod tests {
    use ::{Network};

    fn all_networks() -> Vec<Network> {
        vec![Network::bitcoin(), Network::bitcoin_testnet(), Network::bitcoin_signet(), Network::bitcoin_regtest()]
    }

    #[test]
    fn debug() {
        for n in all_networks() {
            assert!(format!("{:?}", n).contains(n.name()));
        }
    }

    #[test]
    fn dont_panic() {
        for n in all_networks() {
            let _ = n.hrp();
            let _ = n.p2pk_prefix();
            let _ = n.p2pkh_prefix();
            let _ = n.p2sh_prefix();
            let _ = n.xpub_prefix();
            let _ = n.xpriv_prefix();
            let _ = n.wif_prefix();
            let _ = n.magic();
            let _ = n.name();
            let _ = n.network_type();
            let _ = n.chain_params();
            let _ = n.genesis_block();
            let _ = n.clone_boxed();
        }
    }
}
