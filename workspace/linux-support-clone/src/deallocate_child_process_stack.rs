#[inline(always)]
fn deallocate_child_process_stack(child_stack_allocator: &mut impl AllocRef, layout: Layout, bottom_of_child_stack_pointer: NonNull<u8>)
{
	unsafe { child_stack_allocator.dealloc(bottom_of_child_stack_pointer, layout) }
}
