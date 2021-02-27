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
namespace Details {

template<typename TStoppedReason, typename TPropKey>
class SessionStoppedEventArgsBase : public SessionEventArgsBase<TPropKey>
{
public:

    TStoppedReason GetReason(TStoppedReason min, TStoppedReason max) const
    {
        auto property = SessionEventArgsBase<TPropKey>::Get("session.stopped.reason", "0");
        auto value = std::stoi(property.c_str());
        SPX_IFTRUE_THROW_HR(value < (int)min || value > (int)max, SPXERR_INVALID_RESULT_REASON);
        return (TStoppedReason)value;
    }

protected:

    explicit SessionStoppedEventArgsBase(SPXHANDLE eventArgs) : SessionEventArgsBase<TPropKey>(eventArgs)
    {
    }

private:

    DISABLE_DEFAULT_CTORS(SessionStoppedEventArgsBase);
};

} } } } } // Azure::AI::Vision::Core::Details
