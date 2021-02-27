//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_details_session_event_args_base.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Session {
namespace Events {

/// <summary>
/// Represents inference processing starting.
/// </summary>
class SessionStartedEventArgs : private Core::Details::SessionEventArgsBase<int>
{
private:

    using BaseEventArgs = Core::Details::SessionEventArgsBase<int>;

public:

    /// <summary>
    /// Gets the unique id for the Session from which this SessionResult originated.
    /// </summary>
    /// <returns>
    /// The Session ID string.
    /// </returns>
    SPXSTRING GetSessionId() const { return BaseEventArgs::GetSessionId(); }

protected:

    static std::shared_ptr<SessionStartedEventArgs> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new SessionStartedEventArgs(handle);
        return std::shared_ptr<SessionStartedEventArgs>(ptr);
    }

    explicit SessionStartedEventArgs(SPXHANDLE eventArgs) : SessionEventArgsBase(eventArgs)
    {
    }

private:

    DISABLE_DEFAULT_CTORS(SessionStartedEventArgs);
};

} } } } } } // Azure::AI::Vision::Core::Session::Events
