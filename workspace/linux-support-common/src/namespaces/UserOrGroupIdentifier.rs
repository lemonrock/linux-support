/// An User Identifier or a Group Identifier.
pub trait UserOrGroupIdentifier: Into<u32> + Sized + Default + Copy
{
	/// Zero.
	const Zero: Self;

	/// File name.
	const FileName: &'static str;

	/// Current value.
	fn current() -> Self;

	/// Get inner value.
	fn get(self) -> u32;
}
