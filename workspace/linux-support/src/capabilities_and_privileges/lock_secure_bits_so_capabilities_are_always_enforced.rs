// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Capability protection.
///
/// * Disables capability grants if setuid / setgid programs are run.
/// 	* Prevents this ever being revoked.
/// * Disables preserving some or all of the permitted, effective, and ambient capability sets when the real, effective or filesystem user identifiers of the process change to non-root values.
/// 	* Prevents this ever being revoked.
/// * Disables a thread ever being able to obtain the `SECBIT_KEEP_CAPS` bit.
/// * Disables the ability to add capabilities to the ambient set.
/// 	* Prevents this ever being revoked.
/// 	* Technically `SECBIT_NO_CAP_AMBIENT_RAISE` and `SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED` aren't needed/
#[inline(always)]
pub fn lock_secure_bits_so_capabilities_are_always_enforced(allow_keep_capabilities: bool) -> Result<(), io::Error>
{
	//noinspection SpellCheckingInspection
	const SECBIT_KEEP_CAPS_off: c_ulong = 0;

	let flags = if allow_keep_capabilities
	{
		SECBIT_KEEP_CAPS
	}
	else
	{
		SECBIT_KEEP_CAPS_off
	};

	let result = unsafe { prctl(PR_SET_SECUREBITS, SECBIT_NOROOT | SECBIT_NOROOT_LOCKED | SECBIT_NO_SETUID_FIXUP | SECBIT_NO_SETUID_FIXUP_LOCKED | flags | SECBIT_KEEP_CAPS_LOCKED | SECBIT_NO_CAP_AMBIENT_RAISE | SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED) };

	if likely!(result == 0)
	{
		Ok(())
	}
	else if likely!(result == -1)
	{
		Err(io::Error::last_os_error())
	}
	else
	{
		unreachable!("Unexpected result {} from prctl()", result)
	}
}
