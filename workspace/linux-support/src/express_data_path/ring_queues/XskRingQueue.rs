// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Inspired by `xsk_ring_prod` in `libbpf`.
#[derive(Debug)]
pub(crate) struct XskRingQueue<XRQK: XskRingQueueKind, D: Descriptor>
{
	ring_queue_depth: RingQueueDepth,
	
	producer: NonNull<u32>,
	
	cached_producer: Cell<u32>,
	
	consumer: NonNull<u32>,
	
	cached_consumer: Cell<u32>,
	
	ring: *mut D,
	
	flags: NonNull<u32>,
	
	memory: MappedMemory,

	marker: PhantomData<(XRQK, D)>,
}

impl<XRQK: XskRingQueueKind, D: Descriptor> XskRingQueue<XRQK, D>
{
	/// Should be treated as initialized data.
	#[inline(always)]
	fn ring_entry(&self, index: u32) -> &D
	{
		unsafe { & * self.ring.add(self.array_index(index)) }
	}
	
	/// Should be treated as uninitialized data.
	#[inline(always)]
	fn ring_entry_mut(&self, index: u32) -> NonNull<D>
	{
		unsafe { NonNull::new_unchecked(self.ring.add(self.array_index(index))) }
	}
	
	#[inline(always)]
	fn from_ring_queue_offsets(express_data_path_socket_file_descriptor: &ExpressDataPathSocketFileDescriptor, ring_queue_offsets: &xdp_ring_offset, ring_queue_depth: RingQueueDepth, defaults: &DefaultPageSizeAndHugePageSizes, offset: u64) -> Self
	{
		let length = ring_queue_offsets.length_of_memory_to_map::<D>(ring_queue_depth);
		let memory = MappedMemory::from_file(express_data_path_socket_file_descriptor, offset, length, AddressHint::any(), Protection::ReadWrite, Sharing::Shared, None, true, false, defaults).expect("Could not memory map XDP fill ring queue");
		let producer = ring_queue_offsets.producer_pointer(&memory);
		let consumer = ring_queue_offsets.consumer_pointer(&memory);
		Self
		{
			ring_queue_depth,
			producer,
			cached_producer: Cell::new(Self::dereference_u32_pointer(producer)),
			consumer,
			cached_consumer: Cell::new
			(
				Self::dereference_u32_pointer(consumer) + if XRQK::UseRingQueueDepthForConsumer
				{
					ring_queue_depth as u32
				}
				else
				{
					0
				}
			),
			ring: ring_queue_offsets.ring_pointer(&memory),
			flags: ring_queue_offsets.flags_pointer(&memory),
			memory,
			marker: PhantomData,
		}
	}
	
	#[inline(always)]
	fn array_index(&self, index: u32) -> usize
	{
		(index & self.mask()) as usize
	}
	
	#[inline(always)]
	fn set_cached_producer(&self, value: u32)
	{
		self.cached_producer.set(value)
	}
	
	#[inline(always)]
	fn cached_producer(&self) -> u32
	{
		self.cached_producer.get()
	}
	
	#[inline(always)]
	fn set_cached_consumer(&self, value: u32)
	{
		self.cached_consumer.set(value)
	}
	
	#[inline(always)]
	fn cached_consumer(&self) -> u32
	{
		self.cached_consumer.get()
	}
	
	#[inline(always)]
	fn set_producer(&self, value: u32)
	{
		Self::set_u32_pointer(self.producer, value)
	}
	
	#[inline(always)]
	fn producer(&self) -> u32
	{
		Self::dereference_u32_pointer(self.producer)
	}
	
	#[inline(always)]
	fn set_consumer(&self, value: u32)
	{
		Self::set_u32_pointer(self.consumer, value)
	}
	
	#[inline(always)]
	fn consumer(&self) -> u32
	{
		Self::dereference_u32_pointer(self.consumer)
	}
	
	#[inline(always)]
	fn dereference_u32_pointer(u32_pointer: NonNull<u32>) -> u32
	{
		unsafe { *u32_pointer.as_ptr() }
	}
	
	#[inline(always)]
	fn set_u32_pointer(u32_pointer: NonNull<u32>, value: u32)
	{
		unsafe { *u32_pointer.as_ptr() = value }
	}
	
	#[inline(always)]
	fn mask(&self) -> u32
	{
		self.ring_queue_depth.mask()
	}
	
	#[inline(always)]
	fn flags(&self) -> u32
	{
		unsafe { *self.flags.as_ref() }
	}
	
	#[cfg(target_arch = "x86_64")]
	#[inline(always)]
	fn libbpf_smp_rmb()
	{
		Self::memory_barrier()
	}
	
	#[cfg(target_arch = "aarch64")]
	#[inline(always)]
	fn libbpf_smp_rmb()
	{
		unsafe
		{
			llvm_asm!
			(
				"
					dmb ishld
				"
				:
				:
				:
					"memory"
				:
					"volatile"
			);
		}
	}
	
	#[cfg(target_arch = "x86_64")]
	#[inline(always)]
	fn libbpf_smp_wmb()
	{
		Self::memory_barrier()
	}
	
	#[cfg(target_arch = "aarch64")]
	#[inline(always)]
	fn libbpf_smp_wmb()
	{
		unsafe
		{
			llvm_asm!
			(
				"
					dmb ishst
				"
				:
				:
				:
					"memory"
				:
					"volatile"
			);
		}
	}
	
	#[cfg(target_arch = "x86_64")]
	#[inline(always)]
	fn libbpf_smp_rwmb()
	{
		Self::memory_barrier()
	}
	
