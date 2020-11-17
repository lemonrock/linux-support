// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub(crate) struct AuthorityNameNameServers
{
	pub(crate) authority_name: EfficientCaseFoldedName,
	
	/// These are for `authority_name`.
	pub(crate) name_servers: MultipleSortedRecords<NameServerName<EfficientCaseFoldedName>>,
}
