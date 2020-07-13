// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// BTF data to describe the map.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParsedBtfMapData<'data>
{
	/// Data.
	pub data: &'data ParsedBtfData,
	
	/// Identifiers.
	pub btf_key_value_type_identifiers: BtfKeyValueTypeIdentifiers,
}
