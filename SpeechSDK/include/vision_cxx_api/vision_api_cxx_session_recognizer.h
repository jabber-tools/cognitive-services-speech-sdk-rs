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
#include <vision_api_cxx_properties.h>
#include <vision_api_cxx_session.h>
#include <vision_api_cxx_session_options.h>
#include <vision_api_cxx_session_recognizer_property.h>
#include <vision_api_cxx_session_result.h>
#include <vision_api_cxx_session_result_event_args.h>
#include <vision_api_cxx_session_started_event_args.h>
#include <vision_api_cxx_session_stopped_event_args.h>
#include <vision_api_cxx_source.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Session {

/// <summary>
/// Represents a set of Vision capabilities used for a finite period of time, with a given set of configuration, input, and options
/// </summary>
class VisionSessionRecognizer :
    public std::enable_shared_from_this<VisionSessionRecognizer>,
    private Details::RecognizerBase<VisionSessionRecognizer,
        Options::VisionSessionOptions, Options::VisionSessionOption, Session::VisionSessionRecognizerProperty,
        Results::SessionResult, Events::SessionResultEventArgs,
        Events::SessionStartedEventArgs, Events::SessionStoppedEventArgs>
{
protected:

    friend class Details::RecognizerBase<VisionSessionRecognizer,
        Options::VisionSessionOptions, Options::VisionSessionOption, Session::VisionSessionRecognizerProperty,
        Results::SessionResult, Events::SessionResultEventArgs,
        Events::SessionStartedEventArgs, Events::SessionStoppedEventArgs>;
    using BaseRecognizer = Details::RecognizerBase<VisionSessionRecognizer,
        Options::VisionSessionOptions, Options::VisionSessionOption, Session::VisionSessionRecognizerProperty,
        Results::SessionResult, Events::SessionResultEventArgs,
        Events::SessionStartedEventArgs, Events::SessionStoppedEventArgs>;

public:

    /// <summary>
    /// Initializes a new instance of the VisionSessionRecognizer class.
    /// </summary>
    /// <returns>The newly created VisionSessionRecognizer wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<VisionSessionRecognizer> Create(const std::shared_ptr<Service::VisionServiceConfig>& config, const std::shared_ptr<Input::VisionSource>& input, const std::shared_ptr<Options::VisionSessionOptions>& options = nullptr)
    {
        return BaseRecognizer::Create("session.recognizer", config, input, options);
    }

    /// <summary>
    /// Destructs an instance of the VisionSessionRecognizer class.
    /// </summary>
    virtual ~VisionSessionRecognizer() { }

    /// <summary>
    /// Gets the unique Session ID from which this VisionSession originated.
    /// </summary>
    /// <returns>
    /// The Session ID string.
    /// </returns>
    SPXSTRING GetSessionId() const { return BaseRecognizer::GetSessionId(); }

    /// <summary>
    /// Recognize one SessionResult from the input.
    /// </summary>
    /// <returns>The newly created SessionResult wrapped inside a std::shared_ptr</returns>
    std::shared_ptr<Results::SessionResult> RecognizeOnce() { return BaseRecognizer::RecognizeOnce(); }

    /// <summary>
    /// Recognize one SessionResult from the input, asynchronously.
    /// </summary>
    /// <returns>The future SessionResult wrapped inside a std::future<std::shared_ptr<>></returns>
    std::future<std::shared_ptr<Results::SessionResult>> RecognizeOnceAsync() { return BaseRecognizer::RecognizeOnceAsync(); }

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
    Core::Events::EventSignal<const Events::SessionStartedEventArgs&>& SessionStarted;

    /// <summary>
    /// Signal for events indicating the end of a recognition session (operation).
    /// </summary>
    Core::Events::EventSignal<const Events::SessionStoppedEventArgs&>& SessionStopped;

    /// <summary>
    /// Signal for events containing recognition operations.
    /// </summary>
    Core::Events::EventSignal<const Events::SessionResultEventArgs&>& Recognized;

    /// <summary>
    /// Gets a collection of additional inferencing operation properties.
    /// </summary>
    Core::PropertyCollection<Session::VisionSessionRecognizerProperty>& Properties;

protected:

    explicit VisionSessionRecognizer(SPXHANDLE view) :
        RecognizerBase(view),
        SessionStarted(m_SessionStarted),
        SessionStopped(m_SessionStopped),
        Recognized(m_Recognized),
        Properties(GetProperties())
    {
    }

private:

    DISABLE_DEFAULT_CTORS(VisionSessionRecognizer);
};

} } } } } // Azure::AI::Vision::Core::Session

PROMISE_CALLBACK_HELPER_STATICS(Azure::AI::Vision::Core::Session::VisionSessionRecognizer)
