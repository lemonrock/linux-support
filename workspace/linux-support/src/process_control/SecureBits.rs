// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Secure bits.
	#[derive(Serialize, Deserialize)]
	pub struct SecureBits: u32
	{
		/// When set UID 0 has no special privileges.
		/// When unset, we support inheritance of root-permissions and suid-root executable under compatibility mode.
		/// We raise the effective and inheritable bitmasks *of the executable file* if the effective uid of the new process is 0.
		/// If the real uid is 0, we raise the effective (legacy) bit of the executable file.
		const NoRootOn = SECBIT_NOROOT;

		/// Lock for `NoRoot`.
		const NoRootLocked = SECBIT_NOROOT_LOCKED;

		/// When set, setuid to/from uid 0 does not trigger capability-"fixup".
		/// When unset, to provide compatiblility with old programs relying on set*uid to gain/lose privilege, transitions to/from uid 0 cause capabilities to be gained/lost.
		const NoSetUidFixUpOn = SECBIT_NO_SETUID_FIXUP;

		/// Lock for `NoSetUidFixUp`.
		const NoSetUidFixUpLocked = SECBIT_NO_SETUID_FIXUP_LOCKED;

		/// When set, a process can retain its capabilities even after transitioning to a non-root user (the set-uid fixup suppressed by bit 2).
		/// Bit-4 is cleared when a process calls exec(); setting both bit 4 and 5 will create a barrier through exec that `no exec()`'d child can use this feature again.
		const KeepCapabilitiesOn = SECBIT_KEEP_CAPS;

		/// Lock for `NoKeepCapabilities`.
		const KeepCapabilitiesLocked = SECBIT_KEEP_CAPS_LOCKED;

		/// When set, a process cannot add new capabilities to its ambient set.
		const NoAmbientCapabilityRaiseOn = SECBIT_NO_CAP_AMBIENT_RAISE;

		/// Lock for `NoAmbientCapabilityRaise`.
		const NoAmbientCapabilityRaiseLocked = SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED;
	}
}

impl SecureBits
{
	/// Current.
	#[inline(always)]
	pub fn current() -> Result<Self, Errno>
	{
		process_control_wrapper1
		(
			PR_GET_SECUREBITS,
			|non_negative_result| Ok(SecureBits::from_bits_truncate(non_negative_result as u32)),
			|error_number| Err(error_number)
		)
	}
	
	/// Capability protection.
	///
	/// * Disables capability grants if setuid / setgid programs are run.
	/// 	* Prevents this ever being revoked.
	/// * Disables preserving some or all of the permitted, effective, and ambient capability sets when the real, effective or filesystem user identifiers of the process change to non-root values.
	/// 	* Prevents this ever being revoked.
	/// * Disables a thread ever being able to obtain the `SECBIT_KEEP_CAPS` bit.
	/// * Disables the ability to add capabilities to the ambient set.
	/// 	* Prevents this ever being revoked.
	/// 	* Technically `SECBIT_NO_CAP_AMBIENT_RAISE` and `SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED` aren't needed.
	#[inline(always)]
	pub fn lock_secure_bits_so_capabilities_are_always_enforced(allow_keep_capabilities: bool) -> Result<(), Errno>
	{
		let secure_bits = Self::lock_down(allow_keep_capabilities);
		
		process_control_wrapper2
		(
			PR_SET_SECUREBITS,
			secure_bits.bits as usize,
			|non_negative_result| if likely!(non_negative_result == 0)
			{
				Ok(())
			}
			else
			{
				unreachable!("Positive result")
			},
			|error_number| Err(error_number)
		)
	}
	
	fn lock_down(allow_keep_capabilities: bool) -> Self
	{
		let mut secure_bits = Self::all();
		
		if !allow_keep_capabilities
		{
			secure_bits.remove(Self::KeepCapabilitiesOn)
		}
		
		secure_bits
	}
}
