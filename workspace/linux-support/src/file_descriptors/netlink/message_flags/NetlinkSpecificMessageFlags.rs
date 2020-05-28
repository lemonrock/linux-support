// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Copy, Clone)]
pub(super) union NetlinkSpecificMessageFlags
{
	pub(super) acknowledge: NetlinkAcknowledgeMessageFlags,
	
	pub(super) delete_request: NetlinkDeleteRequestMessageFlags,
	
	pub(super) get_request: NetlinkGetRequestMessageFlags,
	
	pub(super) new_request: NetlinkNewRequestMessageFlags,
}
