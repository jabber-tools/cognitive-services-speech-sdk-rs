//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <future>
#include <unordered_map>
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_event_signal.h>
#include <vision_api_cxx_promise_callback_helper.h>
#include <vision_api_cxx_properties.h>
#include <vision_api_cxx_service_config.h>
#include <vision_api_cxx_session_options.h>
#include <vision_api_cxx_session_property.h>
#include <vision_api_cxx_session_result.h>
#include <vision_api_cxx_session_result_event_args.h>
#include <vision_api_cxx_session_started_event_args.h>
#include <vision_api_cxx_session_stopped_event_args.h>
#include <vision_api_cxx_source.h>
#include <vision_api_c_async_op.h>
#include <vision_api_c_session.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Session {

/// <summary>
/// Represents a set of Vision capabilities used for a finite period of time, with a given set of configuration, input, and options
/// </summary>
class VisionSession : public std::enable_shared_from_this<VisionSession>
{
protected:

    template<typename Target> using ProtectedAccess = Azure::AI::Vision::Core::Details::ProtectedAccess<Target>;
    Details::PrivatePropertyCollection<Session::VisionSessionProperty> m_properties;

public:

    /// <summary>
    /// Initializes a new instance of the VisionSession class.
    /// </summary>
    /// <returns>The newly created VisionSession wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<VisionSession> Create(const std::shared_ptr<Service::VisionServiceConfig>& config, const std::shared_ptr<Input::VisionSource>& input, const std::shared_ptr<Options::VisionSessionOptions>& options = nullptr)
    {
        auto allOptions = PropertyCollection<>::Create();
        SPXHANDLE combinedOptions = ProtectedAccess<PropertyCollection<>>::HandleFromPtr(allOptions.get());
        SPXHANDLE configHandle = ProtectedAccess<Service::VisionServiceConfig>::HandleFromPtr(config.get());
        SPXHANDLE optionsHandle = ProtectedAccess<Options::VisionSessionOptions>::HandleFromPtr(options.get());

        SPX_IFTRUE(configHandle != nullptr, SPX_THROW_ON_FAIL(ai_core_properties_copy(configHandle, combinedOptions)));
        SPX_IFTRUE(optionsHandle != nullptr, SPX_THROW_ON_FAIL(ai_core_properties_copy(optionsHandle, combinedOptions)));

        SPXHANDLE session = SPXHANDLE_INVALID;
        SPXHANDLE sourceHandle = ProtectedAccess<Input::VisionSource>::HandleFromPtr(input.get());
        SPX_THROW_ON_FAIL(::vision_session_handle_create(&session, combinedOptions, sourceHandle));
        auto ptr = FromHandle(session);

        SPX_THROW_ON_FAIL(vision_session_view_event_callback_set(session, "session.started", ptr.get(), SessionEventCallbackHandler));
        SPX_THROW_ON_FAIL(vision_session_view_event_callback_set(session, "session.stopped", ptr.get(), SessionEventCallbackHandler));
        SPX_THROW_ON_FAIL(vision_session_view_event_callback_set(session, "recognized", ptr.get(), SessionEventCallbackHandler));

        return ptr;
    }

    /// <summary>
    /// Destructs an instance of the VisionSession class.
    /// </summary>
    virtual ~VisionSession()
    {
        if (vision_session_handle_is_valid(m_session))
        {
            ::vision_session_handle_release(m_session);
            m_session = SPXHANDLE_INVALID;
        }
    };

    /// <summary>
    /// Gets the unique Session ID from which this VisionSession originated.
    /// </summary>
    /// <returns>
    /// The Session ID string.
    /// </returns>
    SPXSTRING GetSessionId() const
    {
        return m_properties.Get("session.id", "");
    }

