/*
   Copyright 2019 Supercomputing Systems AG

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

*/

use sp_std::prelude::*;

#[cfg(feature = "std")]
use std::fmt;

use codec::{Compact, Decode, Encode, Error, Input};
//use indices::address::Address;
use sp_core::blake2_256;
use sp_core::H256;
use sp_runtime::{generic::Era, MultiSignature};

pub use sp_runtime::{AccountId32 as AccountId, MultiAddress};

pub type AccountIndex = u64;

pub type GenericAddress = sp_runtime::MultiAddress<AccountId, ()>;

/// Simple generic extra mirroring the SignedExtra currently used in extrinsics. Does not implement
/// the SignedExtension trait. It simply encodes to the same bytes as the real SignedExtra. The
/// Order is (CheckVersion, CheckGenesis, Check::Era, CheckNonce, CheckWeight, transactionPayment::ChargeTransactionPayment).
/// This can be locked up in the System module. Fields that are merely PhantomData are not encoded and are
/// therefore omitted here.
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Decode, Encode, Clone, Eq, PartialEq)]
// pub struct GenericExtra(Era, Compact<u32>, Compact<u128>);
pub struct GenericExtra(Era, Compact<u32>, Compact<u128>, [u8; 32]);

impl GenericExtra {
    pub fn new(era: Era, nonce: u32) -> GenericExtra {
        GenericExtra(era, Compact(nonce), Compact(0_u128), [0u8; 32])
    }
    pub fn new_account(era: Era, nonce: u32, account: [u8; 32]) -> GenericExtra {
        GenericExtra(era, Compact(nonce), Compact(0_u128), account)
    }
}

impl Default for GenericExtra {
    fn default() -> Self {
        Self::new(Era::Immortal, 0)
    }
}

/// additionalSigned fields of the respective SignedExtra fields.
/// Order is the same as declared in the extra.
pub type AdditionalSigned = (u32, u32, H256, H256, (), (), ());
// pub type AdditionalSigned = (u32, u32, H256, H256, (), (), (), ());

#[derive(Encode, Clone)]
pub struct SignedPayload<Call>((Call, GenericExtra, AdditionalSigned));

impl<Call> SignedPayload<Call>
where
    Call: Encode,
{
    pub fn from_raw(call: Call, extra: GenericExtra, additional_signed: AdditionalSigned) -> Self {
        Self((call, extra, additional_signed))
    }

    /// Get an encoded version of this payload.
    ///
    /// Payloads longer than 256 bytes are going to be `blake2_256`-hashed.
    pub fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
        self.0.using_encoded(|payload| {
            if payload.len() > 256 {
                f(&blake2_256(payload)[..])
            } else {
                f(payload)
            }
        })
    }
}

/// Mirrors the currently used Extrinsic format (V3) from substrate. Has less traits and methods though.
/// The SingedExtra used does not need to implement SingedExtension here.
#[derive(Clone, PartialEq)]
pub struct UncheckedExtrinsicV4<Call> {
    pub signature: Option<(GenericAddress, MultiSignature, GenericExtra)>,
    pub function: Call,
    // pub operator: Option<GenericAddress>,
}

impl<Call> UncheckedExtrinsicV4<Call>
where
    Call: Encode,
{
    pub fn new_signed(
        function: Call,
        signed: GenericAddress,
        signature: MultiSignature,
        extra: GenericExtra,
    ) -> Self {
        UncheckedExtrinsicV4 {
            signature: Some((signed, signature, extra)),
            function,
            // operator: None,
        }
    }
    
    // pub fn new_signed2(
    //     function: Call,
    //     signed: GenericAddress,
    //     signature: MultiSignature,
    //     extra: GenericExtra,
    //     operator: GenericAddress
    // ) -> Self {
    //     UncheckedExtrinsicV4 {
    //         signature: Some((signed, signature, extra)),
    //         function,
    //         operator: Some(operator),
    //     }
    // }

    #[cfg(feature = "std")]
    pub fn hex_encode(&self) -> String {
        let mut hex_str = hex::encode(self.encode());
        hex_str.insert_str(0, "0x");
        hex_str
    }
}

#[cfg(feature = "std")]
impl<Call> fmt::Debug for UncheckedExtrinsicV4<Call>
where
    Call: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hex_account = match self.signature.as_ref() {
            Some((_, _, extra)) => {
                let account = extra.3;
                let hex = hex::encode(account);
                hex
            },
            None => "".to_string()
        };
        write!(
            f,
            "UncheckedExtrinsic({:?}, {:?}), account:{}",
            self.signature.as_ref().map(|x| (&x.0, &x.2)),
            self.function,
            // self.operator,
            hex_account
        )
    }
}

