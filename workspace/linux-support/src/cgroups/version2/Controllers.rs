// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Controllers.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Controllers(HashSet<Controller>);

impl From<HashSet<Controller>> for Controllers
{
	#[inline(always)]
	fn from(value: HashSet<Controller>) -> Self
	{
		Self(value)
	}
}

impl Deref for Controllers
{
	type Target = HashSet<Controller>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for Controllers
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl FromBytes for Controllers
{
	type Error = ControllersFileError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		if unlikely!(bytes.is_empty())
		{
			return Ok(Self::default())
		}
		
		use self::ControllersFileError::*;
		
		let mut controllers = Self::new_if_going_to_be_full();
		for potential_controller in bytes.split_bytes_n(Controller::MaximumNumberOfControllers, b' ')
		{
			let controller = Controller::from_bytes(potential_controller)?;
			let added_first_time = controllers.insert(controller);
			if unlikely!(!added_first_time)
			{
				return Err(DuplicateController(controller))
			}
		}
		controllers.shrink_to_fit();
		Ok(controllers)
	}
}

impl Controllers
{
	#[inline(always)]
	fn new_if_going_to_be_full() -> Self
	{
		Self(HashSet::with_capacity(Controller::MaximumNumberOfControllers))
	}
	
	pub(crate) fn add_if_some<CC: ControllerConfiguration>(&mut self, controller: &Option<CC>)
	{
		if controller.is_some()
		{
			self.insert(CC::Controller);
		}
	}
	
	/// Complement, taking into account Controllers that are not extant on this version of Linux.
	///
	/// Trying to enabled or disable a Controller that is not extant causes a write error.
	#[inline(always)]
	pub fn to_enable_and_disable(&self, available_controllers: &Self) -> (Self, Self)
	{
		let mut enabled = Self::new_if_going_to_be_full();
		let mut disabled = Self::new_if_going_to_be_full();
		
		for controller in Controller::iter()
		{
			if !available_controllers.contains(&controller)
			{
				continue
			}
			if self.contains(&controller)
			{
				enabled.insert(controller);
			}
			else
			{
				disabled.insert(controller);
			}
		}
		(enabled, disabled)
	}
	
	/// Contains only threaded controllers, ie no domain controllers.
	#[inline(always)]
	pub fn contains_only_threaded_controllers(&self) -> bool
	{
		for controller in self.iter()
		{
			if !(*controller).is_threaded_controller()
			{
				return false
			}
		}
		true
	}
	
	#[inline(always)]
	pub(crate) fn merge(&mut self, other: &Self)
	{
		for controller in other.iter()
		{
			self.insert(*controller);
		}
	}
	
	/// Creates a change line such as `+pids -memory` *with* a trailing line feed.
	fn create_change_line(enable: &Self, disable: &Self) -> Vec<u8>
	{
		#[inline(always)]
		fn append_to_line(line: &mut Vec<u8>, sign: u8, controllers: &Controllers)
		{
			let mut controllers = controllers.iter();
			let first_controller = controllers.next();
			match first_controller
			{
				None => return,
				Some(first_controller) => first_controller.append_to(line, sign),
			}
			for subsequent_controller in controllers
			{
				subsequent_controller.append_to(line, sign)
			}
		}

		let mut line = Vec::with_capacity(Controller::MaximumNumberOfControllers * 10);
		append_to_line(&mut line, b'+', enable);
		append_to_line(&mut line, b'-', disable);
		line.push(b'\n');
		line
	}
}
