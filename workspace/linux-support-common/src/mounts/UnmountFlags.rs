bitflags!
{
	/// Unmount flags.
	#[allow(missing_docs)]
	#[derive(Deserialize, Serialize)]
	pub struct UnmountFlags: i32
	{
		/// Force.
		const Force = MNT_FORCE;

		/// Detach.
		const Detach = MNT_DETACH;

		/// Expire.
		const Expire = MNT_EXPIRE;

		// Not in libc crate
		// const NoFollow = UMOUNT_NOFOLLOW,
	}
}
