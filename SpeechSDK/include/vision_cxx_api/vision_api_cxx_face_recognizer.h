//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <future>
#include <unordered_map>
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_event_signal.h>
#include <vision_api_cxx_details_recognizer_base.h>
#include <vision_api_cxx_face_recognizer_options.h>
#include <vision_api_cxx_face_recognizer_property.h>
#include <vision_api_cxx_face_result.h>
#include <vision_api_cxx_face_result_event_args.h>
#include <vision_api_cxx_face_session_started_event_args.h>
#include <vision_api_cxx_face_session_stopped_event_args.h>
#include <vision_api_cxx_properties.h>
#include <vision_api_cxx_session.h>
#include <vision_api_cxx_source.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Face {

// TODO: Update ref doc comments for FaceRecognizer and related

/// <summary>
/// Represents a set of Vision capabilities used for a finite period of time, with a given set of configuration, input, and options
/// </summary>
/// <remarks>Use FaceRecognizer::Create(service, input) to instantiate</remarks>
class FaceRecognizer :
    public std::enable_shared_from_this<FaceRecognizer>,
    private Core::Details::RecognizerBase<Face::FaceRecognizer,
        Options::FaceRecognizerOptions, Options::FaceRecognizerOption, Face::FaceRecognizerProperty,
        Results::FaceResult, Events::FaceResultEventArgs,
        Events::FaceSessionStartedEventArgs, Events::FaceSessionStoppedEventArgs>
{
protected:

    friend class Core::Details::RecognizerBase<Face::FaceRecognizer,
        Options::FaceRecognizerOptions, Options::FaceRecognizerOption, Face::FaceRecognizerProperty,
        Results::FaceResult, Events::FaceResultEventArgs,
        Events::FaceSessionStartedEventArgs, Events::FaceSessionStoppedEventArgs>;
    using BaseRecognizer = Core::Details::RecognizerBase<FaceRecognizer,
        Options::FaceRecognizerOptions, Options::FaceRecognizerOption, Face::FaceRecognizerProperty,
        Results::FaceResult, Events::FaceResultEventArgs,
        Events::FaceSessionStartedEventArgs, Events::FaceSessionStoppedEventArgs>;

public:

    /// <summary>
    /// Initializes a new instance of the FaceRecognizer class.
    /// </summary>
    /// <returns>The newly created FaceRecognizer wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<FaceRecognizer> Create(const std::shared_ptr<Service::VisionServiceConfig>& config, const std::shared_ptr<Input::VisionSource>& input, const std::shared_ptr<Options::FaceRecognizerOptions>& options = nullptr)
    {
        return BaseRecognizer::Create("face.recognizer", config, input, options);
    }

    /// <summary>
    /// Destructs an instance of the FaceRecognizer class.
    /// </summary>
    virtual ~FaceRecognizer() { }

    /// <summary>
    /// Gets the unique Session ID for this FaceRecognizer.
    /// </summary>
    /// <returns>
    /// The Session ID string.
    /// </returns>
    SPXSTRING GetSessionId() const { return BaseRecognizer::GetSessionId(); }

    /// <summary>
    /// Recognize one FaceResult from the input.
    /// </summary>
    /// <returns>The newly created FaceResult wrapped inside a std::shared_ptr</returns>
    std::shared_ptr<Results::FaceResult> RecognizeOnce() { return BaseRecognizer::RecognizeOnce(); }

    /// <summary>
    /// Recognize one FaceResult from the input, asynchronously.
    /// </summary>
    /// <returns>The future FaceResult wrapped inside a std::future<std::shared_ptr<>></returns>
    std::future<std::shared_ptr<Results::FaceResult>> RecognizeOnceAsync() { return BaseRecognizer::RecognizeOnceAsync(); }

    /// <summary>
    /// Starts recognizing Results from the input, continuously.
    /// </summary>
    void StartContinuousRecognition() { BaseRecognizer::StartContinuousRecognition(); }

    /// <summary>
    /// Starts recognizing Results from the input, continuously.
    /// </summary>
    /// <returns>A std::future<void> to be completed once continuous recognition has started</returns>
    std::future<void> StartContinuousRecognitionAsync() { return BaseRecognizer::StartContinuousRecognitionAsync(); }

    /// <summary>
    /// Stops recognizing Results from the input
    /// </summary>
    void StopContinuousRecognition() { BaseRecognizer::StopContinuousRecognition(); }

    /// <summary>
    /// Stops recognizing Results from the input, asynchronously.
    /// </summary>
    /// <returns>A std::future<void> to be completed once continuous recognition has stopped</returns>
    std::future<void> StopContinuousRecognitionAsync() { return BaseRecognizer::StopContinuousRecognitionAsync(); }

    /// <summary>
    /// Waits for recognition to stop
    /// </summary>
    /// <returns>true if the session stopped, false if not stopped after, the default timeout</returns>
    /// <remarks>WaitForStop does not initiate stopping. Call StopContinuousRecognition or similar to initiate stopping</remarks>
    bool WaitForStop() { return BaseRecognizer::WaitForStop(); }

    /// <summary>
    /// Waits for recognition to stop
    /// </summary>
    /// <returns>true if the session stopped, false if not stopped after timeout</returns>
    /// <remarks>WaitForStop does not initiate stopping. Call StopContinuousRecognition or similar to initiate stopping</remarks>
    bool WaitForStop(const std::chrono::milliseconds& timeout) { return BaseRecognizer::WaitForStop(timeout); }

    /// <summary>
    /// Waits for recognition to stop
    /// </summary>
    /// <returns>std::future<bool> representing the session stopping</returns>
    /// <remarks>WaitForStop does not initiate stopping. Call StopContinuousRecognition or similar to initiate stopping</remarks>
    std::future<void> WaitForStopAsync() { return BaseRecognizer::WaitForStopAsync(); }

    /// <summary>
    /// Signal for events indicating the start of a recognition session (operation).
    /// </summary>
    Core::Events::EventSignal<const Events::FaceSessionStartedEventArgs&>& SessionStarted;

    /// <summary>
    /// Signal for events indicating the end of a recognition session (operation).
    /// </summary>
    Core::Events::EventSignal<const Events::FaceSessionStoppedEventArgs&>& SessionStopped;

    /// <summary>
    /// Signal for events containing intermedia recognition operations.
    /// </summary>
    Core::Events::EventSignal<const Events::FaceResultEventArgs&>& Recognizing;

    /// <summary>
    /// Signal for events containing recognition operations.
    /// </summary>
    Core::Events::EventSignal<const Events::FaceResultEventArgs&>& Recognized;

    /// <summary>
    /// Gets a collection of additional inferencing operation properties.
    /// </summary>
    Core::PropertyCollection<FaceRecognizerProperty>& Properties;

protected:

    explicit FaceRecognizer(SPXHANDLE view) :
        RecognizerBase(view),
        SessionStarted(m_SessionStarted),
        SessionStopped(m_SessionStopped),
        Recognizing(m_Recognizing),
        Recognized(m_Recognized),
        Properties(GetProperties())
    {
    }

private:

    DISABLE_DEFAULT_CTORS(FaceRecognizer);
};

} } } } // Azure::AI::Vision::Face

PROMISE_CALLBACK_HELPER_STATICS(Azure::AI::Vision::Face::FaceRecognizer)
