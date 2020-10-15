// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// 3 bits in size (0 to 7 inclusive; all values defined but only values 1 to 6 inclusive have meaning).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(crate) enum AssociationMode
{
	Reserved = 0,
	
	SymmetricActive = 1,
	
	SymmetricPassive = 2,
	
	/// Used in packet from a client to a server when operating in unicast or manycasr mode.
	Client = 3,
	
	/// Used in packet from a server to a client when operating in unicast or manycast mode.
	Server = 4,
	
	/// Used in packets from a server broadcasting on a network.
	///
	/// This is never going to be secure in 2020.
	Broadcast = 5,
	
	/// Or broadcast client.
	ControlMessage = 6,
	
	ReservedForPrivateUse = 7,
}
