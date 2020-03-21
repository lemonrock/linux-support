/// `mount -t cgroup2 none /mnt/cgroup2`.
///
/// The only supported mount option is `nsdelegate`.
pub struct CgroupMountPoint
{
	path: PathBuf,
}

impl CgroupMountPoint
{
	/// To a Path.
	#[inline(always)]
	pub fn to_path(&self) -> &Path
	{
		&self.path
	}
}
