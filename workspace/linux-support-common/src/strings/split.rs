/// split.
#[inline(always)]
pub fn split<'a>(slice: &'a [u8], predicate: u8) -> ::std::slice::Split<'a, u8, impl FnMut(&u8) -> bool>
{
	slice.split(move |value| *value == predicate)
}
