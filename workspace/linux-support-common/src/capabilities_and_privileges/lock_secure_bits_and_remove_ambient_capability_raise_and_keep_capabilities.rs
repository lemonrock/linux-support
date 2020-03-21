/// Capability protection.
#[inline(always)]
pub fn lock_secure_bits_and_remove_ambient_capability_raise_and_keep_capabilities()
{
	//noinspection SpellCheckingInspection
	const SECBIT_KEEP_CAPS_LOCKED_off: c_ulong = 0;

	unsafe
	{
		prctl(PR_SET_SECUREBITS, SECBIT_NOROOT | SECBIT_NOROOT_LOCKED | SECBIT_NO_SETUID_FIXUP | SECBIT_NO_SETUID_FIXUP_LOCKED | SECBIT_KEEP_CAPS_LOCKED_off | SECBIT_KEEP_CAPS_LOCKED | SECBIT_NO_CAP_AMBIENT_RAISE | SECBIT_NO_CAP_AMBIENT_RAISE_LOCKED);
	}
}
