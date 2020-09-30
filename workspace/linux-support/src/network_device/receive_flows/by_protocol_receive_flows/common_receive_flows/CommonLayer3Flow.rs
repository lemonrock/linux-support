// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct CommonLayer3Flow<IPA: InternetProtocolAddress + Unmasked>
{
	pub source_address: MaskedData<IPA>,
	
	pub destination_address: MaskedData<IPA>,
	
	pub type_of_service_or_traffic_class: MaskedData<TrafficClassOrTypeOfService>,
}

impl<IPA: InternetProtocolAddress + Unmasked> CommonLayer3Flow<IPA>
{
	pub(super) fn new<CL3FS: CommonLayer3FlowSpecification<IPA::Underlying>>(data: &CL3FS, mask: &CL3FS) -> Self
	{
		Self
		{
			source_address: MaskedData::from_underlying_data_and_mask(data.source_address(), mask.source_address()),
			
			destination_address: MaskedData::from_underlying_data_and_mask(data.destination_address(), mask.destination_address()),
			
			type_of_service_or_traffic_class: MaskedData::from_underlying_data_and_mask(data.tos_or_tclass(), mask.tos_or_tclass()),
		}
	}
}
