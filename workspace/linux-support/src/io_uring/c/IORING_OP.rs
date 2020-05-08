// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub(super) enum IORING_OP
{
	/// Does nothing.
	IORING_OP_NOP = 0,

	/// Vectored read operation similar to preadv2(2).
	IORING_OP_READV = 1,

	/// Vectored write operation similar to pwritev2(2).
	IORING_OP_WRITEV = 2,

	/// File sync.
	///
	/// See also `fsync(2)`.
	///
	/// Note that, while I/O is initiated in the order in which it appears in the submission queue, completions are unordered.
	/// For example, an application which places a write I/O followed by an fsync in the submission queue cannot expect the fsync to apply to the write.
	/// The two operations execute in parallel, so the fsync may complete before the write is issued to the storage.
	/// The same is also true for previously issued writes that have not completed prior to the fsync.
	IORING_OP_FSYNC = 3,

	/// Read from pre-mapped buffers.
	///
	/// `addr` and `len` must fall within the buffer located at `buf_index` in the fixed buffer array.
	IORING_OP_READ_FIXED = 4,

	/// Write to pre-mapped buffers.
	///
	/// `addr` and `len` must fall within the buffer located at `buf_index` in the fixed buffer array.
	IORING_OP_WRITE_FIXED = 5,

	/// Poll the fd specified in the submission queue entry for the events specified in the poll_events field.
	/// Unlike poll or epoll without `EPOLLONESHOT`, this interface always works in one shot mode.
	/// That is, once the poll operation is completed, it will have to be resubmitted.
	IORING_OP_POLL_ADD = 6,

	/// Remove an existing poll request.
	///
	/// If found, the res field of the struct io_uring_cqe will contain 0.
	/// If not found, res will contain -ENOENT.
	IORING_OP_POLL_REMOVE = 7,

	/// Issue the equivalent of a sync_file_range (2) on the file descriptor.
	/// The `fd` field is the file descriptor to sync, the `off` field holds the offset in bytes, the `len` field holds the length in bytes, and the `flags` field holds the flags for the command.
	///
	/// See also sync_file_range(2) for the general description of the related system call.
	///
	/// Since Linux 5.2.
	IORING_OP_SYNC_FILE_RANGE = 8,

	/// Issue the equivalent of a sendmsg(2) system call.
	///
	/// `fd` must be set to the socket file descriptor, `addr` must contain a pointer to the `msghdr` structure, and flags holds the flags associated with the system call.
	///
	/// See also sendmsg(2) for the general description of the related system call.
	///
	/// Since Linux 5.3.
	IORING_OP_SENDMSG = 9,

	/// Issue the equivalent of a recvmsg(2) system call.
	///
	/// `fd` must be set to the socket file descriptor, `addr` must contain a pointer to the `msghdr` structure, and flags holds the flags associated with the system call.
	///
	/// See also recvmsg(2) for the general description of the related system call.
	///
	/// Since Linux 5.3.
	IORING_OP_RECVMSG = 10,

	/// This command will register a timeout operation.
	///
	/// The `addr` field must contain a pointer to a struct `timespec64`, `len` must contain 1 to signify one timespec64 structure, `timeout_flags` may contain `IORING_TIMEOUT_ABS` for an absolute timeout value, or `0` for a relative timeout.
	/// `off` may contain a completion event count.
	/// If not set, this defaults to 1.
	/// A timeout will trigger a wakeup event on the completion ring for anyone waiting for events.
	/// A timeout condition is met when either the specified timeout expires, or the specified number of events have completed.
	/// Either condition will trigger the event.
	/// io_uring timeouts use the `CLOCK_MONOTONIC` clock source.
	/// The request will complete with `-ETIME` if the timeout got completed through expiration of the timer, or 0 if the timeout got completed through requests completing on their own.
	/// If the timeout was cancelled before it expired, the request will complete with `-ECANCELED`.
	///
	/// Since Linux 5.4.
	IORING_OP_TIMEOUT = 11,

	/// Attempt to remove an existing timeout operation.
	///
	/// `addr` must contain the user_data field of the previously issued timeout operation.
	/// If the specified timeout request is found and cancelled successfully, this request will terminate with a result value of 0.
	/// If the timeout request was found but expiration was already in progress, this request will terminate with a result value of `-EBUSY`.
	/// If the timeout request wasn't found, the request will terminate with a result value of `-ENOENT`.
	///
	/// Since Linux 5.5.
	IORING_OP_TIMEOUT_REMOVE = 12,

	/// Issue the equivalent of an `accept4()` system call.
	///
	/// `fd` must be set to the socket file descriptor, `addr` must contain the pointer to the `sockaddr` structure, and `addr2` must contain a pointer to the `socklen_t addrlen` field.
	///
	/// See also accept4(2) for the general description of the related system call.
	///
	/// Since Linux 5.5.
	IORING_OP_ACCEPT = 13,

	/// Attempt to cancel an already issued request.
	///
	/// `addr` must contain the `user_data` field of the request that should be cancelled.
	///
	/// The cancellation request will complete with one of the following results codes:-
	///
	/// * If found, the `res` field of the CQE will contain 0.
	/// * If not found, res will contain `-ENOENT`.
	/// * If found and attempted cancelled, the res field will contain `-EALREADY`: in this case, the request may or may not terminate.
	///
	/// In general, requests that are interruptible (like socket IO) will get cancelled, while disk IO requests cannot be cancelled if already started.
	///
	/// Since Linux 5.5.
	IORING_OP_ASYNC_CANCEL = 14,

	/// This request must be linked with another request through IOSQE_IO_LINK which is described below.
	///
	/// Unlike IORING_OP_TIMEOUT, IORING_OP_LINK_TIMEOUT acts on the linked request, not the completion queue.
	/// The format of the command is otherwise like IORING_OP_TIMEOUT, except there's no completion event count as it's tied to a specific request.
	/// If used, the timeout specified in the command will cancel the linked command, unless the linked command completes before the timeout.
	/// The timeout will complete with -ETIME if the timer expired and the linked request was attempted cancelled, or -ECANCELED if the timer got cancelled because of completion of the linked request
	///
	/// Like IORING_OP_TIMEOUT the clock source used is `CLOCK_MONOTONIC`.
	///
	/// Since Linux 5.5.
	IORING_OP_LINK_TIMEOUT = 15,

	/// Issue the equivalent of a connect(2) system call.
	///
	/// `fd` must be set to the socket file descriptor, `addr` must contain the pointer to the `sockaddr` structure, and `off` must contain the `socklen_t addrlen` field.
	///
	/// See also connect(2) for the general description of the related system call.
	///
	/// Since Linux 5.5.
	IORING_OP_CONNECT = 16,

	/// Issue the equivalent of a fallocate(2) system call.
	///
	/// `fd` must be set to the file descriptor, `off` must contain the offset on which to operate, and `len` must contain the length.
	///
	/// `addr` is `AllocationMode`.
	///
	/// See also `fallocate(2)` for the general description of the related system call.
	///
	/// Since Linux 5.6.
	IORING_OP_FALLOCATE = 17,

	/// Issue the equivalent of a openat(2) system call.
	///
	/// `fd` is the dirfd argument, `addr` must contain a pointer to the `*pathname` argument, `open_flags` should contain any flags passed in, and `mode` is access mode of the file.
	///
	/// See also `openat(2)` for the general description of the related system call.
	///
	/// Since Linux 5.6.
	IORING_OP_OPENAT = 18,

	/// Issue the equivalent of a close(2) system call.
	///
	/// `fd` is the file descriptor to be closed.
	///
	/// See also `close(2)` for the general description of the related system call.
	///
	/// Since Linux 5.6.
	IORING_OP_CLOSE = 19,

	/// This command is an alternative to using `replace_some_registered_file_descriptors()` which then works in an async fashion, like the rest of the io_uring commands.
	///
	/// The arguments passed in are the same.
	/// `addr` must contain a pointer to the array of file descriptors, `len` must contain the length of the array, and `off` must contain the offset at which to operate.
	/// Note that the array of file descriptors pointed to in `addr` must remain valid until this operation has completed.
	///
	/// Since Linux 5.6.
	IORING_OP_FILES_UPDATE = 20,

	/// Issue the equivalent of a statx(2) system call.
	///
	/// `fd` is the dirfd argument, `addr` must contain a pointer to the `*pathname` string, `statx_flags` is the flags argument, `len` should be the mask argument, and `off` must contain a pointer to the `statxbuf` to be filled in.
	///
	/// See also statx(2) for the general description of the related system call.
	///
	/// Since Linux 5.6.
	IORING_OP_STATX = 21,

	/// Issue the equivalent of a read(2) system call.
	///
	/// `fd` is the file descriptor to be operated on, `addr` contains the buffer in question, and `len` contains the length of the IO operation.
	///
	/// See also read(2) for the general description of the related system call.
	///
	/// Since Linux 5.6.
	IORING_OP_READ = 22,

	/// Issue the equivalent of a write(2) system call.
	///
	/// `fd` is the file descriptor to be operated on, `addr` contains the buffer in question, and `len` contains the length of the IO operation.
	///
	/// See also write(2) for the general description of the related system call.
	///
	/// Since Linux 5.6.
	IORING_OP_WRITE = 23,

	/// Issue the equivalent of a posix_fadvise(2) system call.
	///
	/// `fd` must be set to the file descriptor, `off` must contain the offset on which to operate, `len` must contain the length, and `fadvise_advice` must contain the advice associated with the operation.
	///
	/// See also posix_fadvise(2) for the general description of the related system call.
	///
	/// Since Linux 5.6.
	IORING_OP_FADVISE = 24,

	/// Issue the equivalent of a madvise(2) system call.
	///
	/// `addr` must contain the address to operate on, `len` must contain the length on which to operate, and `fadvise_advice` must contain the advice associated with the operation.
	///
	/// See also madvise(2) for the general description of the related system call.
	///
	/// Since Linux 5.6.
	IORING_OP_MADVISE = 25,

	/// Issue the equivalent of a send(2) system call
	///
	/// `fd` must be set to the socket file descriptor, `addr` must contain a pointer to the buffer, `len` denotes the length of the buffer to send, and `flags` holds the flags associated with the system call.
	///
	/// See also send(2) for the general description of the related system call.
	///
	/// Since Linux 5.6.
	IORING_OP_SEND = 26,

	/// Issue the equivalent of a recv(2) system call
	///
	/// `fd` must be set to the socket file descriptor, `addr` must contain a pointer to the buffer, `len` denotes the length of the buffer to send, and `flags` holds the flags associated with the system call.
	///
	/// See also recv(2) for the general description of the related system call.
	///
	/// Since Linux 5.6.
	IORING_OP_RECV = 27,

	/// Issue the equivalent of a `openat2(2)` system call.
	/// 
	/// `fd` is the `dirfd` argument, `addr` must contain a pointer to the `*pathname` argument, `len` should contain the size of the open_how structure, and `off` should be set to the address of the open_how structure.
	/// 
	/// See also `openat2(2)` for the general description of the related system call.
	///
	/// Since Linux 5.6.
	IORING_OP_OPENAT2 = 28,

	/// Add, remove or modify entries in the interest list of `epoll()`.
	///
	/// See `epoll_ctl()` for details of the system call.
	///
	/// `fd` holds the file descriptor that represents the epoll instance, `addr` holds the file descriptor to add, remove or modify, `len` holds the operation (`EPOLL_CTL_ADD`, `EPOLL_CTL_DEL`, or `EPOLL_CTL_MOD`) to perform and, `off` holds a pointer to the `epoll_events` structure.
	///
	/// Since Linux 5.6.
	IORING_OP_EPOLL_CTL = 29,

	/// Issue the equivalent of a `splice(2)` system call.
	///
	/// `splice_fd_in` is the file descriptor to read from, `splice_off_in` is a pointer to an offset to read from, `fd` is the file descriptor to write to, `off` is a pointer to an offset to from which to start writing to.
	/// `len` contains the number of bytes to copy.
	/// `splice_flags` contains a bit mask for the flag field associated with the system call.
	///
	/// Please note that one of the file descriptors must refer to a pipe.
	///
	/// See also `splice(2)` for the general description of the related system call.
	///
	/// Since Linux 5.7.
	IORING_OP_SPLICE = 30,

	IORING_OP_PROVIDE_BUFFERS = 31,

	IORING_OP_REMOVE_BUFFERS = 32,
}

impl IORING_OP
{
	pub const IORING_OP_LAST: usize = 33;
}
