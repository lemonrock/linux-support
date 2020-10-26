// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
struct LeastRecentlyUsedListPointer<Record: Sized>
{
	previous: *mut Self,
	next: *mut Self,
	
	key: CaseFoldedName,
	value: CacheEntry<Record>,
}

impl<Record: Sized> LeastRecentlyUsedListPointer<Record>
{
	#[inline(always)]
	unsafe fn new<'cache>(least_recently_used_list_head: &mut *mut Self, least_recently_used_list_tail: &mut *mut Self, key: CaseFoldedName, value: CacheEntry<Record>) -> (LeastRecentlyUsedListKeyReference<'cache>, NonNull<Self>)
	{
		let mut this = Box::new
		(
			Self
			{
				previous: null_mut(),
				next: null_mut(),
				
				key,
				value,
			}
		);
		
		this.attach_to_tail_of_least_recently_used_list(least_recently_used_list_tail);
		
		let key_reference = this.key_reference();
		let non_null = NonNull::new_unchecked(Box::into_raw(this));
		
		let head = *least_recently_used_list_head;
		if unlikely!(head.is_null())
		{
			*least_recently_used_list_head = non_null.as_ptr()
		}
		
		(key_reference, non_null)
	}
	
	#[inline(always)]
	unsafe fn move_to_tail(&mut self, least_recently_used_list_head: &mut *mut Self, least_recently_used_list_tail: &mut *mut Self)
	{
		self.detach(least_recently_used_list_head, least_recently_used_list_tail);
		self.attach_to_tail_of_least_recently_used_list(least_recently_used_list_tail)
	}
	
	#[inline(always)]
	unsafe fn key_reference<'cache>(&mut self) -> LeastRecentlyUsedListKeyReference<'cache>
	{
		LeastRecentlyUsedListKeyReference
		{
			key: unsafe { NonNull::new_unchecked(&mut self.key) }
		}
	}
	
	#[inline(always)]
	unsafe fn destroy(this: NonNull<Self>, least_recently_used_list_head: &mut *mut Self, least_recently_used_list_tail: &mut *mut Self, records_count: &mut usize)
	{
		let instance = this.as_mut();
		
		instance.detach(least_recently_used_list_head, least_recently_used_list_tail);
		let removed_records_count = instance.removed_records_count();
		*records_count = (*records_count) - removed_records_count;
		drop(Box::from_raw(this.as_ptr()));
	}
	
	#[inline(always)]
	fn removed_records_count(&self) -> usize
	{
		self.value.records_count()
	}
	
	#[inline(always)]
	unsafe fn detach(&mut self, least_recently_used_list_head: &mut *mut Self, least_recently_used_list_tail: &mut *mut Self)
	{
		if likely!(!self.previous.is_null())
		{
			let previous = &mut * self.previous;
			previous.next = self.next
		}
		else
		{
			debug_assert_eq!(self, *least_recently_used_list_head);
			*least_recently_used_list_head = self.next;
		}
		
		if likely!(!self.next.is_null())
		{
			let next = &mut * self.next;
			next.previous = self.previous
		}
		else
		{
			debug_assert_eq!(self, *least_recently_used_list_tail);
			*least_recently_used_list_tail = self.previous;
		}
		
		self.previous = null_mut();
		self.next = null_mut();
	}
	
	unsafe fn attach_to_tail_of_least_recently_used_list(&mut self, least_recently_used_list_tail: &mut *mut LeastRecentlyUsedListPointer)
	{
		debug_assert!(self.previous.is_null());
		debug_assert!(self.next.is_null());
		
		let tail = *least_recently_used_list_tail;
		debug_assert!((*tail.next).is_null());
		
		if likely!(!tail.is_null())
		{
			(*tail).next = self;
			self.previous = tail;
		}
		
		*least_recently_used_list_tail = self
	}
}
