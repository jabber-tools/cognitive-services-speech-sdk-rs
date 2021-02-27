//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_details_event_args_base.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Details {

template<typename TPropKey>
class SessionEventArgsBase : public EventArgsBase<TPropKey>
{
public:

    SPXSTRING GetSessionId() const
    {
        return EventArgsBase<TPropKey>::Get("session.id", "");
    }

protected:

    explicit SessionEventArgsBase(SPXHANDLE eventArgs) : EventArgsBase<TPropKey>(eventArgs)
    {
    }

private:

    DISABLE_DEFAULT_CTORS(SessionEventArgsBase);
};

} } } } } // Azure::AI::Vision::Core::Details
