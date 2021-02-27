//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once

#include <vision_api_cxx_common.h>
#include <vision_api_cxx_enums.h>
#include <vision_api_cxx_event_signal.h>
#include <vision_api_cxx_face_recognizer.h>
#include <vision_api_cxx_face_recognizer_options.h>
#include <vision_api_cxx_face_result.h>
#include <vision_api_cxx_face_result_event_args.h>
#include <vision_api_cxx_frame_format.h>
#include <vision_api_cxx_frame_source.h>
#include <vision_api_cxx_frame_writer.h>
#include <vision_api_cxx_properties.h>
#include <vision_api_cxx_session_options.h>
#include <vision_api_cxx_session_recognizer.h>
#include <vision_api_cxx_session_result.h>
#include <vision_api_cxx_session_result_event_args.h>
#include <vision_api_cxx_session_result_reason.h>
#include <vision_api_cxx_service_config.h>
#include <vision_api_cxx_session_started_event_args.h>
#include <vision_api_cxx_session_stopped_event_args.h>
#include <vision_api_cxx_session.h>
#include <vision_api_cxx_session_options.h>
#include <vision_api_cxx_source.h>

/*

=== Azure.Core (owned by AzSDK team)

=== Auzre.AI.Core (owned by us... for whatever we want)

=== Azure.AI.Vision.Core

=== Azure.AI.Vision.Core.Details

    ✔ PromiseCallbackHelper<> (namespaced, vision_cxx_api_promise_callback_helper.h)
    ✔ RecognizerBase<> (renamed, namespaced, templatized, vision_cxx_api_recognizer.h)
    ✔ Need common place for single copy of things like ProtectedAccess, PrivatePropertyCollection,

    === Events
    ✔ EventArgsBase<TPropKey=int> (renamed, namespaced, vision_cxx_api_event_args.h)
    ✔ ResultEventArgsBase<TResult, TPropKey=int> (renamed, namespaced, vision_cxx_api_recognition_event_args.h)
    ✔ SessionEventArgsBase<TPropKey=int> (new, namespaced, refactored from vision_api_cxx_session_started_event_args.h)

    === Results
    ✔ ResultBase<TReason,TPropKey=int> (renamed, namespaced, templatized, vision_cxx_api_result.h)

=== Azure.AI.Vision.Core.Session

    ✔ VisionSessionRecognizer (new, inherits from Details::RecognizerBase<VisionRecognizerOptions, VisionRecognizerOption>)
    ✔ VisionSession (namespaced, vision_cxx_api_session.h)

    === Events
    ✔ SessionResultEventArgs
    ✔ SessionStartedEventArgs
    ✔ SessionStoppedEventArgs
    ✔ SessionResultEventArgs inherits from Details::ResultEventArgsBase<SessionResult, int/VisionResultProperties>
    ✔ SessionStartedEventArgs inherits from Details::SessionEventArgsBase<int/VisionSessionProperties>
    ✔ SessionStoppedEventArgs inherits from Details::SessionStoppedEventArgsBase<int/VisionSessionProperties>

    === Results
    ◻ TODO: VisionErrorDetails
    ◻ TODO: VisionErrorReason
    ◻ TODO: VisionNoMatchDetails
    ◻ TODO: VisionNoMatchReason
    ✔ SessionResult (new, inherits from Details::ResultBase<VisionResultReason, int>)
    ✔ SessionResultReason (renamed, namespaced, vision_cxx_api_result_reason.h)
    ✔ SessionStoppedReason (renamed, namespaced, vision_api_cxx_session_stopped_reason.h)

    === Options
    ✔ VisionRecognizerOptions (moved/copied, namespaced, vision_api_cxx_session_options.h))
    ✔ VisionRecognizerOption (moved/copied, namespaced, vision_api_cxx_session_options.h))
    ✔ VisionRecognizerOption::AdvancedOptions (moved, namespaced, vision_api_cxx_session_options.h))
    ✔ VisionSessionOptions (moved, namespaced, vision_api_cxx_session_options.h)
    ✔ VisionSessionOption (moved, namespaced, vision_api_cxx_session_options.h)

=== Azure.AI.Vision.Input

    ✔ VisionSource (namespaced, vision_cxx_api_source.h)

    ✔ FrameFormatProperty enum class, switch PropertyCollection<...>
    ✔ FrameSourceProperty enum class, switch PropertyCollection<...>

    === Devices
    ◻ 🧩 TODO: future use ...

    === Frames
    ✔ FrameWriter (namespaced, vision_cxx_api_frame_writer.h)
    ✔ FrameSource (namespaced, vision_cxx_api_frame_source.h)
    ✔ FrameFormat (namespaced, vision_cxx_api_frame_format.h)

    === Streams
    ◻ 🧩 TODO: StreamSource
    ◻ 🧩 TODO: StreamWriter
    ◻ 🧩 TODO: StreamFormat

    === Remote
    ◻ 🧩 TODO: future use ...

=== Azure.AI.Vision.Service

    ✔ VisionServiceConfig
    ✔ VisionServiceOption
    ✔ VisionServiceOptionsAdvancedOptions

=== Azure.AI.Vision.Face

    ✔ FaceRecognizer (new, inherits from Details::RecognizerBase<...>)

    === Events
    ✔ FaceResultEventArgs
    ✔ FaceResultEventArgs inherits from Details::ResultEventArgsBase<FaceResult, int>
    ✔ FaceResultEventArgs inherits from Details::ResultEventArgsBase<FaceResult, int/FaceResultProperty>
    ✔ FaceSessionStartedEventArgs (new, inherits from Details::SessionEventArgsBase<int/FaceRecognizerProperties>)
    ✔ FaceSessionStoppedEventArgs (new, inherits from Details::SessionStoppedEventArgsBase<int/FaceRecognizerProperties>)

    === Results
    ✔ FaceResult (new, inherits from Details::ResultBase<VisionResultReason, FaceResultProperty>)
    ✔ FaceResultReason (new, namespaced, copied from vision_cxx_api_result_reason.h)
    ✔ FaceSessionStoppedReason (copied, vision_api_cxx_session_stopped_reason.h)

    ◻ TODO: ... future stuff... FaceCollection
    ◻ TODO: ... future stuff... RecognizedFace...

    === Options
    ✔ FaceRecognizerAdvancedOptions (moved/copied, namespaced, vision_api_cxx_session_options.h))
    ✔ FaceRecognizerOptions (moved/copied, namespaced, vision_api_cxx_session_options.h))
    ✔ FaceRecognizerOption (moved/copied, namespaced, vision_api_cxx_session_options.h))

=== Azure.AI.Vision.Prototype

    ◻ TODO: PixelRecognizer (new, inherits from Details::RecognizerBase<...>)

    === Events
    ◻ TODO: PixelEventArgs
    ◻ TODO: PixelResultEventArgs
    ◻ 🧩 TODO: PixelEventArgs inherits from Details::EventArgsBase<int>
    ◻ 🧩 TODO: PixelResultEventArgs inherits from Details::ResultEventArgsBase<VisionResult>
    ◻ 🧩 TODO: PixelRecognizerStartedEventArgs (new, inherits from Details::SessionEventArgsBase<int/PixelRecognizerProperties>)
    ◻ 🧩 TODO: PixelRecognizerStoppedEventArgs (new, inherits from Details::SessionEventArgsBase<int/PixelRecognizerProperties>)

    === Results
    ◻ TODO: PixelResult (new, inherits from Details::ResultBase<VisionResultReason, int>)
    ◻ TODO: PixelResultReason (new, namespaced, copied from vision_cxx_api_result_reason.h)
    ◻ TODO: PixelRecognizerStoppedReason (copied, vision_api_cxx_session_stopped_reason.h)

    ◻ TODO: ... future stuff... PixelCollection
    ◻ TODO: ... future stuff... RecognizedPixel...

    === Options
    ◻ TODO: PixelRecognizerAdvancedOptions (moved/copied, namespaced, vision_api_cxx_session_options.h))
    ◻ TODO: PixelRecognizerOptions (moved/copied, namespaced, vision_api_cxx_session_options.h))
    ◻ TODO: PixelRecognizerOption (moved/copied, namespaced, vision_api_cxx_session_options.h))

*/
