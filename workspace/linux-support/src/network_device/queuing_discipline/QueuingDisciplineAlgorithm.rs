// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Congestion control algorithm.
///
/// The Linux default is `pfifo_fast` but we prefer a default of `fq`.
///
/// Many more names can be found by looking in the Linux source for calls to the function `register_qdisc()`.
///
/// Linux models this internally as `ObjectName16`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct QueuingDisciplineAlgorithm(ObjectName16);

impl Default for QueuingDisciplineAlgorithm
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::fq().clone()
	}
}

impl FromBytes for QueuingDisciplineAlgorithm
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		ObjectName16::from_bytes(bytes).map(|object_name| Self(object_name))
	}
}

impl From<ObjectName16> for QueuingDisciplineAlgorithm
{
	#[inline(always)]
	fn from(value: ObjectName16) -> Self
	{
		Self(value)
	}
}

impl Into<ObjectName16> for QueuingDisciplineAlgorithm
{
	#[inline(always)]
	fn into(self) -> ObjectName16
	{
		self.0
	}
}

impl QueuingDisciplineAlgorithm
{
	/// pfifo_fast.
	#[inline(always)]
	pub fn pfifo_fast() -> &Self
	{
		lazy_static!
		{
    		static ref pfifo_fast: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"pfifo_fast\0");
    	}
		
