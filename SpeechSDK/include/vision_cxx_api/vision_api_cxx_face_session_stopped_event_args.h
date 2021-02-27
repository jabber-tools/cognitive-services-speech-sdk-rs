//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_details_session_stopped_event_args_base.h>
#include <vision_api_cxx_face_session_stopped_reason.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Face {
namespace Events {

/// <summary>
/// Represents inference processing stopping.
/// </summary>
class FaceSessionStoppedEventArgs : private Core::Details::SessionStoppedEventArgsBase<Results::FaceSessionStoppedReason, int>
{
private:

    using BaseEventArgs = Core::Details::SessionStoppedEventArgsBase<Results::FaceSessionStoppedReason, int>;

public:

    /// <summary>
    /// Gets the unique Session ID from which this Event originated.
    /// </summary>
    /// <returns>
    /// The Session ID string.
    /// </returns>
    SPXSTRING GetSessionId() const { return BaseEventArgs::GetSessionId(); }

    /// <summary>
    /// Gets the SessionStoppedReason for generation of this result.
    /// </summary>
    Results::FaceSessionStoppedReason GetReason() const { return BaseEventArgs::GetReason(Results::FaceSessionStoppedReason::Error, Results::FaceSessionStoppedReason::StopRequested); }

protected:

    static std::shared_ptr<FaceSessionStoppedEventArgs> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new FaceSessionStoppedEventArgs(handle);
        return std::shared_ptr<FaceSessionStoppedEventArgs>(ptr);
    }

    explicit FaceSessionStoppedEventArgs(SPXHANDLE eventArgs) : SessionStoppedEventArgsBase(eventArgs)
    {
    }

private:

    DISABLE_DEFAULT_CTORS(FaceSessionStoppedEventArgs);
};

} } } } } // Azure::AI::Vision::Face::Events
