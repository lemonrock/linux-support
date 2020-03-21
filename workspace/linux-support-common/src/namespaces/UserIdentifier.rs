/// A user identifier.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
pub struct UserIdentifier(uid_t);

impl UserOrGroupIdentifier for UserIdentifier
{
	const Zero: Self = UserIdentifier(0);

	const FileName: &'static str = "uid_map";

	#[inline(always)]
	fn current() -> Self
	{
		Self(unsafe { getuid() })
	}

	#[inline(always)]
	fn get(self) -> u32
	{
		self.0
	}
}

impl From<uid_t> for UserIdentifier
{
	#[inline(always)]
	fn from(value: uid_t) -> Self
	{
		Self(value)
	}
}

impl Into<uid_t> for UserIdentifier
{
	#[inline(always)]
	fn into(self) -> uid_t
	{
		self.0
	}
}
