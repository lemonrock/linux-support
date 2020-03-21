/// Gets the program name.
#[inline(always)]
pub fn get_program_name() -> String
{
	unsafe { CStr::from_ptr(program_invocation_short_name).to_string_lossy().into_owned() }
}
