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
namespace Face {
namespace Events {

/// <summary>
/// Represents inference processing starting.
/// </summary>
class FaceSessionStartedEventArgs : private Core::Details::SessionEventArgsBase<int>
{
private:

    using BaseEventArgs = Core::Details::SessionEventArgsBase<int>;

public:

    /// <summary>
    /// Gets the unique id for the Session from which this Event originated.
    /// </summary>
    /// <returns>
    /// The Session ID string.
    /// </returns>
    SPXSTRING GetSessionId() const { return BaseEventArgs::GetSessionId(); }

protected:

    static std::shared_ptr<FaceSessionStartedEventArgs> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new FaceSessionStartedEventArgs(handle);
        return std::shared_ptr<FaceSessionStartedEventArgs>(ptr);
    }

    explicit FaceSessionStartedEventArgs(SPXHANDLE eventArgs) : SessionEventArgsBase(eventArgs)
    {
    }

private:

    DISABLE_DEFAULT_CTORS(FaceSessionStartedEventArgs);
};

} } } } } // Azure::AI::Vision::Face::Events
