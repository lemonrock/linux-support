
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct mntent
{
	pub mnt_fsname: *mut c_char,
	pub mnt_dir: *mut c_char,
	pub mnt_type: *mut c_char,
	pub mnt_opts: *mut c_char,
	pub mnt_freq: c_int,
	pub mnt_passno: c_int,
}

impl Default for mntent
{
	fn default() -> Self
	{
		unsafe { ::std::mem::zeroed() }
	}
}