	#[cfg(target_arch = "aarch64")]
	#[inline(always)]
	fn libbpf_smp_rwmb()
	{
		Self::memory_barrier()()
	}
	
	#[cfg(target_arch = "x86_64")]
	#[inline(always)]
	fn memory_barrier()
	{
		// This can probably be converted to use `std::sync::atomic::compiler_fence()` if we can find documentation explicitly detailing which `Ordering` to use; it may be that `Ordering` does not ultimately matter.
		unsafe
		{
			llvm_asm!
			(
				""
				:
				:
				:
					"memory"
				:
					"volatile"
			);
		}
	}
	
	#[cfg(target_arch = "aarch64")]
	#[inline(always)]
	fn memory_barrier()
	{
		unsafe
		{
			llvm_asm!
			(
				"
					dmb ish
				"
				:
				:
				:
					"memory"
				:
					"volatile"
			);
		}
	}
}

impl<D: Descriptor> XskRingQueue<ProducerXskRingQueueKind, D>
{
	/// Based on `xsk_ring_prod__needs_wakeup()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// Call `poll()` or `sendto()` if this is true; see `kick_tx()` in Linux source `sample/bpf/xdpsock_user.c`.
	#[inline(always)]
	pub(super) fn needs_wake_up(&self) -> bool
	{
		self.flags() & XDP_RING_NEED_WAKEUP != 0
	}
	
	/// Based on `xsk_prod_nb_free()` in Linux source `tools/lib/bpf/xsk.h`.
	#[inline(always)]
	pub(super) fn number_free(&self, number: u32) -> u32
	{
		let cached_producer = self.cached_producer();
		
		let cached_consumer = self.cached_consumer();
		debug_assert!(cached_consumer >= cached_producer, "cached_consumer is less than cached_producer");
		let free_entries = cached_consumer - cached_producer;
		
		if free_entries >= number
		{
			return free_entries
		}
		
		// Refresh the local (cached) tail pointer
		//
		// `cached_consumer` is `ring_queue_depth` bigger than the real consumer pointer so that this addition can be avoided in the more frequently executed code that computes `free_entries` in the beginning of this function.
		// Without this optimization it whould have been `free_entries = cached_producer - cached_consumer + ring_queue_depth`.
		let cached_consumer = self.consumer() + self.ring_queue_depth as u32;
		self.set_cached_consumer(cached_consumer);
		
		debug_assert!(cached_consumer >= cached_producer, "cached_consumer is less than cached_producer");
		cached_consumer - cached_producer
	}
	
	/// Based on `xsk_ring_prod__reserve()` in Linux source `tools/lib/bpf/xsk.h`.
	#[inline(always)]
	pub(super) fn reserve(&self, number: NonZeroU32) -> Option<u32>
	{
		if self.number_free(number.get()) < number.get()
		{
			None
		}
		else
		{
			let cached_producer = self.cached_producer();
			let index = cached_producer;
			self.set_cached_producer(cached_producer + number);
			
			Some(index)
		}
	}
	
	/// Based on `xsk_ring_prod__submit()` in Linux source `tools/lib/bpf/xsk.h`.
	#[inline(always)]
	pub(super) fn submit(&self, number: NonZeroU32)
	{
		// Make sure everything has been written to the ring before indicating this to the kernel by writing the producer pointer.
		Self::libbpf_smp_wmb();
		
		self.set_producer(self.producer() + number.get());
	}
}

impl<D: Descriptor> XskRingQueue<ConsumerXskRingQueueKind, D>
{
	/// Based on `xsk_cons_nb_avail()` in Linux source `tools/lib/bpf/xsk.h`.
	#[inline(always)]
	fn number_available(&self, number: NonZeroU32) -> u32
	{
		let cached_producer = self.cached_producer();
		let cached_consumer = self.cached_consumer();
		
		debug_assert!(cached_producer >= cached_consumer, "cached_consumer is less than cached_producer");
		let mut entries = cached_producer - cached_consumer;
		
		if entries == 0
		{
			let new_cached_producer = self.producer();
			self.set_cached_producer(new_cached_producer);
			entries = new_cached_producer - cached_consumer;
		}
		
		min(number.get(), entries)
	}
	
	/// Based on `xsk_ring_cons__peek()` in Linux source `tools/lib/bpf/xsk.h`.
	///
	/// Returns `Some(non_zero_available, completion_queue_index)` if there is space in the queue.
	///
	/// Returns `None` if there is no space in the queue.
	#[inline(always)]
	pub(super) fn peek(&self, number: NonZeroU32) -> Option<(NonZeroU32, u32)>
	{
		let entries = self.number_available(number);
		
		if entries == 0
		{
			None
		}
		else
		{
			// Make sure we do not speculatively read the data before we have received the packet buffers from the ring.
			Self::libbpf_smp_rmb();
			
			let cached_consumer = self.cached_consumer();
			let index = cached_consumer;
			self.set_cached_consumer(cached_consumer + entries);
			Some((unsafe { NonZeroU32::new_unchecked(entries) }, index))
		}
	}
	
	/// Based on `xsk_ring_cons__release()` in Linux source `tools/lib/bpf/xsk.h`.
	#[inline(always)]
	pub(super) fn release(&self, number: NonZeroU32)
	{
		// Make sure data has been read before indicating we are done with the entries by updating the consumer pointer.
		Self::libbpf_smp_rwmb();
		
		self.set_consumer(self.consumer() + number.get())
	}
}
