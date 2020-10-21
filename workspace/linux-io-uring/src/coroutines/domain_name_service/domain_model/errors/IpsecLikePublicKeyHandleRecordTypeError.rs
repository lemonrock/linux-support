// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// IPSec-like public key errors.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum IpsecLikePublicKeyHandleRecordTypeError
{
	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for no public key (value in tuple).
	HasWrongLengthForNoPublicKey(DataType, usize),
	
	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key (value in tuple).
	HasTooShortALengthForRSAPublicKey(DataType, usize),
	
	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key (value in tuple).
	HasTooShortALengthForRSAPublicKeyForAThreeByteExponentLength(DataType, usize),
	
	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key exponent.
	HasAZeroExponentForARSAPublicKey(DataType),
	
	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key exponent.
	HasTooShortALengthForARSAPublicKeyForExponentLength(DataType),
	
	/// Resource data for resource record type `IPSECKEY` or `HIP` has an incorrect length for a RSA public key modulus.
	HasAZeroModulusForARSAPublicKey(DataType),
	
	/// Resource data for resource record type `IPSECKEY` or `HIP` has an unusual length for a ECDSA public key (ie it does not seem to be for `P-256` or `P-384`).
	HasAUnusualLengthForAnECDSAPublicKey(DataType, usize),
}

impl Display for IpsecLikePublicKeyHandleRecordTypeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for IpsecLikePublicKeyHandleRecordTypeError
{
}
