/// Transparent Huge Page (THP) regular memory choice.
///
/// Used for at least:-
///
/// * Ashmem
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TransparentHugePageRegularMemoryChoice
{
	/// Never allocate.
	Never,

	/// Always use.
	Always,

	/// Only for pages so specified by the `madvise()` (or `fadvise()`) syscall with the `MADV_HUGEPAGE` flag.
	Advise,
}

impl TransparentHugePageRegularMemoryChoice
{
	/// To value.
	#[inline(always)]
	pub fn to_value(self) -> &'static str
	{
		use self::TransparentHugePageRegularMemoryChoice::*;

		match self
		{
			Never => "never",
			Always => "always",
			Advise => "madvise",
		}
	}
}
