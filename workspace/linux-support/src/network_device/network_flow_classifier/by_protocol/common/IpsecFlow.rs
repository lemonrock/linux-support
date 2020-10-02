// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields, bound(deserialize = "IPA: DeserializeOwned"))]
pub struct IpsecFlow<IPA: InternetProtocolAddress + Unmasked>
{
	#[allow(missing_docs)]
	#[serde(flatten)] pub layer_3: CommonLayer3Flow<IPA>,
	
	#[allow(missing_docs)]
	pub security_parameter_index: MaskedData<SecurityParameterIndex>,
}

impl<IPA: InternetProtocolAddress + Unmasked> IpsecFlow<IPA>
{
	pub(crate) fn new<IAHFS: IpsecFlowSpecification<IPA::Underlying>>(data: &IAHFS, mask: &IAHFS) -> Self
	{
		Self
		{
			layer_3: CommonLayer3Flow::new(data, mask),
			
			security_parameter_index: MaskedData::from_underlying_data_and_mask(data.security_parameter_index(), mask.security_parameter_index()),
		}
	}
}
