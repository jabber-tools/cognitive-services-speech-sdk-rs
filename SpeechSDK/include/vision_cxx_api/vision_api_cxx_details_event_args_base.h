//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_properties.h>
#include <vision_api_c_event_args.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Details {

template<typename TPropKey>
class EventArgsBase : public PrivatePropertyCollection<TPropKey>
{
public:

    using BaseProperties = PrivatePropertyCollection<TPropKey>;

    ~EventArgsBase()
    {
        if (vision_event_args_handle_is_valid(m_eventArgs))
        {
            ::vision_event_args_handle_release(m_eventArgs);
            m_eventArgs = SPXHANDLE_INVALID;
        }
    };

protected:

    explicit EventArgsBase(SPXHANDLE eventArgs) :
        PrivatePropertyCollection<TPropKey>(eventArgs, [](auto handle, auto* properties) { return vision_event_args_properties_handle_get(handle, properties); }),
        m_eventArgs(eventArgs)
    {
    }

    explicit operator SPXHANDLE() { return m_eventArgs; }

private:

    DISABLE_DEFAULT_CTORS(EventArgsBase);

    SPXHANDLE m_eventArgs;
};

} } } } } // Azure::AI::Vision::Core::Details
