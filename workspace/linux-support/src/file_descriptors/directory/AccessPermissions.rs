// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// File access permissions.
///
/// Can also be used to set `umask`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AccessPermissions(mode_t);

impl<'de> Deserialize<'de> for AccessPermissions
{
	#[inline(always)]
	fn deserialize<D:  Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
	{
		struct AccessPermissionsVisitor;

		impl<'de> Visitor<'de> for AccessPermissionsVisitor
		{
			type Value = AccessPermissions;

			#[inline(always)]
			fn expecting(&self, formatter: &mut Formatter) -> fmt::Result
			{
				formatter.write_str("A 32-bit unsigned integer")
			}

			#[inline(always)]
			fn visit_u8<E: de::Error>(self, v: u8) -> Result<Self::Value, E>
			{
				self.visit_u64(v as u64)
			}

			#[inline(always)]
			fn visit_u16<E: de::Error>(self, v: u16) -> Result<Self::Value, E>
			{
				self.visit_u64(v as u64)
			}

			#[inline(always)]
			fn visit_u32<E: de::Error>(self, v: u32) -> Result<Self::Value, E>
			{
				self.visit_u64(v as u64)
			}

			#[inline(always)]
			fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E>
			{
				if likely!(v & !0o7777 == 0)
				{
					Ok(AccessPermissions(v as mode_t))
				}
				else
				{
					Err(E::invalid_value(Unexpected::Unsigned(v as u64), &"Had bits other than 0o7777 set"))
				}
			}
		}

		deserializer.deserialize_u16(AccessPermissionsVisitor)
	}
}

impl Serialize for AccessPermissions
{
	#[inline(always)]
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
	{
		serializer.serialize_u32(self.0)
	}
}

impl Default for AccessPermissions
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(0o0022)
	}
}

impl From<u16> for AccessPermissions
{
	#[inline(always)]
	fn from(value: u16) -> Self
	{
		Self::from(value as u32)
	}
}

impl From<mode_t> for AccessPermissions
{
	#[inline(always)]
	fn from(value: mode_t) -> Self
	{
		Self(value)
	}
}

impl AccessPermissions
{
	/// Sets the process' umask and returns the process' previous umask.
	#[inline(always)]
	pub fn set_umask(self) -> Self
	{
		Self(unsafe { umask(DirectoryFileDescriptor::mask_mode(self)) })
	}

	/// Special permissions.
	#[inline(always)]
	pub fn special_permissions(self) -> SpecialPermissions
	{
		unsafe { transmute(((self.0 >> 12) & S_IRWXO) as u8) }
	}

	/// User accessibility.
	#[inline(always)]
	pub fn user(self) -> Accessibility
	{
		unsafe { transmute(((self.0 & S_IRWXU) >> 8) as u8) }
	}

	/// Group accessibility.
	#[inline(always)]
	pub fn group(self) -> Accessibility
	{
		unsafe { transmute(((self.0 & S_IRWXG) >> 4) as u8) }
	}

	/// All (other) accessibility.
	#[inline(always)]
	pub fn other(self) -> Accessibility
	{
		unsafe { transmute((self.0 & S_IRWXO) as u8) }
	}
}
