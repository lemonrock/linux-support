#[inline(always)]
pub const fn SCMP_ACT_TRACE(x: u32) -> u32
{
	0x7FF00000 | (x & 0x0000FFFF)
}
