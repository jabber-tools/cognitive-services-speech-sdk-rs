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

template<typename TResult, typename TPropKey>
class ResultEventArgsBase : private EventArgsBase<TPropKey>
{
private:

    template<typename Target> using ProtectedAccess = ::Azure::AI::Vision::Core::Details::ProtectedAccess<Target>;

public:

    std::shared_ptr<TResult> GetResult() const
    {
        SPXHANDLE result = SPXHANDLE_INVALID;
        auto eventArgs = ProtectedAccess<EventArgsBase<TPropKey>>::HandleFromPtr((EventArgsBase<TPropKey>*)this);
        SPX_THROW_ON_FAIL(vision_event_args_result_handle_get(eventArgs, &result));
        return ProtectedAccess<TResult>::FromHandle(result);
    }

protected:

    explicit ResultEventArgsBase(SPXHANDLE eventArgs) : EventArgsBase<TPropKey>(eventArgs)
    {
    }

private:

    DISABLE_DEFAULT_CTORS(ResultEventArgsBase);
};

} } } } } // Azure::AI::Vision::Core::Details
