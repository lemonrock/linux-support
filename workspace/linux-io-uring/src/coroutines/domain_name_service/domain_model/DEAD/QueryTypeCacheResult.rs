// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
pub enum QueryTypeCacheResult<'cache, Record: Record>
{
	/// Query for the data.
	Nothing,
	
	/// Known to not exist.
	DoesNotExist(Rc<StartOfAuthority<EfficientCaseFoldedName>>),
	
	/// Known to exist.
	Exists(Exists<Record>),
}
