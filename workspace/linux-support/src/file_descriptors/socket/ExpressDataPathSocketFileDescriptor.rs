// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents an eXpress Data Path (XDP) socket instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ExpressDataPathSocketFileDescriptor(RawFd);

impl Drop for ExpressDataPathSocketFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.as_raw_fd().close()
	}
}

impl AsRawFd for ExpressDataPathSocketFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for ExpressDataPathSocketFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.as_raw_fd()
	}
}

impl FromRawFd for ExpressDataPathSocketFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for ExpressDataPathSocketFileDescriptor
{
}

impl MemoryMappableFileDescriptor for ExpressDataPathSocketFileDescriptor
{
}

impl ExpressDataPathSocketFileDescriptor
{
	/// New.
	#[inline(always)]
	pub(crate) fn new() -> Result<Self, CreationError>
	{
		let blocking = Blocking::default();
		
		new_socket(AF_XDP, SOCK_RAW, 0, blocking.is_non_blocking()).map(|file_descriptor| Self(file_descriptor))
	}
	
	/// Statistics.
	///
	/// Makes a syscall.
	#[inline(always)]
	pub(crate) fn statistics(&self) -> xdp_statistics
	{
		self.get_xdp_socket_option(XDP_STATISTICS, false)
	}
	
	/// Options.
	///
	/// Makes a syscall.
	#[inline(always)]
	pub(crate) fn options(&self) -> xdp_options
	{
		self.get_xdp_socket_option(XDP_OPTIONS, false)
	}
	
	#[inline(always)]
	pub(crate) fn register_user_space_memory(&self, configuration: &xdp_umem_reg, fill_ring_queue_depth: RingQueueDepth, completion_ring_queue_depth: RingQueueDepth)
	{
		self.set_xdp_socket_option(XDP_UMEM_REG, configuration);
		self.set_xdp_socket_option(XDP_UMEM_FILL_RING, &fill_ring_queue_depth);
		self.set_xdp_socket_option(XDP_UMEM_COMPLETION_RING, &completion_ring_queue_depth);
	}
	
	#[inline(always)]
	pub(crate) fn get_memory_map_offsets(&self) -> xdp_mmap_offsets
	{
		self.get_xdp_socket_option(XDP_MMAP_OFFSETS, true)
	}
	
	#[inline(always)]
	pub(crate) fn set_xdp_socket_option_receive_ring(&self, receive_ring_queue_depth: &RingQueueDepth)
	{
		self.set_xdp_socket_option(XDP_RX_RING, receive_ring_queue_depth)
	}
	
	#[inline(always)]
	pub(crate) fn set_xdp_socket_option_transmit_ring(&self, transmit_ring_queue_depth: &RingQueueDepth)
	{
		self.set_xdp_socket_option(XDP_TX_RING, transmit_ring_queue_depth)
	}
	
	#[inline(always)]
	#[allow(deprecated)]
	fn get_xdp_socket_option<V: Sized>(&self, xdp_socket_option: i32, validate_length_unchanged: bool) -> V
	{
		let OptLen: u32 = size_of::<V>() as u32;
		
		let mut value: V = unsafe { uninitialized() };
		let mut size = OptLen;
		let result = unsafe { getsockopt(self.as_raw_fd(), SOL_XDP, xdp_socket_option, &mut value as *mut V as *mut c_void, &mut size) };
		if likely!(result == 0)
		{
			if validate_length_unchanged
			{
				assert_eq!(size, OptLen, "Unsupported older Linux kernel version 5.3 or older");
			}
			value
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("The argument umem_or_xsk_socket_file_descriptor is not a valid file descriptor"),
				EFAULT => panic!(" The address pointed to by optval is not in a valid part of the process address space. For getsockopt(), this error may also be returned if optlen is not in a valid part of the process address space."),
				EINVAL => panic!("optlen invalid in setsockopt().  In some cases this error can also occur for an invalid value in optval (e.g., for the IP_ADD_MEMBERSHIP option described in ip(7))"),
				ENOPROTOOPT => panic!("The option is unknown at the level indicated"),
				ENOTSOCK => panic!("The file descriptor umem_or_xsk_socket_file_descriptor does not refer to a socket"),
				EOPNOTSUPP => panic!("Unsupported sockopt"),
				
				unexpected @ _ => unreachable!("Unexpected error {} from getsockopt()", unexpected),
			}
		}
		else
		{
			unreachable!("Unexpected result {} from getsockopt()", result);
		}
	}
	
	#[inline(always)]
	fn set_xdp_socket_option<V: Sized>(&self, xdp_socket_option: i32, value: &V)
	{
		let size = size_of::<V>() as u32;
		let result = unsafe { setsockopt(self.as_raw_fd(), SOL_XDP, xdp_socket_option, value as *const V as *const c_void, size) };
		
		if likely!(result == 0)
		{
			return
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("The argument `sockfd` is not a valid descriptor"),
				EFAULT => panic!("The address pointed to by `optval` is not in a valid part of the process address space"),
				EINVAL => panic!("`optlen` is invalid, or there is an invalid value in `optval`"),
				ENOPROTOOPT => panic!("The option is unknown at the level indicated"),
				ENOTSOCK => panic!("The argument `sockfd` is a file, not a socket"),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!();
		}
	}
	
}