		&pfifo_fast
	}
	
	/// Used for devices such as `lo` and `veth`.
	#[inline(always)]
	pub fn noqueue() -> &Self
	{
		lazy_static!
		{
    		static ref noqueue: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"noqueue\0");
    	}
		
		&noqueue
	}
	
	/// Does nothing.
	#[inline(always)]
	pub fn noop() -> &Self
	{
		lazy_static!
		{
    		static ref noop: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"noop\0");
    	}
		
		&noop
	}
	
	/// fq.
	#[inline(always)]
	pub fn fq() -> &Self
	{
		lazy_static!
		{
    		static ref fq: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&fq
	}
	
	/// atm.
	#[inline(always)]
	pub fn atm() -> &Self
	{
		lazy_static!
		{
    		static ref atm: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&atm
	}
	
	/// blackhole.
	#[inline(always)]
	pub fn blackhole() -> &Self
	{
		lazy_static!
		{
    		static ref blackhole: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&blackhole
	}
	
	/// cake.
	#[inline(always)]
	pub fn cake() -> &Self
	{
		lazy_static!
		{
    		static ref cake: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&cake
	}
	
	/// cbq.
	#[inline(always)]
	pub fn cbq() -> &Self
	{
		lazy_static!
		{
    		static ref cbq: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&cbq
	}
	
	/// cbs.
	#[inline(always)]
	pub fn cbs() -> &Self
	{
		lazy_static!
		{
    		static ref cbs: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&cbs
	}
	
	/// choke.
	#[inline(always)]
	pub fn cbs() -> &Self
	{
		lazy_static!
		{
    		static ref choke: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&choke
	}
	
	/// codel.
	#[inline(always)]
	pub fn codel() -> &Self
	{
		lazy_static!
		{
    		static ref codel: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&codel
	}
	
	/// drr.
	#[inline(always)]
	pub fn drr() -> &Self
	{
		lazy_static!
		{
    		static ref drr: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&drr
	}
	
	/// dsmark.
	#[inline(always)]
	pub fn dsmark() -> &Self
	{
		lazy_static!
		{
    		static ref dsmark: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&dsmark
	}
	
	/// etf.
	#[inline(always)]
	pub fn etf() -> &Self
	{
		lazy_static!
		{
    		static ref etf: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&etf
	}
	
	/// ets.
	#[inline(always)]
	pub fn ets() -> &Self
	{
		lazy_static!
		{
    		static ref ets: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&ets
	}
	
	/// fq_codel.
	#[inline(always)]
	pub fn fq_codel() -> &Self
	{
		lazy_static!
		{
    		static ref fq_codel: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&fq_codel
	}
	
	/// fq_pie.
	#[inline(always)]
	pub fn fq_pie() -> &Self
	{
		lazy_static!
		{
    		static ref fq_pie: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&fq_pie
	}
	
	/// gred.
	#[inline(always)]
	pub fn gred() -> &Self
	{
		lazy_static!
		{
    		static ref gred: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&gred
	}
	
	/// hfsc.
	#[inline(always)]
	pub fn hfsc() -> &Self
	{
		lazy_static!
		{
    		static ref hfsc: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&hfsc
	}
	
	/// hhf.
	#[inline(always)]
	pub fn hhf() -> &Self
	{
		lazy_static!
		{
    		static ref hhf: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&hhf
	}
	
	/// htb.
	#[inline(always)]
	pub fn hhf() -> &Self
	{
		lazy_static!
		{
    		static ref htb: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&htb
	}
	
	/// clsact.
	///
	/// Used for ingress and egress.
	#[inline(always)]
	pub fn clsact() -> &Self
	{
		lazy_static!
		{
    		static ref clsact: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&clsact
	}
	
	/// ingress.
	///
	/// Used for ingress.
	#[inline(always)]
	pub fn ingress() -> &Self
	{
		lazy_static!
		{
    		static ref ingress: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&ingress
	}
	
	/// mqprio.
	#[inline(always)]
	pub fn mqprio() -> &Self
	{
		lazy_static!
		{
    		static ref mqprio: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&mqprio
	}
	
	/// multiq.
	#[inline(always)]
	pub fn multiq() -> &Self
	{
		lazy_static!
		{
    		static ref multiq: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&multiq
	}
	
	/// netem.
	#[inline(always)]
	pub fn netem() -> &Self
	{
		lazy_static!
		{
    		static ref netem: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&netem
	}
	
	/// pie.
	#[inline(always)]
	pub fn pie() -> &Self
	{
		lazy_static!
		{
    		static ref pie: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&pie
	}
	
	/// plug.
	#[inline(always)]
	pub fn plug() -> &Self
	{
		lazy_static!
		{
    		static ref plug: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&plug
	}
	
	/// prio.
	#[inline(always)]
	pub fn prio() -> &Self
	{
		lazy_static!
		{
    		static ref prio: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&prio
	}
	
	/// qfq.
	#[inline(always)]
	pub fn qfq() -> &Self
	{
		lazy_static!
		{
    		static ref qfq: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&qfq
	}
	
	/// red.
	#[inline(always)]
	pub fn red() -> &Self
	{
		lazy_static!
		{
    		static ref red: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&red
	}
	
	/// sfb.
	#[inline(always)]
	pub fn sfb() -> &Self
	{
		lazy_static!
		{
    		static ref sfb: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&sfb
	}
	
	/// sfq.
	#[inline(always)]
	pub fn sfq() -> &Self
	{
		lazy_static!
		{
    		static ref sfq: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&sfq
	}
	
	/// skbprio.
	#[inline(always)]
	pub fn skbprio() -> &Self
	{
		lazy_static!
		{
    		static ref skbprio: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&skbprio
	}
	
	/// taprio.
	#[inline(always)]
	pub fn taprio() -> &Self
	{
		lazy_static!
		{
    		static ref taprio: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&taprio
	}
	
	/// tbf.
	#[inline(always)]
	pub fn tbf() -> &Self
	{
		lazy_static!
		{
    		static ref tbf: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&tbf
	}
	
	/// teql.
	#[inline(always)]
	pub fn teql() -> &Self
	{
		lazy_static!
		{
    		static ref teql: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0");
    	}
		
		&teql
	}
	
	/// Set value of `/proc/sys/net/core/default_qdisc` if it exists.
	#[inline(always)]
	pub fn set_global_default(self, proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write to /proc/sys/net/core/default_qdisc");
		
		let file_path = Self::sys_net_core_default_qdisc_file_path(proc_path);
		
		if file_path.exists()
		{
			self.0.write_to_file_line_feed_terminated(&file_path)
		}
		else
		{
			Ok(())
		}
	}
	
	#[inline(always)]
	fn sys_net_core_default_qdisc_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_net_core_file_path("default_qdisc")
	}
}
