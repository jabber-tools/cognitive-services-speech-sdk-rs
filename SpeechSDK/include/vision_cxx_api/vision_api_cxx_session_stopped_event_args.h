//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_details_session_stopped_event_args_base.h>
#include <vision_api_cxx_session_stopped_reason.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Session {
namespace Events {

/// <summary>
/// Represents inference processing stopping.
/// </summary>
class SessionStoppedEventArgs : private Core::Details::SessionStoppedEventArgsBase<Results::SessionStoppedReason, int>
{
private:

    using BaseEventArgs = Core::Details::SessionStoppedEventArgsBase<Results::SessionStoppedReason, int>;

public:

    /// <summary>
    /// Gets the unique Session ID from which this SessionResult originated.
    /// </summary>
    /// <returns>
    /// The Session ID string.
    /// </returns>
    SPXSTRING GetSessionId() const { return BaseEventArgs::GetSessionId(); }

    /// <summary>
    /// Gets the SessionStoppedReason for generation of this result.
    /// </summary>
    Results::SessionStoppedReason GetReason() const { return BaseEventArgs::GetReason(Results::SessionStoppedReason::Error, Results::SessionStoppedReason::StopRequested); }

protected:

    static std::shared_ptr<SessionStoppedEventArgs> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new SessionStoppedEventArgs(handle);
        return std::shared_ptr<SessionStoppedEventArgs>(ptr);
    }

    explicit SessionStoppedEventArgs(SPXHANDLE eventArgs) : SessionStoppedEventArgsBase(eventArgs)
    {
    }

private:

    DISABLE_DEFAULT_CTORS(SessionStoppedEventArgs);
};

} } } } } } // Azure::AI::Vision::Core::Session::Events
