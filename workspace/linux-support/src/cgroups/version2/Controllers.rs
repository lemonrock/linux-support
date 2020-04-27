// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Controllers.
#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Controllers(HashSet<Controller>);

impl Default for Controllers
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(HashSet::with_capacity(Controller::MaximumNumberOfControllers))
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

impl Controllers
{
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

	#[inline(always)]
	fn from_file(file_path: &Path) -> Result<Self, ControllersFileError>
	{
		Self::from_file_contents(file_path.read_raw_without_line_feed()?)
	}

	#[inline(always)]
	fn from_file_contents(contents: Box<[u8]>) -> Result<Self, ControllersFileError>
	{
		use self::ControllersFileError::*;

		if unlikely!(contents.is_empty())
		{
			return Ok(Self::empty())
		}

		let mut controllers = Self::default();
		for potential_controller in contents.splitn(Controller::MaximumNumberOfControllers, |byte| *byte == b' ')
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

	#[inline(always)]
	fn empty() -> Self
	{
		Self(HashSet::default())
	}
}
