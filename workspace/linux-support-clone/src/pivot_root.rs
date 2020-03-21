extern "C"
{
	fn pivot_root(new_root: *const c_char, put_old: *const c_char) -> c_int;
}
