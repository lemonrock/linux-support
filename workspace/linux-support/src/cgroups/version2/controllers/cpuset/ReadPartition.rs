// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Read partition.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum ReadPartition
{
	/// Valid partition.
	Valid(Partition),

	/// Invalid partition root.
	///
	/// `root invalid`.
	RootInvalid,
}

impl FromBytes for ReadPartition
{
	type Error = ParseReadPartitionError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		use self::Partition::*;
		use self::ReadPartition::*;
		let variant = match bytes
		{
			b"root" => Valid(Root),
			b"member" => Valid(NonRootMember),
			b"root invalid" => RootInvalid,
			
			_ => return Err(ParseReadPartitionError::UnknownVariant(bytes.to_vec()))
		};
		Ok(variant)
	}
}
