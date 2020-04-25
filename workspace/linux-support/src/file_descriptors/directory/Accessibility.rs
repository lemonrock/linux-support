// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// File accessibility.
	#[repr(transparent)]
	pub struct Accessibility: u8
	{
		/// File exists (equivalent to `Self::empty()`).
		const Exists = F_OK as u8;

		/// Process can read.
		const Read = R_OK as u8;

		/// Process can write.
		const Write = W_OK as u8;

		/// Process can execute.
		const Execute = X_OK as u8;
	}
}

impl Accessibility
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_readable(self) -> bool
	{
		self.contains(Accessibility::Read)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_writable(self) -> bool
	{
		self.contains(Accessibility::Write)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_readable_and_writable(self) -> bool
	{
		self.contains(Accessibility::Read | Accessibility::Write)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_readable_and_executable(self) -> bool
	{
		self.contains(Accessibility::Read | Accessibility::Write)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_executable(self) -> bool
	{
		self.contains(Accessibility::Execute)
	}
}