    /// <summary>
    /// Recognize one SessionResult from the input.
    /// </summary>
    /// <returns>The newly created SessionResult wrapped inside a std::shared_ptr</returns>
    std::shared_ptr<Results::SessionResult> RecognizeOnce()
    {
        auto timeout = GetMillisecondTimeout("session.recognize.once", 60000);

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        SPX_THROW_ON_FAIL(vision_session_view_single_shot_start(m_session, nullptr, nullptr, &asyncOp));

        SPXHANDLE result = SPXHANDLE_INVALID;
        auto waited = async_op_wait_for_result(asyncOp, timeout, &result);
        SPX_THROW_ON_FAIL(async_op_handle_release(asyncOp));

        auto recognized = SPX_SUCCEEDED(waited)
            ? ProtectedAccess<Results::SessionResult>::FromHandle(result)
            : GetRecognizeOnceWaitFailedResult(waited);

        return recognized;
    }

    /// <summary>
    /// Recognize one SessionResult from the input, asynchronously.
    /// </summary>
    /// <returns>The future SessionResult wrapped inside a std::future<std::shared_ptr<>></returns>
    std::future<std::shared_ptr<Results::SessionResult>> RecognizeOnceAsync()
    {
        auto promiseResult = std::make_shared<std::promise<std::shared_ptr<Results::SessionResult>>>();
        auto promiseAsyncOp = std::make_shared<std::promise<SPXHANDLE>>();

        auto asyncOpCallback = m_promises.PromiseAsyncOpCallback(std::move(shared_from_this()), nullptr, [=](int /* id */, SPXHANDLE /* asyncOpCallback */) {

            auto asyncOp = promiseAsyncOp->get_future().get();

            SPXHANDLE result = SPXHANDLE_INVALID;
            auto waited = async_op_wait_for_result(asyncOp, 0, &result);
            SPX_REPORT_ON_FAIL(async_op_handle_release(asyncOp));

            auto recognized = SPX_SUCCEEDED(waited)
                ? ProtectedAccess<Results::SessionResult>::FromHandle(result)
                : GetRecognizeOnceWaitFailedResult(waited);

            promiseResult->set_value(recognized);
        });

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        auto check = vision_session_view_single_shot_start(m_session, nullptr, asyncOpCallback, &asyncOp);

        if (SPX_FAILED(check))
        {
            SPX_REPORT_ON_FAIL(async_op_callback_handle_release(asyncOpCallback));
            SPX_THROW_HR(check);
        }

        promiseAsyncOp->set_value(asyncOp);
        return promiseResult->get_future();
    }

    /// <summary>
    /// Starts recognizing Results from the input, continuously.
    /// </summary>
    void StartContinuousRecognition()
    {
        auto timeout = GetMillisecondTimeout("session.start.continuous.recognition", 60000);

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        SPX_THROW_ON_FAIL(vision_session_view_continuous_start(m_session, nullptr, nullptr, &asyncOp));

        auto waited = async_op_wait_for(asyncOp, timeout);
        SPX_THROW_ON_FAIL(async_op_handle_release(asyncOp));
        SPX_THROW_ON_FAIL(waited);
    }

    /// <summary>
    /// Starts recognizing Results from the input, continuously.
    /// </summary>
    /// <returns>A std::future<void> to be completed once continuous recognition has started</returns>
    std::future<void> StartContinuousRecognitionAsync()
    {
        auto promise = std::make_shared<std::promise<void>>();
        auto asyncOpCallback = m_promises.PromiseAsyncOpCallback(std::move(shared_from_this()), std::move(promise));

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        auto check = vision_session_view_continuous_start(m_session, nullptr, asyncOpCallback, &asyncOp);
        SPX_REPORT_ON_FAIL(async_op_handle_release(asyncOp));

        SPX_IFTRUE(SPX_FAILED(check), SPX_REPORT_ON_FAIL(async_op_callback_handle_release(asyncOpCallback)));
        SPX_THROW_ON_FAIL(check);

        return promise->get_future();
    }

    /// <summary>
    /// Stops recognizing Results from the input
    /// </summary>
    void StopContinuousRecognition()
    {
        auto timeout = GetMillisecondTimeout("session.stop.continuous.recognition", 60000);

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        SPX_THROW_ON_FAIL(vision_session_view_continuous_stop(m_session, nullptr, nullptr, &asyncOp));

        auto waited = async_op_wait_for(asyncOp, timeout);
        SPX_THROW_ON_FAIL(async_op_handle_release(asyncOp));
        SPX_THROW_ON_FAIL(waited);
    }

