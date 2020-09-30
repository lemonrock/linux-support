// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(crate) trait UserFlowSpecification<IPA: Copy>: CommonLayer3FlowSpecification<IPA>
{
	/// First 4 bytes of transport (layer 4) header.
	fn first_four_bytes_of_layer4_header(&self) -> [u8; 4];
	
	/// Transport protocol number.
	///
	/// Masked value must always be `0`.
	fn transport_protocol_number(&self) -> u8;
}
