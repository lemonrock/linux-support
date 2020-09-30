// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Congestion control algorithm.
///
/// The Linux default is `pfifo_fast` but we prefer a default of `fq`.
///
/// Many more names can be found by looking in the Linux source for calls to the function `register_qdisc()`.
///
/// Linux models this internally as `ObjectName16`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
	pub fn pfifo_fast() -> &'static Self
	{
		lazy_static!
		{
    		static ref pfifo_fast: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"pfifo_fast\0").unwrap();
    	}
		
		&pfifo_fast
	}
	
	/// Used for devices such as `lo` and `veth`.
	#[inline(always)]
	pub fn noqueue() -> &'static Self
	{
		lazy_static!
		{
    		static ref noqueue: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"noqueue\0").unwrap();
    	}
		
		&noqueue
	}
	
	/// Does nothing.
	#[inline(always)]
	pub fn noop() -> &'static Self
	{
		lazy_static!
		{
    		static ref noop: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"noop\0").unwrap();
    	}
		
		&noop
	}
	
	/// fq.
	#[inline(always)]
	pub fn fq() -> &'static Self
	{
		lazy_static!
		{
    		static ref fq: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq\0").unwrap();
    	}
		
		&fq
	}
	
	/// atm.
	#[inline(always)]
	pub fn atm() -> &'static Self
	{
		lazy_static!
		{
    		static ref atm: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"atm\0").unwrap();
    	}
		
		&atm
	}
	
	/// blackhole.
	#[inline(always)]
	pub fn blackhole() -> &'static Self
	{
		lazy_static!
		{
    		static ref blackhole: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"blackhole\0").unwrap();
    	}
		
		&blackhole
	}
	
	/// cake.
	#[inline(always)]
	pub fn cake() -> &'static Self
	{
		lazy_static!
		{
    		static ref cake: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"cake\0").unwrap();
    	}
		
		&cake
	}
	
	/// cbq.
	#[inline(always)]
	pub fn cbq() -> &'static Self
	{
		lazy_static!
		{
    		static ref cbq: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"cbq\0").unwrap();
    	}
		
		&cbq
	}
	
	/// cbs.
	#[inline(always)]
	pub fn cbs() -> &'static Self
	{
		lazy_static!
		{
    		static ref cbs: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"cbs\0").unwrap();
    	}
		
		&cbs
	}
	
	/// choke.
	#[inline(always)]
	pub fn choke() -> &'static Self
	{
		lazy_static!
		{
    		static ref choke: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"choke\0").unwrap();
    	}
		
		&choke
	}
	
	/// codel.
	#[inline(always)]
	pub fn codel() -> &'static Self
	{
		lazy_static!
		{
    		static ref codel: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"codel\0").unwrap();
    	}
		
		&codel
	}
	
	/// drr.
	#[inline(always)]
	pub fn drr() -> &'static Self
	{
		lazy_static!
		{
    		static ref drr: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"drr\0").unwrap();
    	}
		
		&drr
	}
	
	/// dsmark.
	#[inline(always)]
	pub fn dsmark() -> &'static Self
	{
		lazy_static!
		{
    		static ref dsmark: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"dsmark\0").unwrap();
    	}
		
		&dsmark
	}
	
	/// etf.
	#[inline(always)]
	pub fn etf() -> &'static Self
	{
		lazy_static!
		{
    		static ref etf: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"etf\0").unwrap();
    	}
		
		&etf
	}
	
	/// ets.
	#[inline(always)]
	pub fn ets() -> &'static Self
	{
		lazy_static!
		{
    		static ref ets: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"ets\0").unwrap();
    	}
		
		&ets
	}
	
	/// fq_codel.
	#[inline(always)]
	pub fn fq_codel() -> &'static Self
	{
		lazy_static!
		{
    		static ref fq_codel: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq_codel\0").unwrap();
    	}
		
		&fq_codel
	}
	
	/// fq_pie.
	#[inline(always)]
	pub fn fq_pie() -> &'static Self
	{
		lazy_static!
		{
    		static ref fq_pie: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"fq_pie\0").unwrap();
    	}
		
		&fq_pie
	}
	
	/// gred.
	#[inline(always)]
	pub fn gred() -> &'static Self
	{
		lazy_static!
		{
    		static ref gred: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"gred\0").unwrap();
    	}
		
		&gred
	}
	
	/// hfsc.
	#[inline(always)]
	pub fn hfsc() -> &'static Self
	{
		lazy_static!
		{
    		static ref hfsc: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"hfsc\0").unwrap();
    	}
		
		&hfsc
	}
	
	/// hhf.
	#[inline(always)]
	pub fn hhf() -> &'static Self
	{
		lazy_static!
		{
    		static ref hhf: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"hhf\0").unwrap();
    	}
		
		&hhf
	}
	
	/// htb.
	#[inline(always)]
	pub fn htb() -> &'static Self
	{
		lazy_static!
		{
    		static ref htb: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"htb\0").unwrap();
    	}
		
		&htb
	}
	
	/// clsact.
	///
	/// Used for ingress and egress.
	#[inline(always)]
	pub fn clsact() -> &'static Self
	{
		lazy_static!
		{
    		static ref clsact: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"clasct\0").unwrap();
    	}
		
		&clsact
	}
	
	/// ingress.
	///
	/// Used for ingress.
	#[inline(always)]
	pub fn ingress() -> &'static Self
	{
		lazy_static!
		{
    		static ref ingress: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"ingress\0").unwrap();
    	}
		
		&ingress
	}
	
	/// mqprio.
	#[inline(always)]
	pub fn mqprio() -> &'static Self
	{
		lazy_static!
		{
    		static ref mqprio: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"mqprio\0").unwrap();
    	}
		
		&mqprio
	}
	
	/// multiq.
	#[inline(always)]
	pub fn multiq() -> &'static Self
	{
		lazy_static!
		{
    		static ref multiq: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"multiq\0").unwrap();
    	}
		
		&multiq
	}
	
	/// netem.
	#[inline(always)]
	pub fn netem() -> &'static Self
	{
		lazy_static!
		{
    		static ref netem: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"netem\0").unwrap();
    	}
		
		&netem
	}
	
	/// pie.
	#[inline(always)]
	pub fn pie() -> &'static Self
	{
		lazy_static!
		{
    		static ref pie: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"pie\0").unwrap();
    	}
		
		&pie
	}
	
	/// plug.
	#[inline(always)]
	pub fn plug() -> &'static Self
	{
		lazy_static!
		{
    		static ref plug: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"plug\0").unwrap();
    	}
		
		&plug
	}
	
	/// prio.
	#[inline(always)]
	pub fn prio() -> &'static Self
	{
		lazy_static!
		{
    		static ref prio: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"prio\0").unwrap();
    	}
		
		&prio
	}
	
	/// qfq.
	#[inline(always)]
	pub fn qfq() -> &'static Self
	{
		lazy_static!
		{
    		static ref qfq: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"qfq\0").unwrap();
    	}
		
		&qfq
	}
	
	/// red.
	#[inline(always)]
	pub fn red() -> &'static Self
	{
		lazy_static!
		{
    		static ref red: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"red\0").unwrap();
    	}
		
		&red
	}
	
	/// sfb.
	#[inline(always)]
	pub fn sfb() -> &'static Self
	{
		lazy_static!
		{
    		static ref sfb: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"sfb\0").unwrap();
    	}
		
		&sfb
	}
	
	/// sfq.
	#[inline(always)]
	pub fn sfq() -> &'static Self
	{
		lazy_static!
		{
    		static ref sfq: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"sfq\0").unwrap();
    	}
		
		&sfq
	}
	
	/// skbprio.
	#[inline(always)]
	pub fn skbprio() -> &'static Self
	{
		lazy_static!
		{
    		static ref skbprio: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"skbprio\0").unwrap();
    	}
		
		&skbprio
	}
	
	/// taprio.
	#[inline(always)]
	pub fn taprio() -> &'static Self
	{
		lazy_static!
		{
    		static ref taprio: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"taprio\0").unwrap();
    	}
		
		&taprio
	}
	
	/// tbf.
	#[inline(always)]
	pub fn tbf() -> &'static Self
	{
		lazy_static!
		{
    		static ref tbf: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"tbf\0").unwrap();
    	}
		
		&tbf
	}
	
	/// teql.
	#[inline(always)]
	pub fn teql() -> &'static Self
	{
		lazy_static!
		{
    		static ref teql: QueuingDisciplineAlgorithm = QueuingDisciplineAlgorithm::from_bytes(b"teql\0").unwrap();
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
