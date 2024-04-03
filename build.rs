// build.rs
#[allow(unused_imports)]
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    str,
};

const SPEECH_SDK_VERSION: &str = "1.36.0";

fn download_file(url: &str, dst: &str) {
    Command::new("curl")
        .args(["-SL", url, "-o", dst])
        .status()
        .expect("failed to download Speech SDK!");
}

#[cfg(target_os = "linux")]
fn main() {
    let linux_sdk_url = format!(
        "https://csspeechstorage.blob.core.windows.net/drop/{SPEECH_SDK_VERSION}/SpeechSDK-Linux-{SPEECH_SDK_VERSION}.tar.gz");

    let parent_dir = PathBuf::from("./SpeechSDK").join("linux");
    if !parent_dir.exists() {
        fs::create_dir_all(&parent_dir).unwrap();
    }

    let mut renew = env::var("RENEW_SDK").map(|v| v == "1").unwrap_or(false);
    let sdk_output_dir = parent_dir.join("sdk_output");
    if !sdk_output_dir.exists() || fs::read_dir(&sdk_output_dir).unwrap().next().is_none() {
        renew = true;
        fs::create_dir_all(&sdk_output_dir).unwrap();
    }

    let sdk_tar_file = parent_dir.join(format!("SpeechSDK-Linux-{SPEECH_SDK_VERSION}.tar.gz"));
    if !sdk_tar_file.exists() {
        download_file(linux_sdk_url.as_str(), sdk_tar_file.to_str().unwrap());
    }

    if renew {
        let args = [
            "--strip",
            "1",
            "-xzf",
            sdk_tar_file.to_str().unwrap(),
            "-C",
            sdk_output_dir.to_str().unwrap(),
        ];
        Command::new("tar").args(args).status().unwrap();
    }

    #[cfg(target_arch = "x86")]
    let lib_path = sdk_path.join("lib").join("x86");
    #[cfg(target_arch = "x86_64")]
    let lib_path = sdk_output_dir.join("lib").join("x64");
    #[cfg(target_arch = "arm")]
    let lib_path = sdk_path.join("lib").join("arm32");
    #[cfg(target_arch = "aarch64")]
    let lib_path = sdk_path.join("lib").join("arm64");

    let mut inc_arg = String::from("-I");
    inc_arg.push_str(
        sdk_output_dir
            .join("include")
            .join("c_api")
            .to_str()
            .unwrap(),
    );

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=dylib=Microsoft.CognitiveServices.Speech.core");

    let skip_bindgen = env::var("MS_COG_SVC_SPEECH_SKIP_BINDGEN")
        .map(|v| v == "1")
        .unwrap_or(false);

    if skip_bindgen {
        return;
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings_builder = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("c_api/wrapper.h")
        .clang_arg(inc_arg.as_str());

    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    let bindings_builder = bindings_builder
        // use gcc to find correct include path for stddef.h
        .clang_arg(
            Command::new("gcc")
                .arg("--print-file-name=include")
                .output()
                .map(|o| format!("-I{}", String::from_utf8_lossy(&o.stdout).trim()))
                .unwrap(),
        );

    // Finish the builder and generate the bindings.
    let bindings = bindings_builder
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the src/ffi/bindings.rs file.
    bindings
        .write_to_file("src/ffi/bindings.rs")
        .expect("Couldn't write bindings!");
}

#[cfg(any(
    all(target_os = "macos", target_arch = "aarch64"),
    all(target_os = "macos", target_arch = "arm"),
    all(target_os = "macos", target_arch = "x86_64")
))]
fn main() {
    let mac_sdk_url = format!("https://csspeechstorage.blob.core.windows.net/drop/{SPEECH_SDK_VERSION}/MicrosoftCognitiveServicesSpeech-MacOSXCFramework-{SPEECH_SDK_VERSION}.zip");

    let parent_dir = PathBuf::from("./SpeechSDK").join("macOS");
    if !parent_dir.exists() {
        fs::create_dir_all(&parent_dir).unwrap();
    }

    let mut renew = env::var("RENEW_SDK").map(|v| v == "1").unwrap_or(false);
    let sdk_output_dir = parent_dir.join("sdk_output");
    if !sdk_output_dir.exists() || fs::read_dir(&sdk_output_dir).unwrap().next().is_none() {
        renew = true;
        fs::create_dir_all(&sdk_output_dir).unwrap();
    }

    let sdk_zip_file = parent_dir.join(format!(
        "MicrosoftCognitiveServicesSpeech-MacOSXCFramework-{SPEECH_SDK_VERSION}.zip"
    ));
    if !sdk_zip_file.exists() {
        download_file(mac_sdk_url.as_str(), sdk_zip_file.to_str().unwrap());
    }

    if renew {
        let args = [
            "-o",                           // Overwrite files without prompting
            sdk_zip_file.to_str().unwrap(), // The zip file
            "-d",
            sdk_output_dir.to_str().unwrap(), // The directory to extract to
        ];
        Command::new("unzip").args(args).status().unwrap();
    }

    println!("cargo:rustc-link-search=framework={}/MicrosoftCognitiveServicesSpeech.xcframework/macos-arm64_x86_64", sdk_output_dir.display());
    println!("cargo:rustc-link-lib=framework=MicrosoftCognitiveServicesSpeech");

    let inc_arg = format!("-I{}/MicrosoftCognitiveServicesSpeech.xcframework/macos-arm64_x86_64/MicrosoftCognitiveServicesSpeech.framework/Headers", sdk_output_dir.display());

    let bindings_builder = bindgen::Builder::default()
        .header("c_api/wrapper.h")
        .clang_arg(inc_arg);

    let bindings_builder = bindings_builder.clang_arg(
        Command::new("gcc")
            .arg("--print-file-name=include")
            .output()
            .map(|o| format!("-I{}", String::from_utf8_lossy(&o.stdout).trim()))
            .unwrap(),
    );

    let bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/ffi/bindings.rs")
        .expect("Couldn't write bindings!");
}
