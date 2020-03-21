/// An inode
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Inode(u64);

impl From<u64> for Inode
{
	#[inline(always)]
	fn from(value: u64) -> Self
	{
		Self(value)
	}
}

impl Into<u64> for Inode
{
	#[inline(always)]
	fn into(self) -> u64
	{
		self.0
	}
}
