extern "C"
{
	fn mkdir(path: *const c_char, mode: mode_t) -> c_int;
}
