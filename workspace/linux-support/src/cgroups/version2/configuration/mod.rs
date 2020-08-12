// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use self::controllers::*;
use super::*;


/// Controller configuration.
pub mod controllers;


include!("BelowRootCgroupConfiguration.rs");
include!("CachedDesiredControllersAndOurDepth.rs");
include!("CgroupConfiguration.rs");
include!("CgroupConfigurationVariant.rs");
include!("ChildCgroupConfiguration.rs");
include!("ChildrenCgroupConfiguration.rs");
include!("DomainCgroupConfiguration.rs");
include!("DomainCgroupConfigurationVariant.rs");
include!("DomainOrThreadedCgroupConfiguration.rs");
include!("Migration.rs");
include!("ProcessOrThreadIdentifierChoice.rs");
include!("RootCgroupConfiguration.rs");
include!("ThreadedCgroupConfiguration.rs");
include!("ThreadedCgroupConfigurationVariant.rs");

// TODO: Delete old cgroup hierarchy - how?

// TODO: pressure stall information - available on all cgroups, and is writable and pollable!
/*
#ifdef CONFIG_PSI
	{
		.name = "io.pressure",
		.seq_show = cgroup_io_pressure_show,
		.write = cgroup_io_pressure_write,
		.poll = cgroup_pressure_poll,
		.release = cgroup_pressure_release,
	},
	{
		.name = "memory.pressure",
		.seq_show = cgroup_memory_pressure_show,
		.write = cgroup_memory_pressure_write,
		.poll = cgroup_pressure_poll,
		.release = cgroup_pressure_release,
	},
	{
		.name = "cpu.pressure",
		.seq_show = cgroup_cpu_pressure_show,
		.write = cgroup_cpu_pressure_write,
		.poll = cgroup_pressure_poll,
		.release = cgroup_pressure_release,
	},

 */

/// Configures.
// pub fn configure(&self, mount_point: &CgroupMountPoint) -> Result<(), CgroupConfigurationError>
// {
// 	// TODO: We cannot remove children if there are active processes (or threads) in a cgroup - check the populated field in cgroup.events.
// 	for extant_child in child_cgroup_names(mount_point.to_path()).map_err(CouldNotFindExistingChildren)?
// 	{
// 		if !self.children.contains_key(extant_child.name())
// 		{
// 			extant_child.remove(mount_point).map_err(CouldNotRemoveUnwantedChild)?
// 		}
// 	}
// }

