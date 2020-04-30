// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[link(name = "seccomp")]
extern
{
	/// Initialize the filter state.
	///
	/// * `def_action`: the default filter action.
	///
	/// This function initializes the internal seccomp filter state and should be called before any other functions in this library to ensure the filter state is initialized.
	/// Returns a filter context on success, `NULL` on failure.
	pub(crate) fn seccomp_init(def_action: u32) -> *mut scmp_filter_ctx;

	/// Destroys the filter state and releases any resources.
	///
	/// * `ctx`: the filter context.
	///
	/// This functions destroys the given seccomp filter state and releases any resources, including memory, associated with the filter state.
	/// This function does not reset any seccomp filters already loaded into the kernel.
	/// The filter context can no longer be used after calling this function.
	pub(crate) fn seccomp_release(ctx: *mut scmp_filter_ctx);

	/// Adds an architecture to the filter.
	///
	/// * `ctx`: the filter context.
	/// * `arch_token`: the architecture token, `SCMP_ARCH_*`.
	///
	/// This function adds a new architecture to the given seccomp filter context.
	/// Any new rules added after this function successfully returns will be added to this architecture but existing rules will not be added to this architecture.
	///
	/// If the architecture token is `SCMP_ARCH_NATIVE` then the native architecture will be assumed.
	/// Returns zero on success, negative values on failure.
	pub(crate) fn seccomp_arch_add(ctx: *mut scmp_filter_ctx, arch_token: u32) -> c_int;

	/// Loads the filter into the kernel
	///
	/// * `ctx`: the filter context.
	///
	/// This function loads the given seccomp filter context into the kernel.  If
	/// the filter was loaded correctly, the kernel will be enforcing the filter
	/// when this function returns.  Returns zero on success, negative values on
	/// error.
	pub(crate) fn seccomp_load(ctx: *const scmp_filter_ctx) -> c_int;

	/// Set the value of a filter attribute.
	///
	/// * `ctx`: the filter context.
	/// * `attr`: the filter attribute name.
	/// * `value`: the filter attribute value.
	///
	/// This function sets the value of the given attribute.
	/// Returns zero on success, negative values on failure.
	pub(crate) fn seccomp_attr_set(ctx: *mut scmp_filter_ctx, attr: scmp_filter_attr, value: u32) -> c_int;

	/// Resolve a syscall name to a number.
	///
	/// * `name`: the syscall name.
	///
	/// Resolve the given syscall name to the syscall number.
	/// Returns the syscall number on success, including negative pseudo syscall numbers (e.g. `__PNR_*`); returns `__NR_SCMP_ERROR` on failure.
	#[allow(dead_code)]
	pub(crate) fn seccomp_syscall_resolve_name(name: *const c_char) -> c_int;

	/// Set the priority of a given syscall.
	///
	/// * `ctx`: the filter context.
	/// * `syscall`: the syscall number.
	/// * `priority`: priority value, higher value => higher priority.
	///
	/// This function sets the priority of the given syscall; this value is used when generating the seccomp filter code such that higher priority syscalls will incur less filter code overhead than the lower priority syscalls in the filter.
	/// Returns zero on success, negative values on failure.
	pub(crate) fn seccomp_syscall_priority(ctx: *mut scmp_filter_ctx, syscall: c_int, priority: u8) -> c_int;

	/// Add a new rule to the filter.
	///
	/// * `ctx`: the filter context.
	/// * `action`: the filter action.
	/// * `syscall`: the syscall number.
	/// * `arg_cnt`:  the number of elements in the `arg_array` parameter.
	/// * `arg_array`: array of `scmp_arg_cmp` structs.
	///
	/// This function adds a series of new argument/value checks to the seccomp filter for the given syscall; multiple argument/value checks can be specified and they will be chained together (AND'd together) in the filter.
	/// If the specified rule can not be represented on the architecture the function will fail.
	/// Returns zero on success, negative values on failure.
	pub(crate) fn seccomp_rule_add_exact_array(ctx: *mut scmp_filter_ctx, action: u32, syscall: c_int, arg_cnt: c_uint, arg_array: *const scmp_arg_cmp) -> c_int;

	pub(crate) fn seccomp_syscall_resolve_name_arch(arch_token: u32, name: *const c_char) -> c_int;

	/// Generate seccomp Pseudo Filter Code (PFC) and export it to a file.
	///
	/// * `ctx`: the filter context.
	/// * `fd`: the destination fd.
	///
	/// This function generates seccomp Pseudo Filter Code (PFC) and writes it to the given fd.
	/// Returns zero on success, negative values on failure.
	#[allow(dead_code)]
	pub(crate) fn seccomp_export_pfc(ctx: *const scmp_filter_ctx, fd: c_int) -> c_int;

	/// Generate seccomp Berkley Packet Filter (BPF) code and export it to a file.
	///
	/// * `ctx`: the filter context.
	/// * `fd`: the destination fd.
	///
	/// This function generates seccomp Berkley Packer Filter (BPF) code and writes it to the given fd.
	/// Returns zero on success, negative values on failure.
	#[allow(dead_code)]
	pub(crate) fn seccomp_export_bpf(ctx: *const scmp_filter_ctx, fd: c_int) -> c_int;
}