const V4: u8 = 4;

impl<Call> Encode for UncheckedExtrinsicV4<Call>
where
    Call: Encode,
{
    fn encode(&self) -> Vec<u8> {
        encode_with_vec_prefix::<Self, _>(|v| {
            // the first 4 bytes is version
            // then if signed, the next is signature
            match self.signature.as_ref() {
                Some(s) => {
                    v.push(V4 | 0b1000_0000);
                    s.encode_to(v);
                }
                None => {
                    v.push(V4 & 0b0111_1111);
                }
            }
            // match self.operator.as_ref() {
            //     Some(s) => {
            //         v.push(V4 | 0b1000_0000);
            //     }
            //     None => {
            //         v.push(V4 & 0b0111_1111);
            //     }
            // }
            // match self.signature.as_ref() {
            //     Some(s) => {
            //         // v.push(V4 | 0b1000_0000);
            //         s.encode_to(v);
            //     }
            //     None => {
            //         // v.push(V4 & 0b0111_1111);
            //     }
            // }
            self.function.encode_to(v);
            // match self.operator.as_ref() {
            //     Some(s) => {
            //         // v.push(V4 | 0b1000_0000);
            //         s.encode_to(v);
            //     }
            //     None => {
            //         // v.push(V4 & 0b0111_1111);
            //     }
            // }
        })
    }
}

impl<Call> Decode for UncheckedExtrinsicV4<Call>
where
    Call: Decode + Encode,
{
    fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
        // This is a little more complicated than usual since the binary format must be compatible
        // with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
        // will be a prefix of vector length (we don't need
        // to use this).
        let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(input)?;

        let version = input.read_byte()?;

        let is_signed = version & 0b1000_0000 != 0;
        let version = version & 0b0111_1111;
        if version != V4 {
            return Err("Invalid transaction version".into());
        }

        // let is_operator = input.read_byte()?;
        // let is_operator = is_operator & 0b1000_0000 != 0;

        Ok(UncheckedExtrinsicV4 {
            signature: if is_signed {
                Some(Decode::decode(input)?)
            } else {
                None
            },
            function: Decode::decode(input)?,
            // operator: if is_operator {
            //     Some(Decode::decode(input)?)
            // } else {
            //     None
            // },
        })
    }
}

/// Same function as in primitives::generic. Needed to be copied as it is private there.
fn encode_with_vec_prefix<T: Encode, F: Fn(&mut Vec<u8>)>(encoder: F) -> Vec<u8> {
    let size = sp_std::mem::size_of::<T>();
    let reserve = match size {
        0..=0b0011_1111 => 1,
        0b0100_0000..=0b0011_1111_1111_1111 => 2,
        _ => 4,
    };
    let mut v = Vec::with_capacity(reserve + size);
    v.resize(reserve, 0);
    encoder(&mut v);

    // need to prefix with the total length to ensure it's binary compatible with
    // Vec<u8>.
    let mut length: Vec<()> = Vec::new();
    length.resize(v.len() - reserve, ());
    length.using_encoded(|s| {
        v.splice(0..reserve, s.iter().cloned());
    });

    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extrinsic::xt_primitives::{GenericAddress, GenericExtra};
    use sp_runtime::MultiSignature;

    #[test]
    fn encode_decode_roundtrip_works() {
        let xt = UncheckedExtrinsicV4::new_signed(
            vec![1, 1, 1],
            GenericAddress::default(),
            MultiSignature::default(),
            GenericExtra::default(),
        );
        println!("xt:{:?}", xt);
        let xt_enc = xt.encode();
        assert_eq!(xt, Decode::decode(&mut xt_enc.as_slice()).unwrap())
    }

    // #[test]
    // fn encode_decode_roundtrip_works2() {
    //     let xt = UncheckedExtrinsicV4::new_signed2(
    //         vec![1, 1, 1],
    //         GenericAddress::default(),
    //         MultiSignature::default(),
    //         GenericExtra::default(),
    //         GenericAddress::default()
    //     );
    //     println!("xt:{:?}", xt);

    //     let xt_enc = xt.encode();
    //     assert_eq!(xt, Decode::decode(&mut xt_enc.as_slice()).unwrap())
    // }
}
