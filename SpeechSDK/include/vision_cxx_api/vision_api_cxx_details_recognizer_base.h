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
#include <vision_api_cxx_session.h>
#include <vision_api_cxx_source.h>
#include <vision_api_c_async_op.h>
#include <vision_api_c_session.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Details {

template <typename TRecognizer,
          typename TOptions,
          typename TOptionKey,
          typename TPropertyKey,
          typename TResult,
          typename TResultEventArgs,
          typename TStartedEventArgs,
          typename TStoppedEventArgs>
class RecognizerBase
{
protected:

    template<typename Target> using ProtectedAccess = Azure::AI::Vision::Core::Details::ProtectedAccess<Target>;
    PrivatePropertyCollection<TPropertyKey> m_properties;

public:

    static std::shared_ptr<TRecognizer> Create(const char* viewKind, const std::shared_ptr<Service::VisionServiceConfig>& config, const std::shared_ptr<Input::VisionSource>& input, const std::shared_ptr<TOptions>& options = nullptr)
    {
        // create the session
        auto session = Core::Session::VisionSession::Create(config, input);
        auto sessionHandle = ProtectedAccess<Core::Session::VisionSession>::HandleFromPtr(session.get());

        // create the view (aka Recognizer)
        SPXHANDLE view = SPXHANDLE_INVALID;
        SPXHANDLE optionsHandle = ProtectedAccess<TOptions>::HandleFromPtr(options.get());
        SPX_THROW_ON_FAIL(vision_session_view_handle_create(&view, sessionHandle, viewKind, optionsHandle));
        auto ptr = TRecognizer::FromHandle(view);
        ptr->m_session = session;

        // hook up the event handlers
        SPX_THROW_ON_FAIL(vision_session_view_event_callback_set(view, "session.started", ptr.get(), SessionEventCallbackHandler));
        SPX_THROW_ON_FAIL(vision_session_view_event_callback_set(view, "session.stopped", ptr.get(), SessionEventCallbackHandler));
        SPX_THROW_ON_FAIL(vision_session_view_event_callback_set(view, "recognized", ptr.get(), SessionEventCallbackHandler));

        return ptr;
    }

    virtual ~RecognizerBase()
    {
        if (vision_session_view_handle_is_valid(m_recognizerView))
        {
            ::vision_session_view_handle_release(m_recognizerView);
            m_recognizerView = SPXHANDLE_INVALID;
        }

        m_session.reset();
    };

    SPXSTRING GetSessionId() const
    {
        return m_session->GetSessionId();
    }

    std::shared_ptr<TResult> RecognizeOnce()
    {
        auto timeout = GetMillisecondTimeout("session.recognize.once", 60000);

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        SPX_THROW_ON_FAIL(vision_session_view_single_shot_start(m_recognizerView, nullptr, nullptr, &asyncOp));

        SPXHANDLE result = SPXHANDLE_INVALID;
        auto waited = async_op_wait_for_result(asyncOp, timeout, &result);
        SPX_THROW_ON_FAIL(async_op_handle_release(asyncOp));

        auto recognized = SPX_SUCCEEDED(waited)
            ? ProtectedAccess<TResult>::FromHandle(result)
            : GetRecognizeOnceWaitFailedResult(waited);

        return recognized;
    }

    std::future<std::shared_ptr<TResult>> RecognizeOnceAsync()
    {
        auto promiseResult = std::make_shared<std::promise<std::shared_ptr<TResult>>>();
        auto promiseAsyncOp = std::make_shared<std::promise<SPXHANDLE>>();

        auto ptr = static_cast<TRecognizer*>(this);
        auto asyncOpCallback = m_promises.PromiseAsyncOpCallback(ptr->shared_from_this(), nullptr, [=](int /* id */, SPXHANDLE /* asyncOpCallback */) {

            auto asyncOp = promiseAsyncOp->get_future().get();

            SPXHANDLE result = SPXHANDLE_INVALID;
            auto waited = async_op_wait_for_result(asyncOp, 0, &result);
            SPX_REPORT_ON_FAIL(async_op_handle_release(asyncOp));

            auto recognized = SPX_SUCCEEDED(waited)
                ? ProtectedAccess<TResult>::FromHandle(result)
                : GetRecognizeOnceWaitFailedResult(waited);

            promiseResult->set_value(recognized);
        });

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        auto check = vision_session_view_single_shot_start(m_recognizerView, nullptr, asyncOpCallback, &asyncOp);

        if (SPX_FAILED(check))
        {
            SPX_REPORT_ON_FAIL(async_op_callback_handle_release(asyncOpCallback));
            SPX_THROW_HR(check);
        }

        promiseAsyncOp->set_value(asyncOp);
        return promiseResult->get_future();
    }

