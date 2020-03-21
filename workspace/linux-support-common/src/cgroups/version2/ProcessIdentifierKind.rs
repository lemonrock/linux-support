/// Process identifier kind.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ProcessIdentifierKind
{
	/// The process identifier for the current process.
	Current,

	/// A known process identifier,
	Known(NonZeroU32),
}

impl Into<u32> for ProcessIdentifierKind
{
	#[inline(always)]
	fn into(self) -> u32
	{
		use self::ProcessIdentifierKind::*;

		match self
		{
			Current => 0,
			Known(value) => value.get(),
		}
	}
}

impl From<u32> for ProcessIdentifierKind
{
	#[inline(always)]
	fn from(value: u32) -> Self
	{
		use self::ProcessIdentifierKind::*;

		if unlikely!(value == 0)
		{
			Current
		}
		else
		{
			Known(unsafe { NonZeroU32::new_unchecked(value) })
		}
	}
}
