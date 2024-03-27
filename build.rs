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
    let sdk_path = parent_dir.join("speech");
    if !sdk_path.exists() {
        renew = true;
        fs::create_dir_all(&sdk_path).unwrap();
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
            sdk_path.to_str().unwrap(),
        ];
        Command::new("tar").args(args).status().unwrap();
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

#[cfg(any(
    all(target_os = "macos", target_arch = "aarch64"),
    all(target_os = "macos", target_arch = "arm"),
    all(target_os = "macos", target_arch = "x86_64")
))]
fn main() {
    let mac_sdk_url = format!("https://csspeechstorage.blob.core.windows.net/drop/1.36.0/MicrosoftCognitiveServicesSpeech-MacOSXCFramework-{SPEECH_SDK_VERSION}.zip");

    let parent_dir = PathBuf::from("./SpeechSDK").join("macOS");
    if !parent_dir.exists() {
        fs::create_dir_all(&parent_dir).unwrap();
    }

    let mut renew = env::var("RENEW_SDK").map(|v| v == "1").unwrap_or(false);
    let sdk_path = parent_dir.join("speech");
    if !sdk_path.exists() {
        renew = true;
        fs::create_dir_all(&sdk_path).unwrap();
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
            sdk_path.to_str().unwrap(), // The directory to extract to
        ];
        Command::new("unzip").args(args).status().unwrap();
    }

    println!("cargo:rustc-link-search=framework={}/MicrosoftCognitiveServicesSpeech.xcframework/macos-arm64_x86_64", sdk_path.display());
    println!("cargo:rustc-link-lib=framework=MicrosoftCognitiveServicesSpeech");

    let inc_arg = format!("-I{}/MicrosoftCognitiveServicesSpeech.xcframework/macos-arm64_x86_64/MicrosoftCognitiveServicesSpeech.framework/Headers", sdk_path.display());

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

#[cfg(target_os = "windows")]
fn main() {
    use std::{fs::File, io::BufReader};

    use zip::ZipArchive;

    let nuget_package_url = format!("https://www.nuget.org/api/v2/package/Microsoft.CognitiveServices.Speech/{SPEECH_SDK_VERSION}");

    let parent_dir = PathBuf::from("./SpeechSDK").join("windows");
    if !parent_dir.exists() {
        fs::create_dir_all(&parent_dir).unwrap();
    }

    let mut renew = env::var("RENEW_SDK").map(|v| v == "1").unwrap_or(false);
    let sdk_path = parent_dir.join("speech");
    if !sdk_path.exists() {
        renew = true;
        fs::create_dir_all(&sdk_path).unwrap();
    }

    let sdk_zip_file = parent_dir.join(format!(
        "microsoft.cognitiveservices.speech.{SPEECH_SDK_VERSION}.nupkg"
    ));
    if !sdk_zip_file.exists() {
        download_file(nuget_package_url.as_str(), sdk_zip_file.to_str().unwrap());
    }

    if renew {
        let reader = File::open(sdk_zip_file).unwrap();
        let mut archive = ZipArchive::new(BufReader::new(reader)).unwrap();

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = sdk_path.join(file.mangled_name());

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath).unwrap();
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p).unwrap();
                    }
                }
                let mut outfile = std::fs::File::create(&outpath).unwrap();
                std::io::copy(&mut file, &mut outfile).unwrap();
            }
        }
    }

    let native = sdk_path.join("build").join("native");

    #[cfg(target_arch = "x86")]
    let lib_path = native.join("Win32").join("Release");
    #[cfg(target_arch = "x86_64")]
    let lib_path = native.join("x64").join("Release");
    #[cfg(target_arch = "arm")]
    let lib_path = native.join("ARM").join("Release");
    #[cfg(target_arch = "aarch64")]
    let lib_path = native.join("ARM64").join("Release");

    let mut inc_arg = String::from("-I");
    inc_arg.push_str(native.join("include").join("c_api").to_str().unwrap());

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
