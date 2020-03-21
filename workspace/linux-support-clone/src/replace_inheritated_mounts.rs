/// Mount the various mount points (at the very least, `/proc`) then pivot_root.
#[inline(always)]
fn replace_inheritated_mounts(new_root: &Path) -> io::Result<()>
{
	#[inline(always)]
	fn mount_proc(new_root: &Path) -> io::Result<()>
	{
		const Source: ConstCStr = ConstCStr(b"proc\0");
		let mount_point = new_root.join("proc");
		mkdir_wrapper(&mount_point, 0o0700)?;
		mount_wrapper(Source.as_cstr(), &mount_point, &FileSystemType::_proc, &HashMap::default(), MountFlags::empty())
	}

	#[inline(always)]
	fn pivot_file_system_mounts(new_root: &Path) -> io::Result<()>
	{
		const FullOldPivotRootPath: &'static str = "/.pivot_root";

		let new_root = new_root.canonicalize()?;
		let put_old = new_root.join(&FullOldPivotRootPath[1 .. ]);
		mkdir_wrapper(&put_old, 0o0700)?;

		// bind-mount `new_root` to itself so that `pivot_root()` works.
		const FileSystemTypeIgnored: FileSystemType = FileSystemType::tmpfs;
		let BindMount: MountFlags = MountFlags::BindMount | MountFlags::RecursiveBindMount;
		mount_wrapper(&path_to_cstring(&new_root), &new_root, &FileSystemTypeIgnored, &HashMap::default(), BindMount)?;

		pivot_root_wrapper(&new_root, &put_old);
		set_current_dir("/")?;

		let path = PathBuf::from_str(FullOldPivotRootPath).unwrap();
		Mount::unmount(&path, UnmountFlags::Detach)?;
		remove_dir_all(path)
	}

	mount_proc(new_root)?;
	pivot_file_system_mounts(new_root)?;

	Ok(())
}
