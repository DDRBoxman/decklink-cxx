use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

#[cfg(target_os = "windows")]
fn gen_headers() -> PathBuf {
    use ::std::ffi::OsString;
    use std::fs;
    use std::os::windows::process::CommandExt;

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("decklink_idl");

    invoke_vcvars_if_not_set();

    fs::create_dir(dest_path.clone());

    let mut tmp = OsString::new();
    tmp.push("/out ");
    tmp.push(dest_path.as_os_str());

    Command::new("midl.exe")
        .args(&["/hdecklink_win.h", "/iiddecklink_win.c"])
        .raw_arg(tmp)
        .arg("decklink/Win/include/DeckLinkAPI.idl")
        .status()
        .unwrap();

    return dest_path;
}

#[cfg(not(target_os = "windows"))]
fn gen_headers() -> PathBuf {
    return PathBuf::new();
}

fn main() {
    let mut build = cxx_build::bridge("src/bridge.rs");
    build.file("./include/callback.cc");
    build.file("./include/bridge.cc");

    if cfg!(target_os = "windows") {
        let dest_path = gen_headers();

        build.include(&dest_path);
        build.file("./include/win.cc");

        let decklink_com = dest_path.join("decklink_win.c");
        build.file(decklink_com);
    }

    if cfg!(target_os = "macos") {
        build.file("./decklink/Mac/include/DeckLinkAPIDispatch.cpp");
    } else if cfg!(target_os = "linux") {
        build.file("./decklink/Linux/include/DeckLinkAPIDispatch.cpp");
    }

    build.flag_if_supported("-std=c++14").compile("cxx-demo");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    }

    println!("cargo:rerun-if-changed=src/bridge.rs");
    println!("cargo:rerun-if-changed=include/bridge.cc");
    println!("cargo:rerun-if-changed=include/bridge.h");
    println!("cargo:rerun-if-changed=include/callback.h");
    println!("cargo:rerun-if-changed=include/callback.cc");
    println!("cargo:rerun-if-changed=include/types.h");
    println!("cargo:rerun-if-changed=include/platform.h");
    println!("cargo:rerun-if-changed=include/win.cc");
}

/*
Below lifted from
https://github.com/RustAudio/cpal/blob/2ec761d30b35dbbacfecb41a2f5985781a7d52d1/asio-sys/build.rs#L360
*/

/// Invokes `vcvarsall.bat` to initialize the environment for building with MSVC
///
/// This function is only meant to be called when the host OS is Windows.
fn invoke_vcvars_if_not_set() {
    if vcvars_set() {
        return;
    }
    println!("VCINSTALLDIR is not set. Attempting to invoke vcvarsall.bat..");

    println!("Invoking vcvarsall.bat..");
    println!("Determining system architecture..");

    let arch_arg = determine_vcvarsall_bat_arch_arg();
    println!(
        "Host architecture is detected as {}.",
        std::env::consts::ARCH
    );
    println!("Architecture argument for vcvarsall.bat will be used as: {arch_arg}.");

    let vcvars_all_bat_path = search_vcvars_all_bat();

    println!(
        "Found vcvarsall.bat at {}. Initializing environment..",
        vcvars_all_bat_path.display()
    );

    // Invoke vcvarsall.bat
    let output = Command::new("cmd")
        .args([
            "/c",
            vcvars_all_bat_path.to_str().unwrap(),
            &arch_arg,
            "&&",
            "set",
        ])
        .output()
        .expect("Failed to execute command");

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        // Filters the output of vcvarsall.bat to only include lines of the form "VARNAME=VALUE"
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            env::set_var(parts[0], parts[1]);
            println!("{}={}", parts[0], parts[1]);
        }
    }
}

/// Checks if vcvarsall.bat has been invoked
/// Assumes that it is very unlikely that the user would set `VCINSTALLDIR` manually
fn vcvars_set() -> bool {
    env::var("VCINSTALLDIR").is_ok()
}

/// Searches for vcvarsall.bat in the default installation directories
///
/// If it is not found, it will search for it in the Program Files directories
///
/// If it is still not found, it will panic.
fn search_vcvars_all_bat() -> PathBuf {
    if let Some(path) = guess_vcvars_all_bat() {
        return path;
    }

    // Define search paths for vcvarsall.bat based on architecture
    let paths = &[
        // Visual Studio 2022+
        "C:\\Program Files\\Microsoft Visual Studio\\",
        // <= Visual Studio 2019
        "C:\\Program Files (x86)\\Microsoft Visual Studio\\",
    ];

    // Search for vcvarsall.bat using walkdir
    println!("Searching for vcvarsall.bat in {paths:?}");

    let mut found = None;

    for path in paths.iter() {
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            if entry.path().ends_with("vcvarsall.bat") {
                found.replace(entry.path().to_path_buf());
            }
        }
    }

    match found {
        Some(path) => path,
        None => panic!(
            "Could not find vcvarsall.bat. Please install the latest version of Visual Studio."
        ),
    }
}

