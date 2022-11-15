# cognitive-services-speech-sdk-rs

---
[![License](https://img.shields.io/badge/License-Apache-blue.svg)](LICENSE-APACHE)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![Crates.io][crates-badge]][crates-url]
[![rustdoc][rustdoc-badge]][rustdoc-url]
[![CI](https://github.com/jabber-tools/cognitive-services-speech-sdk-rs/actions/workflows/github-actions-rust-ci.yml/badge.svg)](https://github.com/jabber-tools/cognitive-services-speech-sdk-rs/actions/workflows/github-actions-rust-ci.yml)

[crates-badge]: https://img.shields.io/crates/v/cognitive-services-speech-sdk-rs.svg
[crates-url]: https://crates.io/crates/cognitive-services-speech-sdk-rs
[rustdoc-badge]: https://img.shields.io/badge/rustdoc-0.2.2-green.svg
[rustdoc-url]: https://jabber-tools.github.io/cognitive_services_speech_sdk_rs/doc/0.2.0/cognitive_services_speech_sdk_rs/index.html

Rust bindings for Microsoft Cognitive Speech Services SDK. Provides thin abstraction around native C API. Heavily inspired by official [Go library](https://github.com/microsoft/cognitive-services-speech-sdk-go). Provides speech-to-text, text-to-speech and bot framework dialog management capabilities. 

Pull requests welcome!

### Speech to text 
```rust
use cognitive_services_speech_sdk_rs as msspeech;
use log::*;
use std::env;

async fn speech_to_text() {
    let filename = env::var("WAVFILENAME").unwrap();
    let audio_config = msspeech::audio::AudioConfig::from_wav_file_input(&filename).unwrap();

    let speech_config = msspeech::speech::SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();
    let mut speech_recognizer =
        msspeech::speech::SpeechRecognizer::from_config(speech_config, audio_config).unwrap();

    speech_recognizer
        .set_session_started_cb(|event| info!("set_session_started_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_session_stopped_cb(|event| info!("set_session_stopped_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_speech_start_detected_cb(|event| info!("set_speech_start_detected_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_speech_end_detected_cb(|event| info!("set_speech_end_detected_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_recognizing_cb(|event| info!("set_recognizing_cb {:?}", event.result.text))
        .unwrap();

    speech_recognizer
        .set_recognized_cb(|event| info!("set_recognized_cb {:?}", event))
        .unwrap();

    speech_recognizer
        .set_canceled_cb(|event| info!("set_canceled_cb {:?}", event))
        .unwrap();

    let result = speech_recognizer.recognize_once_async().await.unwrap();
    info!("got recognition {:?}", result);
}
```

### Text to speech
```rust
use cognitive_services_speech_sdk_rs as msspeech;
use log::*;
use std::env;

async fn text_to_speech() {
    let pull_stream = msspeech::audio::PullAudioOutputStream::create_pull_stream().unwrap();
    let audio_config = msspeech::audio::AudioConfig::from_stream_output(&pull_stream).unwrap();

    let speech_config = msspeech::speech::SpeechConfig::from_subscription(
        env::var("MSSubscriptionKey").unwrap(),
        env::var("MSServiceRegion").unwrap(),
    )
    .unwrap();
    let mut speech_synthesizer =
        msspeech::speech::SpeechSynthesizer::from_config(speech_config, audio_config).unwrap();

    speech_synthesizer
        .set_synthesizer_started_cb(|event| info!("synthesizer_started_cb {:?}", event))
        .unwrap();

    speech_synthesizer
        .set_synthesizer_synthesizing_cb(|event| info!("synthesizer_synthesizing_cb {:?}", event))
        .unwrap();

    speech_synthesizer
        .set_synthesizer_completed_cb(|event| info!("synthesizer_completed_cb {:?}", event))
        .unwrap();

    speech_synthesizer
        .set_synthesizer_canceled_cb(|event| info!("synthesizer_canceled_cb {:?}", event))
        .unwrap();

    match speech_synthesizer.speak_text_async("Hello Rust!").await {
        Err(err) => error!("speak_text_async error {:?}", err),
        Ok(speech_audio_bytes) => {
            info!("speech_audio_bytes {:?}", speech_audio_bytes);
        }
    }
}
```

For more see github integration tests (*tests* folder) and samples (*examples* folder).

## Build prerequisites

Currently only build on Linux is supported. Uses Clang and Microsoft Speech SDK shared libraries. Details can be found here [here](https://docs.microsoft.com/en-us/azure/cognitive-services/speech-service/quickstarts/setup-platform?tabs=dotnet%2Cwindows%2Cjre%2Cbrowser&pivots=programming-language-go).

Install following prerequisites before running *cargo build*:

```
sudo apt-get update 
sudo apt-get install clang build-essential libssl1.0.0 libasound2 wget
```

Build is generating Rust bindings for Speech SDK native functions. These are already prebuilt and put into *ffi/bindings.rs* file. In most cases it is not necessary to regenerate them. Set following to skip bindings regeneration:

```
export MS_COG_SVC_SPEECH_SKIP_BINDGEN=1
cargo build
```

## Added in this version

This version (0.2.0) brings following goodies:

* Build support for ARM architecture.
* Upgrade of Microsoft Speech SDK version to 1.22.0.
* Preview of Embedded Speech Config (Details [here](https://docs.microsoft.com/en-us/cpp/cognitive-services/speech/embeddedspeechconfig)). See also *examples/recognizer/embedded_recognize_once_async_from_file*. 
  *EmbeddedSpeechConfig* class is not yet available in public release (there are no tutorials/doc available how to create embedded speech models for this API) but Microsoft will be revealing this information in the near future (initially for selected customers only). 
  This will hopefully make possible to run embedded speech models (possibly on ARM devices) in offline mode emerging some very interesting applications of this library.

Version 0.2.1 brings on the top of that support for build on MacOs (target architecture **aarch64**), see below.

Version 0.2.2 adds MacOS support for target architecture **arm**.

### How To Build On MacOS

We are supporting MacOS **arm** and **aarch64** architectures.

In order to build on MacOS, download respective binaries of MS Speech SDK(v1.23.0) from [here](https://www.dropbox.com/s/w78qg20r60dm5ar/MicrosoftCognitiveServicesSpeech-XCFramework-1.23.0.zip?dl=0). You can also download the latest MacOS Speech SDK from [Microsoft page](https://aka.ms/csspeech/macosbinary) but this will be the latest version of MS Speech SDK which might be not tested and working well with current version of **cognitive-services-speech-sdk-rs**.

Once downloaded, extract the content of the zip file (subfolder **MicrosoftCognitiveServicesSpeech.xcframework/macos-arm64_x86_64**) into dedicated folder, e.g. **/Users/xxx/speechsdk**. The content of the directory should look as follows:

```
➜  cd /Users/xxx/speechsdk 
➜  speechsdk ls -la
total 416
drwxr-xr-x   6 xxx  staff     192 Sep 17 19:55 .
drwxr-x---+ 66 xxx  staff    2112 Sep 17 23:21 ..
drwxr-xr-x   7 xxx  staff     224 Sep 17 17:15 MicrosoftCognitiveServicesSpeech.xcframework
-rw-r--r--   1 xxx  staff    1582 Jul 26 11:10 REDIST.txt
-rw-r--r--   1 xxx  staff  191072 Jul 26 11:10 ThirdPartyNotices.md
-rw-r--r--   1 xxx  staff   14893 Jul 26 11:10 license.md
➜  speechsdk
```

Run following commands to build:
```
export MACOS_SPEECHSDK_ROOT=/Users/xxx/speechsdk
cargo build
```

Speech SDK libraries are linked dynamically during build and run. When running the application use following environment variable to point to custom library location:

```
export DYLD_FALLBACK_FRAMEWORK_PATH=/Users/xxx/speechsdk/MicrosoftCognitiveServicesSpeech.xcframework/macos-arm64_x86_64
```

Then run your application utilizing **cognitive-services-speech-sdk-rs** or examples e.g.:
```
cargo run --example recognizer
```
