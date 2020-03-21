extern "C"
{
	/// In practice, this function's varargs (`...`) are fixed as: `ptid: *mut pid_t, newtls: *mut c_void, ctid: *mut pid_t`.
	fn clone(func: *mut c_void, child_stack: *mut c_void, flags: c_int, arg: *mut c_void, ...) -> c_int;
}