    void StartContinuousRecognition()
    {
        auto timeout = GetMillisecondTimeout("session.start.continuous.recognition", 60000);

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        SPX_THROW_ON_FAIL(vision_session_view_continuous_start(m_recognizerView, nullptr, nullptr, &asyncOp));

        auto waited = async_op_wait_for(asyncOp, timeout);
        SPX_THROW_ON_FAIL(async_op_handle_release(asyncOp));
        SPX_THROW_ON_FAIL(waited);
    }

    std::future<void> StartContinuousRecognitionAsync()
    {
        auto promise = std::make_shared<std::promise<void>>();

        auto ptr = static_cast<TRecognizer*>(this);
        auto asyncOpCallback = m_promises.PromiseAsyncOpCallback(ptr->shared_from_this(), std::move(promise));

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        auto check = vision_session_view_continuous_start(m_recognizerView, nullptr, asyncOpCallback, &asyncOp);
        SPX_REPORT_ON_FAIL(async_op_handle_release(asyncOp));

        SPX_IFTRUE(SPX_FAILED(check), SPX_REPORT_ON_FAIL(async_op_callback_handle_release(asyncOpCallback)));
        SPX_THROW_ON_FAIL(check);

        return promise->get_future();
    }

    void StopContinuousRecognition()
    {
        auto timeout = GetMillisecondTimeout("session.stop.continuous.recognition", 60000);

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        SPX_THROW_ON_FAIL(vision_session_view_continuous_stop(m_recognizerView, nullptr, nullptr, &asyncOp));

        auto waited = async_op_wait_for(asyncOp, timeout);
        SPX_THROW_ON_FAIL(async_op_handle_release(asyncOp));
        SPX_THROW_ON_FAIL(waited);
    }

    std::future<void> StopContinuousRecognitionAsync()
    {
        auto promise = std::make_shared<std::promise<void>>();

        auto ptr = static_cast<TRecognizer*>(this);
        auto asyncOpCallback = m_promises.PromiseAsyncOpCallback(ptr->shared_from_this(), std::move(promise));

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        auto check = vision_session_view_continuous_stop(m_recognizerView, nullptr, asyncOpCallback, &asyncOp);
        SPX_REPORT_ON_FAIL(async_op_handle_release(asyncOp));

        SPX_IFTRUE(SPX_FAILED(check), SPX_REPORT_ON_FAIL(async_op_callback_handle_release(asyncOpCallback)));
        SPX_THROW_ON_FAIL(check);

        return promise->get_future();
    }

    bool WaitForStop()
    {
        auto timeout = std::chrono::milliseconds(GetMillisecondTimeout("session.wait.for.stop", 60000));
        return WaitForStop(timeout);
    }

    bool WaitForStop(const std::chrono::milliseconds& timeout)
    {
        auto future = WaitForStopAsync();
        auto status = future.wait_for(timeout);
        return status == std::future_status::ready;
    }

    std::future<void> WaitForStopAsync()
    {
        auto stoppedPromise = std::make_shared<std::promise<void>>();

        auto ptr = static_cast<TRecognizer*>(this);
        m_promises.PromiseCallback(ptr->shared_from_this(), "session.stopped", std::move(stoppedPromise), [=](int id, SPXHANDLE /* handle */) {
            m_promises.RemovePromiseCallback(id, "session.stopped");
        });

        return stoppedPromise->get_future();
    }

    Core::Events::EventSignal<const TStartedEventArgs&> m_SessionStarted;
    Core::Events::EventSignal<const TStoppedEventArgs&> m_SessionStopped;
    Core::Events::EventSignal<const TResultEventArgs&> m_Recognizing;
    Core::Events::EventSignal<const TResultEventArgs&> m_Recognized;

    PropertyCollection<TPropertyKey>& GetProperties() { return m_properties; }
    const PropertyCollection<TPropertyKey>& GetProperties() const { return m_properties; }

public:

    static std::shared_ptr<TRecognizer> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new TRecognizer(handle);
        return std::shared_ptr<TRecognizer>(ptr);
    }

    explicit RecognizerBase(SPXHANDLE view) :
        m_properties(view, [](auto handle, auto* properties) { return vision_session_view_properties_handle_get(handle, properties); }),
        m_recognizerView(view)
    {
    }

    explicit operator SPXHANDLE() { return m_recognizerView; }

