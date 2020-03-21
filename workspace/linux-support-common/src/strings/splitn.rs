/// splitn.
#[inline(always)]
pub fn splitn<'a>(slice: &'a [u8], n: usize, predicate: u8) -> ::std::slice::SplitN<'a, u8, impl FnMut(&u8) -> bool>
{
	slice.splitn(n, move |value| *value == predicate)
}
