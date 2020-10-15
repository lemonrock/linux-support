use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    str,
};

// The rustc-cfg strings below are *not* public API. Please let us know by
// opening a GitHub issue if your build environment requires some way to enable
// these cfgs other than by executing our build script.
fn main() {
    println!("cargo:rustc-cfg=const_fn_has_build_script");

    let rustc = env::var_os("RUSTC").map_or_else(|| "rustc".into(), PathBuf::from);
    let version = match Version::from_rustc(&rustc) {
        Ok(version) => format!("{:#?}\n", version),
        Err(e) => panic!("{}", e),
    };

    let out_dir = env::var_os("OUT_DIR").map(PathBuf::from).expect("OUT_DIR not set");
    let out_file = out_dir.join("version.rs");
    fs::write(out_file, version).expect("failed to write version.rs");
}

#[derive(Debug)]
struct Version {
    minor: u16,
    patch: u16,
    nightly: bool,
}

// Based on https://github.com/cuviper/autocfg/blob/1.0.1/src/version.rs
//
// Using our own parser instead of the existing crates to generate better errors.
impl Version {
    // from the verbose version output
    fn from_vv(vv: &str) -> Option<Self> {
        // Find the release line in the verbose version output.
        let release = vv
            .lines()
            .find(|line| line.starts_with("release: "))
            .map(|line| &line["release: ".len()..])?;

        // Split the version and channel info.
        let mut version_channel = release.split('-');
        let version = version_channel.next().unwrap();
        let channel = version_channel.next();

        // Split the version into semver components.
        let mut digits = version.splitn(3, '.');
        let major = digits.next()?;
        if major != "1" {
            return None;
        }
        let minor = digits.next()?.parse().ok()?;
        let patch = digits.next().unwrap_or("0").parse().ok()?;

        let nightly = channel.map_or(false, |c| c == "dev" || c == "nightly");
        Some(Version { minor, patch, nightly })
    }

    fn from_rustc(rustc: &Path) -> Result<Self, String> {
        let output =
            Command::new(rustc).args(&["--version", "--verbose"]).output().map_err(|e| {
                format!("failed to run `{} --version --verbose`: {}", rustc.display(), e)
            })?;
        if !output.status.success() {
            return Err("could not execute rustc".to_string());
        }
        let output = str::from_utf8(&output.stdout).map_err(|e| {
            format!("failed to parse output of `{} --version --verbose`: {}", rustc.display(), e)
        })?;

        Self::from_vv(output).ok_or_else(|| {
            format!("unexpected output from `{} --version --verbose`: {}", rustc.display(), output)
        })
    }
}
