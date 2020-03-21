/// What caused the panic?
#[inline(always)]
pub fn panic_payload_to_cause(panic_payload: &(dyn Any + 'static + Send)) -> String
{
	if panic_payload.is::<String>()
	{
		panic_payload.downcast_ref::<String>().unwrap().to_string()
	}
	else if panic_payload.is::<&str>()
	{
		panic_payload.downcast_ref::<&str>().unwrap().to_string()
	}
	else
	{
		"(unknown cause)".to_string()
	}
}
