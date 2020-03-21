#[inline(always)]
fn top_of_child_stack_pointer(bottom_of_child_stack_pointer: NonNull<u8>, stack_size: usize) -> NonNull<u8>
{
	unsafe { NonNull::new_unchecked(((bottom_of_child_stack_pointer.as_ptr() as usize) + stack_size) as *mut u8) }
}
