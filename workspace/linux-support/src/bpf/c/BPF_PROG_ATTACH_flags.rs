// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags for `BPF_PROG_ATTACH`.
	///
	/// cgroup-bpf attach flags used in `BPF_PROG_ATTACH` command.
	///
	/// * `empty()`: No further bpf programs allowed in the subtree.
	/// * `BPF_F_ALLOW_OVERRIDE`: If a sub-cgroup installs some bpf program, the program in this cgroup yields to sub-cgroup program.
	/// * `BPF_F_ALLOW_MULT`I: If a sub-cgroup installs some bpf program, that cgroup program gets run in addition to the program in this cgroup.
	///
	/// Only one program is allowed to be attached to a cgroup with `empty()` or `BPF_F_ALLOW_OVERRIDE` flag.
	/// Attaching another program on top of `empty()` or `BPF_F_ALLOW_OVERRIDE` will release old program and attach the new one.
	/// Attach flags has to match.
	///
	/// Multiple programs are allowed to be attached to a cgroup with `BPF_F_ALLOW_MULTI` flag.
	/// They are executed in FIFO order (those that were attached first, run first).
	/// The programs of sub-cgroup are executed first, then programs of this cgroup and then programs of parent cgroup.
	/// When the child program makes a decision (like picking TCP CA or sock bind) parent program has a chance to override it.
	///
	/// With `BPF_F_ALLOW_MULTI` a new program is added to the end of the list of programs for a cgroup.
	/// Though it's possible to replace an old program at any position by also specifying `BPF_F_REPLACE` flag and position itself in `replace_bpf_fd` attribute.
	/// Old program at this position will be released.
	///
	/// A cgroup with `BPF_F_ALLOW_MULTI` or `BPF_F_ALLOW_OVERRIDE` flag allows any attach flags in sub-cgroups.
	/// A cgroup with `empty()` doesn't allow any programs in sub-cgroups.
	///
	/// Example:-
	/// ```bash
	/// cgroup1 (MULTI progs A, B) ->
	///    cgroup2 (OVERRIDE prog C) ->
	///      cgroup3 (MULTI prog D) ->
	///        cgroup4 (OVERRIDE prog E) ->
	///          cgroup5 (NONE prog F)
	/// ```
	/// the event in cgroup5 triggers execution of F,D,A,B in that order.
	/// if prog F is detached, the execution is E,D,A,B
	/// if prog F and D are detached, the execution is E,A,B
	/// if prog F, E and D are detached, the execution is C,A,B
	///
	/// All eligible programs are executed regardless of return code from earlier programs.
	#[allow(missing_docs)]
	#[derive(Deserialize, Serialize)]
	pub struct BPF_PROG_ATTACH_flags: u32
	{
		#[doc(hidden)]
		const BPF_F_ALLOW_OVERRIDE = 1;
		
		#[doc(hidden)]
		const BPF_F_ALLOW_MULTI = 2;
		
		#[doc(hidden)]
		const BPF_F_REPLACE = 4;
	}
}

impl Default for BPF_PROG_ATTACH_flags
{
	#[inline(always)]
	fn default() -> Self
	{
		BPF_PROG_ATTACH_flags::empty()
	}
}
