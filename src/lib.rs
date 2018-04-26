// Copyright (c) 2017 Clark Moody
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
use std::str::FromStr;

/// The cryptocurrency to act on
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Network {
    /// Bitcoin mainnet
    Bitcoin,
    /// Bitcoin testnet
    Testnet,
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
pub const ALL_NETWORKS: &'static [Network] = &[
    Network::Bitcoin,
    Network::Testnet,
    Network::Litecoin,
    Network::LitecoinTestnet,
    Network::Vertcoin,
    Network::VertcoinTestnet
];

impl Network {
    /// Returns the Human-readable part for the given network
    pub fn hrp(&self) -> &'static str {
        match *self {
            Network::Bitcoin => "bc",
            Network::Testnet => "tb",
            Network::Litecoin => "ltc",
            Network::LitecoinTestnet => "tltc",
            Network::Vertcoin => "vtc",
            Network::VertcoinTestnet => "tvtc",
        }
    }

    /// Classify a Human-readable part as its cryptocurrency
    pub fn from_hrp(hrp: &str) -> Option<Network> {
        Network::find_net_with_property(|n| n.hrp() == hrp)
    }

    /// Returns the network's magic bytes
    pub fn magic(&self) -> u32 {
        match *self {
            // https://github.com/bitcoin/bitcoin/blob/ce650182f4d9847423202789856e6e5f499151f8/src/chainparams.cpp#L115
            Network::Bitcoin => 0xD9B4BEF9,
            Network::Testnet => 0x0709110B,

            // https://github.com/litecoin-project/litecoin/blob/42dddc2f9ef5bdc8369a3c7552e70b974b9d1764/src/chainparams.cpp#L114
            Network::Litecoin => 0xDBB6C0FB,
            Network::LitecoinTestnet => 0xF1C8D2FD,

            // https://github.com/vertcoin-project/vertcoin-core/blob/3b3701e7a76d4fe6d2d7459b6f39a9570ca65b19/src/chainparams.cpp#L114
            Network::Vertcoin => 0xDAB5BFFA,
            Network::VertcoinTestnet => 0x74726576,
        }
    }

    /// Constructs a network from magic bytes if possible
    pub fn from_magic(magic: u32) -> Option<Network> {
        Network::find_net_with_property(|n| n.magic() == magic)
    }

    fn find_net_with_property<P>(predicate: P) -> Option<Network>
        where for<'r> P: FnMut(&'r &Network) -> bool
    {
        ALL_NETWORKS.iter().find::<P>(predicate).map(|n| *n)
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let name = match *self {
            Network::Bitcoin => "bitcoin",
            Network::Testnet => "testnet", // only 'testnet' for compatibility reasons
            Network::Litecoin => "litecoin",
            Network::LitecoinTestnet => "litecoin-testnet",
            Network::Vertcoin => "vertcoin",
            Network::VertcoinTestnet => "vertcoin-testnet",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Debug for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

impl FromStr for Network {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Network::find_net_with_property(|n| &n.to_string() == s) {
            Some(network) => Ok(network),
            None => Err(())
        }
    }
}

#[cfg(feature = "serde-support")]
impl serde::Deserialize for Network {
    #[inline]
    fn deserialize<D>(d: &mut D) -> Result<Network, D::Error>
        where D: serde::Deserializer
    {
        struct Visitor;
        impl serde::de::Visitor for Visitor {
            type Value = Network;

            fn visit_string<E>(&mut self, v: String) -> Result<Network, E>
                where E: serde::de::Error
            {
                self.visit_str(&v)
            }

            fn visit_str<E>(&mut self, s: &str) -> Result<Network, E>
                where E: serde::de::Error
            {
                match s.parse::<Network>() {
                    Ok(network) => Ok(network),
                    Err(()) => Err(serde::de::Error::syntax("Network"))
                }
            }
        }

        d.visit(Visitor)
    }
}

#[cfg(feature = "serde-support")]
impl serde::Serialize for Network {
    fn serialize<S>(&self, s: &mut S) -> Result<(), S::Error>
        where S: ::serde::Serializer
    {
        s.visit_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use Network;

    #[test]
    fn hrp_conversion() {
        assert_eq!(Network::Bitcoin.hrp(), "bc");
        assert_eq!(Network::from_hrp("tvtc"), Some(Network::VertcoinTestnet));
        assert_eq!(Network::from_hrp("test"), None);
    }

    #[test]
    fn magic_conversion() {
        assert_eq!(Network::Bitcoin.magic(), 0xD9B4BEF9);
        assert_eq!(Network::from_magic(0xD9B4BEF9), Some(Network::Bitcoin));
        assert_eq!(Network::from_magic(0xABCDEF01), None);
    }

    #[test]
    fn enum_name_conversion() {
        assert_eq!(Network::Bitcoin.to_string(), "bitcoin".to_string());
        assert_eq!("testnet".parse(), Ok(Network::Testnet));
        assert_eq!("foobar".parse::<Network>(), Err(()));
    }

    #[cfg(feature = "serde-support")]
    #[test]
    fn test_serde() {
        let from = vec![Network::Bitcoin, Network::LitecoinTestnet, Network::Vertcoin];
        let enc = ::serde_json::to_string(&from).unwrap();
        assert!(enc.contains("bitcoin"));
        assert!(enc.contains("litecoin-testnet"));
        assert!(enc.contains("vertcoin"));
        assert_eq!(::serde_json::from_str::<Vec<Network>>(&enc).unwrap(), from);
    }
}