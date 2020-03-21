/// Zero-based argument number.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[allow(missing_docs)]
#[repr(u32)]
pub enum ZeroBasedArgumentNumber
{
	Zero = 0,
	One = 1,
	Two = 2,
	Three = 3,
	Four = 4,
	Five = 5,
}

impl Default for ZeroBasedArgumentNumber
{
	#[inline(always)]
	fn default() -> Self
	{
		ZeroBasedArgumentNumber::Zero
	}
}