    /// <summary>
    /// Stops recognizing Results from the input, asynchronously.
    /// </summary>
    /// <returns>A std::future<void> to be completed once continuous recognition has stopped</returns>
    std::future<void> StopContinuousRecognitionAsync()
    {
        auto promise = std::make_shared<std::promise<void>>();
        auto asyncOpCallback = m_promises.PromiseAsyncOpCallback(std::move(shared_from_this()), std::move(promise));

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        auto check = vision_session_view_continuous_stop(m_session, nullptr, asyncOpCallback, &asyncOp);
        SPX_REPORT_ON_FAIL(async_op_handle_release(asyncOp));

        SPX_IFTRUE(SPX_FAILED(check), SPX_REPORT_ON_FAIL(async_op_callback_handle_release(asyncOpCallback)));
        SPX_THROW_ON_FAIL(check);

        return promise->get_future();
    }

    /// <summary>
    /// Waits for recognition to stop
    /// </summary>
    /// <returns>true if the session stopped, false if not stopped after, the default timeout</returns>
    /// <remarks>WaitForStop does not initiate stopping. Call StopContinuousRecognition or similar to initiate stopping</remarks>
    bool WaitForStop()
    {
        auto timeout = std::chrono::milliseconds(GetMillisecondTimeout("session.wait.for.stop", 60000));
        return WaitForStop(timeout);
    }

    /// <summary>
    /// Waits for recognition to stop
    /// </summary>
    /// <returns>true if the session stopped, false if not stopped after timeout</returns>
    /// <remarks>WaitForStop does not initiate stopping. Call StopContinuousRecognition or similar to initiate stopping</remarks>
    bool WaitForStop(const std::chrono::milliseconds& timeout)
    {
        auto future = WaitForStopAsync();
        auto status = future.wait_for(timeout);
        return status == std::future_status::ready;
    }

    /// <summary>
    /// Waits for recognition to stop
    /// </summary>
    /// <returns>std::future<bool> representing the session stopping</returns>
    /// <remarks>WaitForStop does not initiate stopping. Call StopContinuousRecognition or similar to initiate stopping</remarks>
    std::future<void> WaitForStopAsync()
    {
        auto stoppedPromise = std::make_shared<std::promise<void>>();
        m_promises.PromiseCallback(std::move(shared_from_this()), "session.stopped", std::move(stoppedPromise), [=](int id, SPXHANDLE /* handle */) {
            m_promises.RemovePromiseCallback(id, "session.stopped");
        });
        return stoppedPromise->get_future();
    }

    /// <summary>
    /// Signal for events indicating the start of a recognition session (operation).
    /// </summary>
    Core::Events::EventSignal<const Events::SessionStartedEventArgs&> SessionStarted;

    /// <summary>
    /// Signal for events indicating the end of a recognition session (operation).
    /// </summary>
    Core::Events::EventSignal<const Events::SessionStoppedEventArgs&> SessionStopped;

    /// <summary>
    /// Signal for events containing recognition operations.
    /// </summary>
    Core::Events::EventSignal<const Events::SessionResultEventArgs&> Recognized;

    /// <summary>
    /// Gets a collection of additional inferencing operation properties.
    /// </summary>
    PropertyCollection<Session::VisionSessionProperty>& Properties;

protected:

