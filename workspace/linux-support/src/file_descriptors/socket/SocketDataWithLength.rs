// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Socket data with length.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketDataWithLength<SD: SocketData>
{
	/// Address.
	pub address: SD,
	
	/// Address length.
	pub address_length: usize,
}

impl<SD: SocketData> Display for SocketDataWithLength<SD>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.address.display_format(f, self.address_length)
	}
}

impl<SD: SocketData> Debug for SocketDataWithLength<SD>
{
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "SocketDataWithLength(")?;
		self.address.display_format(f, self.address_length)?;
		write!(f, ")")
	}
}
