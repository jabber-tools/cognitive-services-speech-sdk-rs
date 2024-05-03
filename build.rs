use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    str,
};

use flate2::read::GzDecoder;

// If you update this, run a build with MS_COG_SVC_SPEECH_UPDATE_BINDINGS=1
// to update the bindings.rs file.
const SPEECH_SDK_VERSION: &str = "1.37.0";

fn main() {
    if env_var("DOCS_RS").is_some() {
        // Skip linking and bindgen when building docs as docs.rs won't have the
        // dependency present and can't download it.
        return;
    }

    let link_params = if let Some(lib_dir) = env_var("MS_COG_SVC_SPEECH_LIB_DIR") {
        // If the user has specified an SDK lib directory then use that and
        // don't try and download another one.
        let include_dir = env_var("MS_COG_SVC_SPEECH_INCLUDE_DIR").map(PathBuf::from);
        match TargetOs::get() {
            TargetOs::Linux => LinkParams {
                lib_arg: "dylib=Microsoft.CognitiveServices.Speech.core".to_owned(),
                search_arg: format!("native={}", lib_dir.to_str().unwrap()),
                include_dir,
            },
            TargetOs::MacOS => LinkParams {
                lib_arg: "framework=MicrosoftCognitiveServicesSpeech".to_owned(),
                search_arg: format!("framework={}", lib_dir.to_str().unwrap()),
                include_dir,
            },
            TargetOs::Windows => LinkParams {
                lib_arg: "dylib=Microsoft.CognitiveServices.Speech.core".to_owned(),
                search_arg: format!("native={}", lib_dir.to_str().unwrap()),
                include_dir,
            },
        }
    } else {
        let download_dir = {
            let mut dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
            dir.push("sdk_output");
            dir
        };
        // Don't bother re-downloading the SDK if it looks like it's already
        // there, unless it's been explicitly requested.
        if env_var("MS_COG_SVC_SPEECH_RENEW_SDK").is_some()
            || !download_dir.exists()
            || fs::read_dir(&download_dir).unwrap().next().is_none()
        {
            fs::create_dir_all(&download_dir).unwrap();
            download_sdk(&download_dir);
        }
        let sdk_dir = match TargetOs::get() {
            TargetOs::Linux => download_dir.join(format!("SpeechSDK-Linux-{}", SPEECH_SDK_VERSION)),
            TargetOs::MacOS => download_dir.join("MicrosoftCognitiveServicesSpeech.xcframework"),
            TargetOs::Windows => download_dir,
        };
        link_params(&sdk_dir)
    };

    println!("cargo:rustc-link-search={}", link_params.search_arg);
    println!("cargo:rustc-link-lib={}", link_params.lib_arg);

    if env_var("MS_COG_SVC_SPEECH_SKIP_BINDGEN").is_some() {
        return;
    }
    println!("cargo::rustc-cfg=bindgen");

    let include_dir = match link_params.include_dir {
        Some(include_dir) => include_dir,
        _ => {
            // Note this is only possible to hit if the user has specified the lib dir.
            panic!("Include directory must be specified with MS_COG_SVC_SPEECH_INCLUDE_DIR");
        }
    };
    let bindings_builder = bindgen::Builder::default()
        .header("c_api/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg(format!("-I{}", include_dir.to_str().unwrap()));

    #[cfg(any(
        all(target_os = "linux", any(target_arch = "arm", target_arch = "aarch64")),
        target_os = "macos",
    ))]
    let bindings_builder = bindings_builder
        // use gcc to find correct include path for stddef.h
        .clang_arg(
            std::process::Command::new("gcc")
                .arg("--print-file-name=include")
                .output()
                .map(|o| format!("-I{}", String::from_utf8_lossy(&o.stdout).trim()))
                .unwrap(),
        );

    let bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    if env_var("MS_COG_SVC_SPEECH_UPDATE_BINDINGS").is_some() {
        bindings
            .write_to_file("src/ffi/bindings.rs")
            .expect("Unable to update bindings");
    }
}

