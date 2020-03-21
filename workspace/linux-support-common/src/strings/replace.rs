/// replace.
#[inline(always)]
pub fn replace(extant: &[u8], from: u8, to: u8) -> Box<[u8]>
{
	let mut result = Vec::with_capacity(extant.len());

	for byte in extant.iter()
	{
		let byte = *byte;
		let byte_to_push = if unlikely!(byte == from)
		{
			to
		}
		else
		{
			byte
		};
		result.push(byte_to_push);
	}

	result.into_boxed_slice()
}
