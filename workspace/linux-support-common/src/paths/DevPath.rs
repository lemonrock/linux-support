/// Represents `/dev`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct DevPath(PathBuf);

impl Default for DevPath
{
	#[inline(always)]
	fn default() -> Self
	{
		DevPath(PathBuf::from("/dev"))
	}
}

impl DevPath
{
	/// `/dev/null`.
	#[inline(always)]
	pub fn null(&self) -> PathBuf
	{
		self.file_path("null")
	}

	#[inline(always)]
	fn file_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.path();
		path.push(file_name);
		path
	}

	#[inline(always)]
	fn path(&self) -> PathBuf
	{
		self.0.to_owned()
	}
}
