// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! block_or_character_device
{
	($name: ident) =>
	{
		/// A $name.
		#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		pub struct $name
		{
			/// Major.
			pub major: u32,

			/// Minor.
			pub minor: u32,
		}

		impl From<(u32, u32)> for $name
		{
			#[inline(always)]
			fn from(value: (u32, u32)) -> Self
			{
				Self
				{
					major: value.0,
					minor: value.1,
				}
			}
		}

		impl From<u32> for $name
		{
			#[inline(always)]
			fn from(dev: u32) -> Self
			{
				Self
				{
					major: Self::u32_major(dev),
					minor: Self::u32_minor(dev),
				}
			}
		}

		impl From<dev_t> for $name
		{
			#[inline(always)]
			fn from(value: dev_t) -> Self
			{
				Self
				{
					major: Self::dev_t_major(value),
					minor: Self::dev_t_minor(value),
				}
			}
		}

		impl Into<(u32, u32)> for $name
		{
			#[inline(always)]
			fn into(self) -> (u32, u32)
			{
				(self.major, self.minor)
			}
		}

		impl Into<dev_t> for $name
		{
			#[inline(always)]
			fn into(self) -> dev_t
			{
				Self::dev_t_makedev(self.major, self.minor)
			}
		}

		impl Into<u32> for $name
		{
			#[inline(always)]
			fn into(self) -> u32
			{
				Self::u32_makedev(self.major, self.minor)
			}
		}

		impl $name
		{
			#[allow(dead_code)]
			const ZeroZero: Self = Self
			{
				major: 0,
				minor: 0,
			};

			#[allow(dead_code)]
			#[inline(always)]
			pub(crate) fn is_not_zero_zero(self) -> bool
			{
				self != Self::ZeroZero
			}

			#[inline(always)]
			const fn u32_minor(dev: u32) -> u32
			{
				((dev & 0xFFF00000) >> 12) | (dev & 0xFF)
			}

			#[inline(always)]
			const fn u32_major(dev: u32) -> u32
			{
				(dev & 0xFFF00) >> 8
			}

			#[inline(always)]
			const fn u32_makedev(major: u32, minor: u32) -> u32
			{
			  (minor & 0xFF) | ((major & 0xFFF) << 8) | ((minor & 0xFFF00) << 12)
			}

			#[inline(always)]
			const fn dev_t_major(dev: dev_t) -> u32
			{
				let mut major = 0;
				major |= (dev & 0x00000000000FFF00) >> 8;
				major |= (dev & 0xFFFFF00000000000) >> 32;
				major as u32
			}

			#[inline(always)]
			const fn dev_t_minor(dev: dev_t) -> u32
			{
				let mut minor = 0;
				minor |= (dev & 0x00000000000000FF) >> 0;
				minor |= (dev & 0x00000FFFFFF00000) >> 12;
				minor as u32
			}

			#[inline(always)]
			const fn dev_t_makedev(major: u32, minor: u32) -> dev_t
			{
				let major = major as dev_t;
				let minor = minor as dev_t;
				let mut dev = 0;
				dev |= (major & 0x00000FFF) << 8;
				dev |= (major & 0xFFFFF000) << 32;
				dev |= (minor & 0x000000FF) << 0;
				dev |= (minor & 0xFFFFFF00) << 12;
				dev
			}
		}
	}
}
