// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Wake-on-LAN information.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WakeOnLanInformation
{
	/// Supported Wake-on-LAN settings.
	pub supported: HashSet<WakeOnLanWhen>,
	
	/// Active Wake-on-LAN settings.
	pub active: HashSet<WakeOnLanWhen>,
	
	/// SecureOn™ password.
	///
	/// *May be set* only if `WakeOnLanWhen::Magic` is set in `active`.
	/// Can be `None` even if `WakeOnLanWhen::Magic` is set in `active`.
	///
	/// A 6-byte Ethernet Media Access Control Address.
	pub active_secure_on_magic_password: Option<[u8; 6]>
}
