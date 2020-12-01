// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct LeastRecentlyUsedList
{
	/// Most recently used.
	///
	/// Tail.
	tail: Option<NonNull<LeastRecentlyUsedNode>>,
	
	/// Least recently used.
	///
	/// Head.
	head: Option<NonNull<LeastRecentlyUsedNode>>,
}

impl LeastRecentlyUsedList
{
	#[inline(always)]
	fn pop_least_recently_used(&mut self) -> Option<NonNull<LeastRecentlyUsedNode>>
	{
		match self.head
		{
			None =>
			{
				debug_assert!(self.tail.is_none());
				
				None
			}
			
			Some(head_non_null) =>
			{
				let head = unsafe { head_non_null.as_mut() };
				
				debug_assert!(head.towards_head.is_none());
				
				match head.towards_tail
				{
					None =>
					{
						debug_assert_eq!(self.tail, self.head);
						
						self.head = None;
						self.tail = None;
					}
					
					Some(before_head_non_null) =>
					{
						let before_head = unsafe { before_head_non_null.as_mut() };
						
						debug_assert_ne!(self.tail, self.head);
						
						debug_assert_eq!(before_head.towards_head, Some(head));
						before_head.towards_head = None;
						
						self.head = Some(new_non_null(before_head));
						
						// Clean up node.
						head.towards_tail = None;
					}
				}
				
				Some(head_non_null)
			}
		}
	}
	
	#[inline(always)]
	fn push_most_recently_used(&mut self, node_non_null: NonNull<LeastRecentlyUsedNode>)
	{
		match self.tail
		{
			None =>
			{
				debug_assert!(self.head.is_none());
				
				let node = unsafe { node_non_null.as_mut() };
				node.towards_head = None;
				node.towards_tail = None;
				
				self.tail = Some(node_non_null);
				self.head = Some(node_non_null);
			}
			
			Some(tail_non_null) =>
			{
				let tail = unsafe { tail_non_null.as_mut() };
				
				debug_assert!(tail.towards_tail.is_none());
				tail.towards_tail = Some(node_non_null);
				
				let node = unsafe { node_non_null.as_mut() };
				node.towards_head = Some(tail_non_null);
				node.towards_tail = None;
				
				self.tail = Some(node_non_null);
			}
		}
	}
	
	#[inline(always)]
	fn push_most_recently_used_new_node(&mut self, name: FullyQualifiedDomainName, value: DomainCacheEntry) -> NonNull<LeastRecentlyUsedNode>
	{
		match self.tail
		{
			None =>
			{
				debug_assert!(self.head.is_none());
				
				let node_non_null = Self::new_node(name, value, None);
				
				self.tail = Some(node_non_null);
				self.head = Some(node_non_null);
				
				node_non_null
			}
			
			Some(tail_non_null) =>
			{
				let tail = unsafe { tail_non_null.as_mut() };
				
				debug_assert!(tail.towards_tail.is_none());
				
				let node_non_null = Self::new_node(name, value, Some(tail_non_null));
				
				tail.towards_tail = Some(node_non_null);
				
				self.tail = Some(node_non_null);
				
				node_non_null
			}
		}
	}
	
	#[inline(always)]
	fn remove_node(&mut self, node_non_null: NonNull<LeastRecentlyUsedNode>)
	{
		let node = unsafe { node_non_null.as_mut() };
		
		debug_assert!(self.tail.is_some());
		debug_assert!(self.head.is_some());
		
		if let Some(towards_tail_non_null) = node.towards_tail
		{
			let towards_tail = unsafe { towards_tail_non_null.as_mut() };
			
			towards_tail.towards_head = node.towards_head;
		}
		// node is tail.
		else
		{
			debug_assert_eq!(self.tail, Some(node));
			
			self.tail = node.towards_head
		}
		
		if let Some(towards_head_non_null) = node.towards_head
		{
			let towards_head = unsafe { towards_head_non_null.as_mut() };
			
			towards_head.towards_tail = node.towards_tail;
		}
		// node is head.
		else
		{
			debug_assert_eq!(self.head, Some(node));
			
			self.head = node.towards_tail
		}
	}
	
	#[inline(always)]
	fn new_node(name: FullyQualifiedDomainName, value: DomainCacheEntry, towards_head: Option<NonNull<LeastRecentlyUsedNode>>) -> NonNull<LeastRecentlyUsedNode>
	{
		let node = Box::new
		(
			LeastRecentlyUsedNode
			{
				towards_tail: None,
				towards_head,
				name,
				value,
			}
		);
		new_non_null(node.into_raw())
	}
	
	#[inline(always)]
	fn free_node(node_non_null: NonNull<LeastRecentlyUsedNode>)
	{
		unsafe { drop(Box::from_raw(node_non_null.as_ptr())) }
	}
}
