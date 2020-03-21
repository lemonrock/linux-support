/// A group identifier.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct GroupIdentifier(gid_t);

impl UserOrGroupIdentifier for GroupIdentifier
{
	const Zero: Self = GroupIdentifier(0);

	const FileName: &'static str = "gid_map";

	#[inline(always)]
	fn current() -> Self
	{
		Self(unsafe { getgid() })
	}

	#[inline(always)]
	fn get(self) -> u32
	{
		self.0
	}
}

impl From<gid_t> for GroupIdentifier
{
	#[inline(always)]
	fn from(value: gid_t) -> Self
	{
		Self(value)
	}
}

impl Into<gid_t> for GroupIdentifier
{
	#[inline(always)]
	fn into(self) -> gid_t
	{
		self.0
	}
}
