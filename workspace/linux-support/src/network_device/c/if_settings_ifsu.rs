// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union if_settings_ifsu
{
	pub(crate) raw_hdlc: *mut raw_hdlc_proto,
	pub(crate) cisco: *mut cisco_proto,
	pub(crate) fr: *mut fr_proto,
	pub(crate) fr_pvc: *mut fr_proto_pvc,
	pub(crate) fr_pvc_info: *mut fr_proto_pvc_info,
	pub(crate) x25: *mut x25_hdlc_proto,
	pub(crate) sync: *mut sync_serial_settings,
	pub(crate) te1: *mut te1_settings,
}

impl Default for if_settings_ifsu
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for if_settings_ifsu
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "if_settings_ifsu {{ {:?} }}", unsafe { self.raw_hdlc })
	}
}

impl PartialEq for if_settings_ifsu
{
	#[inline(always)]
	fn eq(&self, rhs: &Self) -> bool
	{
		unsafe { self.raw_hdlc == rhs.raw_hdlc }
	}
}

impl Eq for if_settings_ifsu
{
}

impl PartialOrd for if_settings_ifsu
{
	#[inline(always)]
	fn partial_cmp(&self, rhs: &Self) -> Option<Ordering>
	{
		unsafe { self.raw_hdlc.partial_cmp(&rhs.raw_hdlc) }
	}
}

impl Ord for if_settings_ifsu
{
	#[inline(always)]
	fn cmp(&self, rhs: &Self) -> Ordering
	{
		unsafe { self.raw_hdlc.cmp(&rhs.raw_hdlc) }
	}
}

impl Hash for if_settings_ifsu
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, hasher: &mut H)
	{
		unsafe { self.raw_hdlc.hash(hasher) }
	}
}