/// Guesses the location of vcvarsall.bat by searching it with certain heuristics.
///
/// It is meant to be executed before a top level search over Microsoft Visual Studio directories
/// to ensure faster execution in CI environments.
fn guess_vcvars_all_bat() -> Option<PathBuf> {
    /// Checks if a string is a year
    fn is_year(s: Option<&str>) -> Option<String> {
        let Some(s) = s else {
            return None;
        };

        if s.len() == 4 && s.chars().all(|c| c.is_ascii_digit()) {
            Some(s.to_string())
        } else {
            None
        }
    }

    /// Checks if a string is an edition of Visual Studio
    fn is_edition(s: Option<&str>) -> Option<String> {
        let Some(s) = s else {
            return None;
        };

        let editions = ["Enterprise", "Professional", "Community", "Express"];
        if editions.contains(&s) {
            Some(s.to_string())
        } else {
            None
        }
    }

    /// Constructs a path to vcvarsall.bat based on a base path
    fn construct_path(base: &Path) -> Option<PathBuf> {
        let mut constructed = base.to_path_buf();
        for entry in WalkDir::new(&constructed).max_depth(1) {
            let entry = match entry {
                Err(_) => continue,
                Ok(entry) => entry,
            };
            if let Some(year) = is_year(entry.path().file_name().and_then(|s| s.to_str())) {
                constructed = constructed.join(year);
                for entry in WalkDir::new(&constructed).max_depth(1) {
                    let entry = match entry {
                        Err(_) => continue,
                        Ok(entry) => entry,
                    };
                    if let Some(edition) =
                        is_edition(entry.path().file_name().and_then(|s| s.to_str()))
                    {
                        constructed = constructed
                            .join(edition)
                            .join("VC")
                            .join("Auxiliary")
                            .join("Build")
                            .join("vcvarsall.bat");

                        return Some(constructed);
                    }
                }
            }
        }
        None
    }

    let vs_2022_and_onwards_base = PathBuf::from("C:\\Program Files\\Microsoft Visual Studio\\");
    let vs_2019_and_2017_base = PathBuf::from("C:\\Program Files (x86)\\Microsoft Visual Studio\\");

    construct_path(&vs_2022_and_onwards_base).map_or_else(
        || construct_path(&vs_2019_and_2017_base).map_or_else(|| None, Some),
        Some,
    )
}

/// Determines the right argument to pass to `vcvarsall.bat` based on the host and target architectures.
///
/// Windows on ARM is not supporting 32 bit arm processors.
/// Because of this there is no native or cross compilation is supported for 32 bit arm processors.
fn determine_vcvarsall_bat_arch_arg() -> String {
    let host_architecture = std::env::consts::ARCH;
    let target_architecture = std::env::var("CARGO_CFG_TARGET_ARCH").expect("Target not set.");

    let arch_arg = if target_architecture == "x86_64" {
        if host_architecture == "x86" {
            // Arg for cross compilation from x86 to x64
            "x86_amd64"
        } else if host_architecture == "x86_64" {
            // Arg for native compilation from x64 to x64
            "amd64"
        } else if host_architecture == "aarch64" {
            // Arg for cross compilation from arm64 to amd64
            "arm64_amd64"
        } else {
            panic!("Unsupported host architecture {}", host_architecture);
        }
    } else if target_architecture == "x86" {
        if host_architecture == "x86" {
            // Arg for native compilation from x86 to x86
            "x86"
        } else if host_architecture == "x86_64" {
            // Arg for cross compilation from x64 to x86
            "amd64_x86"
        } else if host_architecture == "aarch64" {
            // Arg for cross compilation from arm64 to x86
            "arm64_x86"
        } else {
            panic!("Unsupported host architecture {}", host_architecture);
        }
    } else if target_architecture == "arm" {
        if host_architecture == "x86" {
            // Arg for cross compilation from x86 to arm
            "x86_arm"
        } else if host_architecture == "x86_64" {
            // Arg for cross compilation from x64 to arm
            "amd64_arm"
        } else if host_architecture == "aarch64" {
            // Arg for cross compilation from arm64 to arm
            "arm64_arm"
        } else {
            panic!("Unsupported host architecture {}", host_architecture);
        }
    } else if target_architecture == "aarch64" {
        if host_architecture == "x86" {
            // Arg for cross compilation from x86 to arm
            "x86_arm64"
        } else if host_architecture == "x86_64" {
            // Arg for cross compilation from x64 to arm
            "amd64_arm64"
        } else if host_architecture == "aarch64" {
            // Arg for native compilation from arm64 to arm64
            "arm64"
        } else {
            panic!("Unsupported host architecture {}", host_architecture);
        }
    } else {
        panic!("Unsupported target architecture.");
    };

    arch_arg.to_owned()
}
