// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Key and Value BTF type identifiers for a map.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BpfTypeFormatKeyValueTypeIdentifiers
{
	/// Key type identifier.
	pub key_type_identifier: NonVoidBpfTypeFormatTypeIdentifier,
	
	/// Value type identifier.
	pub value_type_identifier: NonVoidBpfTypeFormatTypeIdentifier,
}