/// Download the SDK from the official Microsoft source into the given path,
/// the file structure depends on the target OS.
fn download_sdk(path: &Path) {
    let zip_url = match TargetOs::get() {
        TargetOs::Linux => {
            let sdk_url = format!(
                "https://csspeechstorage.blob.core.windows.net/drop/{SPEECH_SDK_VERSION}/SpeechSDK-Linux-{SPEECH_SDK_VERSION}.tar.gz");
            let sdk = ureq::get(&sdk_url)
                .call()
                .expect("Failed to download Speech SDK");
            let mut archive = tar::Archive::new(GzDecoder::new(sdk.into_reader()));
            for entry in archive.entries().unwrap() {
                entry.unwrap().unpack_in(path).unwrap();
            }
            return;
        },
        TargetOs::MacOS => format!("https://csspeechstorage.blob.core.windows.net/drop/{SPEECH_SDK_VERSION}/MicrosoftCognitiveServicesSpeech-MacOSXCFramework-{SPEECH_SDK_VERSION}.zip"),
        TargetOs::Windows => format!("https://www.nuget.org/api/v2/package/Microsoft.CognitiveServices.Speech/{SPEECH_SDK_VERSION}"),
    };
    let mut sdk = ureq::get(&zip_url)
        .call()
        .expect("Failed to download Speech SDK")
        .into_reader();
    while let Some(mut zip_file) = zip::read::read_zipfile_from_stream(&mut sdk).unwrap() {
        if zip_file.is_dir() {
            fs::create_dir_all(path.join(zip_file.enclosed_name().unwrap())).unwrap();
        } else {
            let file_path = path.join(zip_file.enclosed_name().unwrap());
            fs::create_dir_all(file_path.parent().unwrap()).unwrap();
            let mut file = fs::File::create(file_path).unwrap();
            std::io::copy(&mut zip_file, &mut file).unwrap();
        }
    }
}

struct LinkParams {
    lib_arg: String,
    search_arg: String,
    include_dir: Option<PathBuf>,
}

/// Get the link parameters for the Speech SDK based on the target OS and architecture.
fn link_params(sdk_dir: &Path) -> LinkParams {
    match TargetOs::get() {
        TargetOs::Linux => {
            let mut lib_dir = sdk_dir.join("lib");
            if cfg!(target_arch = "x86") {
                lib_dir.push("x86")
            } else if cfg!(target_arch = "x86_64") {
                lib_dir.push("x64")
            } else if cfg!(target_arch = "arm") {
                lib_dir.push("arm32")
            } else if cfg!(target_arch = "aarch64") {
                lib_dir.push("arm64")
            } else {
                panic!("Unsupported target architecture; only x86, x86_64, arm, and aarch64 are supported on Linux.")
            }
            LinkParams {
                lib_arg: "dylib=Microsoft.CognitiveServices.Speech.core".to_owned(),
                search_arg: format!("native={}", lib_dir.to_str().unwrap()),
                include_dir: Some(sdk_dir.join("include/c_api")),
            }
        }
        TargetOs::MacOS => LinkParams {
            lib_arg: "framework=MicrosoftCognitiveServicesSpeech".to_owned(),
            search_arg: format!(
                "framework={}",
                sdk_dir.join("macos-arm64_x86_64").to_str().unwrap()
            ),
            include_dir: Some(
                sdk_dir
                    .join("macos-arm64_x86_64/MicrosoftCognitiveServicesSpeech.framework/Headers"),
            ),
        },
        TargetOs::Windows => {
            let mut lib_dir = sdk_dir.join("build/native");
            if cfg!(target_arch = "x86") {
                lib_dir.push("Win32")
            } else if cfg!(target_arch = "x86_64") {
                lib_dir.push("x64")
            } else if cfg!(target_arch = "arm") {
                lib_dir.push("ARM")
            } else if cfg!(target_arch = "aarch64") {
                lib_dir.push("ARM64")
            } else {
                panic!("Unsupported target architecture; only x86, x86_64, arm, and aarch64 are supported on Windows.")
            }
            lib_dir.push("Release");
            LinkParams {
                lib_arg: "dylib=Microsoft.CognitiveServices.Speech.core".to_owned(),
                search_arg: format!("native={}", lib_dir.to_str().unwrap()),
                include_dir: Some(sdk_dir.join("build/native/include/c_api")),
            }
        }
    }
}

/// Get an environment variable and register it with cargo so the build is rerun
/// if it's changed.
fn env_var(name: &str) -> Option<OsString> {
    println!("cargo::rerun-if-env-changed={name}");
    std::env::var_os(name)
}

enum TargetOs {
    Linux,
    MacOS,
    Windows,
}

impl TargetOs {
    const fn get() -> TargetOs {
        if cfg!(target_os = "linux") {
            TargetOs::Linux
        } else if cfg!(target_os = "macos") {
            TargetOs::MacOS
        } else if cfg!(target_os = "windows") {
            TargetOs::Windows
        } else {
            panic!("Unsupported target OS; only linux, macos, and windows are supported.")
        }
    }
}
