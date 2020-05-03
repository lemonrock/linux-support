// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process capabilities.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct ProcessCapabilitiesConfiguration
{
	#[allow(missing_docs)]
	pub bounding_set_to_retain: BoundingCapabilitySet,

	#[allow(missing_docs)]
	pub permitted_effective_and_inheritable_capability_sets: PermittedEffectiveAndInheritableCapabilitySets,

	#[allow(missing_docs)]
	pub ambient_capabilities: AmbientCapabilitySet,
}

impl ProcessCapabilitiesConfiguration
{
	fn configure_if_wanted(&self) -> Result<(), ProcessCapabilitiesConfigurationError>
	{
		use self::ProcessCapabilitiesConfigurationError::*;

		self.bounding_set_to_retain.retain().map_err(|_ :()| CouldNotConfigureBoundingSet)?;
		self.permitted_effective_and_inheritable_capability_sets.set(ThreadIdentifier::default()).map_err(CouldNotConfigurePermittedEffectiveAndInheritableSets)?;
		self.ambient_capabilities.set_for_current_thread().map_err(CouldNotConfigureAmbient)?;

		lock_secure_bits_so_capabilities_are_always_enforced(true).map_err(|cause| CouldNotLockSecureBits(cause))
	}

	fn configure_if_unwanted() -> Result<(), ProcessCapabilitiesConfigurationError>
	{
		use self::ProcessCapabilitiesConfigurationError::*;

		AmbientCapabilitySet::clear_current_thread_ambient_set();
		lock_secure_bits_so_capabilities_are_always_enforced(false).map_err(|cause| CouldNotLockSecureBits(cause))
	}
}
