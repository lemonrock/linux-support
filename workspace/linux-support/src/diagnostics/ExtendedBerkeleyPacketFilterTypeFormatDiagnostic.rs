// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExtendedBerkeleyPacketFilterTypeFormatDiagnostic
{
	pub id: BpfTypeFormatIdentifier,
	
	pub data: Option<ByteBuf>,
}

impl ExtendedBerkeleyPacketFilterTypeFormatDiagnostic
{
	fn gather(file_descriptor: &BpfTypeFormatFileDescriptor) -> Result<Self, Errno>
	{
		file_descriptor.get_information().map(|information| Self
		{
			id: information.identifier(),
			
			data: information.data().map(|slice| slice.to_vec().into()),
		})
	}
}