protected:

    uint32_t GetMillisecondTimeout(const char* prefix, int defaultMilliseconds)
    {
        auto name = std::string(prefix) + ".timeout.milliseconds";
        auto value = m_properties.Get(name, std::to_string(defaultMilliseconds));
        return std::stoi(value);
    }

    std::shared_ptr<TResult> GetRecognizeOnceWaitFailedResult(SPXHR waited)
    {
        auto timeout = GetMillisecondTimeout("session.recognize.once.wait.failed.stop.timeout", 2000);

        SPXHANDLE asyncOp = SPXHANDLE_INVALID;
        SPX_REPORT_ON_FAIL(vision_session_view_single_shot_stop(m_recognizerView, nullptr, nullptr, &asyncOp));
        SPX_REPORT_ON_FAIL(async_op_wait_for(asyncOp, timeout));
        SPX_REPORT_ON_FAIL(async_op_handle_release(asyncOp));

        return GetWaitFailedResult(waited);
    }

    std::shared_ptr<TResult> GetWaitFailedResult(SPXHR waited)
    {
        return waited == SPXERR_TIMEOUT
            ? GetWaitTimedoutResult()
            : GetWaitErrorResult(waited);
    }

    std::shared_ptr<TResult> GetWaitTimedoutResult()
    {
        // TODO: build timeout result appropriately
        return std::shared_ptr<TResult>(nullptr);
    }

    std::shared_ptr<TResult> GetWaitErrorResult(SPXHR waited)
    {
        UNUSED(waited); // TODO: build failed result appropriately
        return std::shared_ptr<TResult>(nullptr);
    }

    static void SessionEventCallbackHandler(SPXHANDLE session, const char* name, void* context, SPXHANDLE eventArgs)
    {
        auto ptr = static_cast<TRecognizer*>(context);
        ptr->m_promises.CompletePromiseCallbacks(name, eventArgs);

        SPX_IFTRUE(strcmp(name, "session.started") == 0, ptr->SessionStartedCallback(session, context, eventArgs));
        SPX_IFTRUE(strcmp(name, "session.stopped") == 0, ptr->SessionStoppedCallback(session, context, eventArgs));
        SPX_IFTRUE(strcmp(name, "recognizing") == 0, ptr->RecognizingCallback(session, context, eventArgs));
        SPX_IFTRUE(strcmp(name, "recognized") == 0, ptr->RecognizedCallback(session, context, eventArgs));
    }

    void SessionStartedCallback(SPXHANDLE session, void* context, SPXHANDLE eventArgs)
    {
        UNUSED(session); UNUSED(context);

        auto startedBefore = m_started.exchange(true);
        SPX_DBG_ASSERT(!startedBefore); UNUSED(startedBefore);

        auto connected = m_SessionStarted.IsConnected();
        SPX_IFTRUE_RETURN(!connected);

        auto ptr = ProtectedAccess<TStartedEventArgs>::FromHandle(eventArgs);
        m_SessionStarted.Signal(*ptr.get());
    }

    void SessionStoppedCallback(SPXHANDLE session, void* context, SPXHANDLE eventArgs)
    {
        UNUSED(session); UNUSED(context);

        auto startedBefore = m_started.exchange(false);
        SPX_DBG_ASSERT(startedBefore); UNUSED(startedBefore);

        auto connected = m_SessionStopped.IsConnected();
        SPX_IFTRUE_RETURN(!connected);

        auto ptr = ProtectedAccess<TStoppedEventArgs>::FromHandle(eventArgs);
        m_SessionStopped.Signal(*ptr.get());
    }

    void RecognizingCallback(SPXHANDLE session, void* context, SPXHANDLE eventArgs)
    {
        UNUSED(session); UNUSED(context);

        auto connected = m_Recognizing.IsConnected();
        SPX_IFTRUE_RETURN(!connected);

        auto ptr = ProtectedAccess<TResultEventArgs>::FromHandle(eventArgs);
        m_Recognizing.Signal(*ptr.get());
    }

    void RecognizedCallback(SPXHANDLE session, void* context, SPXHANDLE eventArgs)
    {
        UNUSED(session); UNUSED(context);

        auto connected = m_Recognized.IsConnected();
        SPX_IFTRUE_RETURN(!connected);

        auto ptr = ProtectedAccess<TResultEventArgs>::FromHandle(eventArgs);
        m_Recognized.Signal(*ptr.get());
    }

private:

    DISABLE_DEFAULT_CTORS(RecognizerBase);

    SPXHANDLE m_recognizerView;
    std::shared_ptr<Core::Session::VisionSession> m_session;

    std::atomic_bool m_started { false };
    Core::Details::PromiseCallbackHelper<TRecognizer> m_promises;
};

} } } } } // Azure::AI::Vision::Core::Details
