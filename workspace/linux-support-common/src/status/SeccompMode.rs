/// `seccomp` mode.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum SeccompMode
{
	/// Off.
	Off = 0,

	/// Strict.
	Strict = 1,

	/// Filter.
	Filter = 2,
}