    static std::shared_ptr<VisionSession> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new VisionSession(handle);
        return std::shared_ptr<VisionSession>(ptr);
    }

    explicit VisionSession(SPXHANDLE session) :
        m_properties(session, [](auto handle, auto* properties) { return vision_session_properties_handle_get(handle, properties); }),
        Properties(m_properties),
        m_session(session)
    {
    }

    explicit operator SPXHANDLE() { return m_session; }

    uint32_t GetMillisecondTimeout(const char* prefix, int defaultMilliseconds)
    {
        auto name = std::string(prefix) + ".timeout.milliseconds";
        auto value = m_properties.Get(name, std::to_string(defaultMilliseconds));
        return std::stoi(value);
    }

    std::shared_ptr<Results::SessionResult> GetRecognizeOnceWaitFailedResult(SPXHR waited)
    {
        auto timeout = GetMillisecondTimeout("session.recognize.once.wait.failed.stop.timeout", 2000);

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        SPX_REPORT_ON_FAIL(vision_session_view_single_shot_stop(m_session, nullptr, nullptr, &asyncOp));
        SPX_REPORT_ON_FAIL(async_op_wait_for(asyncOp, timeout));
        SPX_REPORT_ON_FAIL(async_op_handle_release(asyncOp));

        return GetWaitFailedResult(waited);
    }

    std::shared_ptr<Results::SessionResult> GetWaitFailedResult(SPXHR waited)
    {
        return waited == SPXERR_TIMEOUT
            ? GetWaitTimedoutResult()
            : GetWaitErrorResult(waited);
    }

    std::shared_ptr<Results::SessionResult> GetWaitTimedoutResult()
    {
        // TODO: build timeout result appropriately
        return std::shared_ptr<Results::SessionResult>(nullptr);
    }

    std::shared_ptr<Results::SessionResult> GetWaitErrorResult(SPXHR waited)
    {
        UNUSED(waited); // TODO: build failed result appropriately
        return std::shared_ptr<Results::SessionResult>(nullptr);
    }

    static void SessionEventCallbackHandler(SPXHANDLE session, const char* name, void* context, SPXHANDLE eventArgs)
    {
        auto ptr = static_cast<VisionSession*>(context);
        ptr->m_promises.CompletePromiseCallbacks(name, eventArgs);

        SPX_IFTRUE(strcmp(name, "session.started") == 0, ptr->SessionStartedCallback(session, context, eventArgs));
        SPX_IFTRUE(strcmp(name, "session.stopped") == 0, ptr->SessionStoppedCallback(session, context, eventArgs));
        SPX_IFTRUE(strcmp(name, "recognized") == 0, ptr->RecognizedCallback(session, context, eventArgs));
    }

    void SessionStartedCallback(SPXHANDLE session, void* context, SPXHANDLE eventArgs)
    {
        UNUSED(session); UNUSED(context);

        auto startedBefore = m_started.exchange(true);
        SPX_DBG_ASSERT(!startedBefore); UNUSED(startedBefore);

        auto connected = SessionStarted.IsConnected();
        SPX_IFTRUE_RETURN(!connected);

        auto ptr = ProtectedAccess<Events::SessionStartedEventArgs>::FromHandle(eventArgs);
        SessionStarted.Signal(*ptr.get());
    }

    void SessionStoppedCallback(SPXHANDLE session, void* context, SPXHANDLE eventArgs)
    {
        UNUSED(session); UNUSED(context);

        auto startedBefore = m_started.exchange(false);
        SPX_DBG_ASSERT(startedBefore); UNUSED(startedBefore);

        auto connected = SessionStopped.IsConnected();
        SPX_IFTRUE_RETURN(!connected);

        auto ptr = ProtectedAccess<Events::SessionStoppedEventArgs>::FromHandle(eventArgs);
        SessionStopped.Signal(*ptr.get());
    }

    void RecognizedCallback(SPXHANDLE session, void* context, SPXHANDLE eventArgs)
    {
        UNUSED(session); UNUSED(context);

        auto connected = Recognized.IsConnected();
        SPX_IFTRUE_RETURN(!connected);

        auto ptr = ProtectedAccess<Events::SessionResultEventArgs>::FromHandle(eventArgs);
        Recognized.Signal(*ptr.get());
    }

private:

    DISABLE_DEFAULT_CTORS(VisionSession);

    SPXHANDLE m_session;
    std::atomic_bool m_started { false };
    Core::Details::PromiseCallbackHelper<VisionSession> m_promises;
};

} } } } } // Azure::AI::Vision::Core::Session


PROMISE_CALLBACK_HELPER_STATICS(Azure::AI::Vision::Core::Session::VisionSession)
