use std::{
    env,
    path::{Path, PathBuf},
};

use semver::Version;

fn main() {
    let libcamera_version = Version::new(
        libcamera_sys::LIBCAMERA_VERSION_MAJOR as _,
        libcamera_sys::LIBCAMERA_VERSION_MINOR as _,
        libcamera_sys::LIBCAMERA_VERSION_PATCH as _,
    );

    let versioned_files = Path::new("versioned_files");
    let mut candidates = std::fs::read_dir(versioned_files)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            let version = Version::parse(path.file_name().ok()?.to_str()?).ok()?;

            Some((version, path))
        })
        .collect::<Vec<_>>();

    // Filter to only compatible versions
    let matching = candidates.iter().filter(|(candidate, _)| {
        // Don't allow major version mismatch
        if libcamera_version.major != candidate.major {
            return false;
        }

        // Don't allow newer minor/minor.patch versions
        if libcamera_version.minor < candidate.minor {
            return false;
        }
        if libcamera_version.minor == candidate.minor && libcamera_version.patch < candidate.patch {
            return false;
        }

        true
    });
    // Most recent compatible version
    let (_, selected_version) = match matching.max_by_key(|(version, _)| version.clone()) {
        Some(v) => v,
        None => panic!(
            "Unsupported version of libcamera detected: {version}\nsupported versions are: \n{}",
            candidates
                .iter()
                .map(|(v, _)| format!("\t{v}"))
                .collect::<Vec<_>>()
                .join("\n")
        ),
    };

    if std::fs::metadata(&selected_version).is_err() {
        panic!("Unsupported version of libcamera detected: {version}");
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    for file in ["controls.rs", "properties.rs"] {
        std::fs::copy(selected_version.join(file), out_path.join(file)).unwrap();
    }
}
