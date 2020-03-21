/// This object encapsulates a piece of behaviour to run on exit to ensure clean-up.
///
/// Currently it justs ensures that PID files are deleted.
#[derive(Debug)]
pub struct DaemonizeCleanUpOnExit
{
	pid_file_path: PathBuf
}

impl DaemonizeCleanUpOnExit
{
	/// Cleans up.
	#[inline(always)]
	pub fn clean_up(self)
	{
		if let Err(_) = remove_file(&self.pid_file_path)
		{
			eprintln!("Could not remove PID file '{:?}'", self.pid_file_path)
		}
	}
}
