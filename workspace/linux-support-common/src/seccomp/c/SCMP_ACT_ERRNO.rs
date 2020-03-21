
#[inline(always)]
pub const fn SCMP_ACT_ERRNO(x: u32) -> u32
{
	0x00050000 | (x & 0x0000FFFF)
}
