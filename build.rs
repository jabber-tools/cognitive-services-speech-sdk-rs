// build.rs
#[allow(unused_imports)]
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    str,
};

#[cfg(not(target_os = "macos"))]
const LINUX_SDK_URL: &str  = "https://github.com/jabber-tools/cognitive-services-speech-sdk-rs-files/blob/main/SpeechSDK/1.22.0/linux/SpeechSDK-Linux-1.22.0.tar.gz?raw=true";

#[cfg(not(target_os = "macos"))]
fn download_file(url: &str, dst: &str) {
    Command::new("curl")
        .args(&["-SL", url, "-o", dst])
        .status()
        .expect("failed to download Speech SDK!");
}

#[cfg(not(target_os = "macos"))]
fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // copying SpeechSDK from local folder just worked fine but crates.io allows
    // only 10 MB to be uploaded per crate. Because of that SpeechSDK shared libraries
    // were moved to separate repository (https://github.com/jabber-tools/cognitive-services-speech-sdk-rs-files)
    // and must be downloaded during build from there.
    /*
    fs::copy(
        "./SpeechSDK/lib/x64/libMicrosoft.CognitiveServices.Speech.core.so",
        Path::new(&out_path).join("libMicrosoft.CognitiveServices.Speech.core.so"),
    )
    .unwrap();
    fs::copy(
        "./SpeechSDK/lib/x64/libMicrosoft.CognitiveServices.Speech.extension.kws.so",
        Path::new(&out_path).join("libMicrosoft.CognitiveServices.Speech.extension.kws.so"),
    )
    .unwrap();
    fs::copy(
        "./SpeechSDK/lib/x64/libMicrosoft.CognitiveServices.Speech.extension.codec.so",
        Path::new(&out_path).join("libMicrosoft.CognitiveServices.Speech.extension.codec.so"),
    )
    .unwrap();

    println!("cargo:rustc-link-search=native={}", out_path.display());
    println!("cargo:rustc-link-lib=dylib=Microsoft.CognitiveServices.Speech.core");
    */

    let mut renew = env::var("RENEW_SDK").map(|v| v == "1").unwrap_or(false);
    let sdk_path = out_path.join("SpeechSDK").join("linux");
    if !sdk_path.exists() {
        renew = true;
        fs::create_dir_all(&sdk_path).unwrap();
    }

    if renew {
        let dw_file = out_path.join("linux.sdk");
        let sdk_file = dw_file.to_str().unwrap();
        download_file(LINUX_SDK_URL, sdk_file);
        let args = [
            "--strip",
            "1",
            "-xzf",
            sdk_file,
            "-C",
            sdk_path.to_str().unwrap(),
        ];
        Command::new("tar").args(&args).status().unwrap();
    }

    #[cfg(target_arch = "x86")]
    let lib_path = sdk_path.join("lib").join("x86");
    #[cfg(target_arch = "x86_64")]
    let lib_path = sdk_path.join("lib").join("x64");
    #[cfg(target_arch = "arm")]
    let lib_path = sdk_path.join("lib").join("arm32");
    #[cfg(target_arch = "aarch64")]
    let lib_path = sdk_path.join("lib").join("arm64");

    let mut inc_arg = String::from("-I");
    inc_arg.push_str(sdk_path.join("include").join("c_api").to_str().unwrap());

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
        // The input header we would like to generate
        // bindings for.
        .header("c_api/wrapper.h")
        // use line below when building from local SpeechSDK folder
        //.clang_arg("-ISpeechSDK/include/c_api/")
        // use this line when building from downloaded files in OUT_DIR
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

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file("src/ffi/bindings.rs")
        .expect("Couldn't write bindings!");
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn main() {
    let speek_sdk_root = env::var("MACOS_SPEECHSDK_ROOT").expect(
        "Set environment variable MACOS_SPEECHSDK_ROOT with location of MS Speech SDK library.",
    );

    println!("cargo:rustc-link-search=framework={}/MicrosoftCognitiveServicesSpeech.xcframework/macos-arm64_x86_64", speek_sdk_root);
    println!("cargo:rustc-link-lib=framework=MicrosoftCognitiveServicesSpeech");

    let inc_arg = format!("-I{}/MicrosoftCognitiveServicesSpeech.xcframework/macos-arm64_x86_64/MicrosoftCognitiveServicesSpeech.framework/Headers", speek_sdk_root);

    let bindings_builder = bindgen::Builder::default()
        .header("c_api/wrapper.h")
        .clang_arg(inc_arg);

    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
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
