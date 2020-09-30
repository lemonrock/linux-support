// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CommonLayer4Flow<IPA: InternetProtocolAddress + Unmasked>
{
	#[serde(flatten)] pub layer_3: CommonLayer3Flow<IPA>,
	
	pub source_port: MaskedData<Layer4Port>,
	
	pub destination_port: MaskedData<Layer4Port>,
}

impl<IPA: InternetProtocolAddress + Unmasked> CommonLayer4Flow<IPA>
{
	pub(crate) fn new<CL4FS: CommonLayer4FlowSpecification<IPA::Underlying>>(data: &CL4FS, mask: &CL4FS) -> Self
	{
		Self
		{
			layer_3: CommonLayer3Flow::new(data, mask),
			
			source_port: MaskedData::from_underlying_data_and_mask(data.source_port(), mask.source_port()),
			
			destination_port: MaskedData::from_underlying_data_and_mask(data.destination_port(), mask.destination_port()),
		}
	}
}
