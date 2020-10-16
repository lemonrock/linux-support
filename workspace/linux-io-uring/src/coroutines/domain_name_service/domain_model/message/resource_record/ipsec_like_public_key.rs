// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! ipsec_like_public_key
{
	($public_key_algorithm_type: ident, $resource_data: ident, $public_key_starts_at_offset: ident, $public_key_length: ident, $resource_data_end_pointer: ident, $dsa_callback: block, $unassigned_callback: block) =>
	{
		{
			use self::PublicKey::*;

			match $public_key_algorithm_type
			{
				0 => if unlikely!($public_key_length != 0)
				{
					return Err(ResourceDataForTypeIPSECKEYOrHIPHasWrongLengthForNoPublicKey($public_key_length))
				}
				else
				{
					None
				},

				1 =>
				{
					$unassigned_callback;
					return Ok($resource_data_end_pointer)
				}

				2 =>
				{
					if unlikely!($public_key_length == 0)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForRSAPublicKey($public_key_length))
					}

					let public_key_data = &$resource_data[$public_key_starts_at_offset .. $public_key_starts_at_offset + $public_key_length];

					const FirstByteSize: usize = 1;

					let first_byte_of_exponent_length = public_key_data.u8(0);
					let (exponent_and_modulus, exponent_length) = if first_byte_of_exponent_length == 0
					{
						const SecondAndThirdBytesSize: usize = 2;

						const SizeSize: usize = FirstByteSize + SecondAndThirdBytesSize;

						if unlikely!(public_key_data.len() < SizeSize)
						{
							return Err(ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForRSAPublicKeyForAThreeByteExponentLength($public_key_length))
						}

						(&public_key_data[SizeSize .. ], public_key_data.u16_as_usize(FirstByteSize))
					}
					else
					{
						(&public_key_data[FirstByteSize .. ], first_byte_of_exponent_length as usize)
					};

					if unlikely!(exponent_length == 0)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasAZeroExponentForARSAPublicKey)
					}

					if unlikely!(exponent_and_modulus.len() < exponent_length)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasTooShortALengthForARSAPublicKeyForExponentLength)
					}

					let modulus_length = exponent_and_modulus.len() - exponent_length;
					if unlikely!(modulus_length == 0)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasAZeroModulusForARSAPublicKey)
					}

					let rsa_public_key = RsaPublicKey
					{
						exponent: &exponent_and_modulus[ .. exponent_length],
						modulus: &exponent_and_modulus[exponent_length .. ],
					};

					Some(RSA(rsa_public_key))
				}

				3 =>
				{
					const BitsInAByte: usize = 8;

					if unlikely!($public_key_length != 256 / BitsInAByte || $public_key_length != 384 / BitsInAByte)
					{
						return Err(ResourceDataForTypeIPSECKEYOrHIPHasAUnusualLengthForAnECDSAPublicKey($public_key_length))
					}

					let public_key_data = &$resource_data[$public_key_starts_at_offset .. $public_key_starts_at_offset + $public_key_length];

					let ec_dsa_public_key = EcDsaPublicKey
					{
						Q: public_key_data,
					};

					Some(ECDSA(ec_dsa_public_key))
				}

				_ =>
				{
					$unassigned_callback;
					return Ok($resource_data_end_pointer)
				}
			}
		}
	}
}
