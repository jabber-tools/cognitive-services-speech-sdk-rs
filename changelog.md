# change log

This version (0.2.0) brings following goodies:

* Build support for ARM architecture.
* Upgrade of Microsoft Speech SDK version to 1.22.0.
* Preview of Embedded Speech Config (Details [here](https://docs.microsoft.com/en-us/cpp/cognitive-services/speech/embeddedspeechconfig)). See also *examples/recognizer/embedded_recognize_once_async_from_file*. 
  *EmbeddedSpeechConfig* class is not yet available in public release (there are no tutorials/doc available how to create embedded speech models for this API) but Microsoft will be revealing this information in the near future (initially for selected customers only). 
  This will hopefully make possible to run embedded speech models (possibly on ARM devices) in offline mode emerging some very interesting applications of this library.

Version 0.2.1 brings on the top of that support for build on MacOs (target architecture **aarch64**), see below.

Version 0.2.2 adds MacOS support for target architecture **arm**.

Version 0.3.0 upgrades to MS Speech SDK 1.37.0 and improves library build process.

Version 0.3.1 windows support!

Version 1.0.0 Some internal stability fixes. All **from_handle** methods are now unsafe. This is breaking change as it requires to call **from_handle** within unsafe block.

Version 1.0.1 Implemented trait **std::error::Error** for **cognitive_services_speech_sdk_rs::error::Error** so that it works well with **anyhow::Result<T>**. See [PR16](https://github.com/jabber-tools/cognitive-services-speech-sdk-rs/pull/16).

Version 1.0.2 Extended PropertyId with missing values

Version 1.0.3 documentation moved to docs.rs

Version 1.0.4 minor fixes in readme files (badges, etc.), integration test **speech_to_text** made more resilient, assertion improved

Version 1.0.5 SpeechSynthesisWordBoundaryEvent extended with **text** property.

Version 1.0.6 Minor fixes: fix panic when getting list of voices, linter fixes (dangling_pointers_from_temporaries), use From trait instead of custom to_u32, to _i32 methods. See [PR26](https://github.com/jabber-tools/cognitive-services-speech-sdk-rs/pull/26) and [PR28](https://github.com/jabber-tools/cognitive-services-speech-sdk-rs/pull/28).