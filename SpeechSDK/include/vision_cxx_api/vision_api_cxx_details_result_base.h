//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_properties.h>
#include <vision_api_c_result.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Details {

template<typename TReason, typename TPropKey>
class ResultBase
{
private:

    PrivatePropertyCollection<TPropKey> m_properties;

public:

    ~ResultBase()
    {
        if (vision_result_handle_is_valid(m_result))
        {
            ::vision_result_handle_release(m_result);
            m_result = SPXHANDLE_INVALID;
        }
    };

    SPXSTRING GetSessionId() const
    {
        return m_properties.Get("session.id", "");
    }

    SPXSTRING GetResultId() const
    {
        return m_properties.Get("result.id", "");
    }

    TReason GetReason(TReason min, TReason max) const
    {
        auto property = m_properties.Get("result.reason", "0");
        auto value = std::stoi(property.c_str());
        SPX_IFTRUE_THROW_HR(value < (int)min || value > (int)max, SPXERR_INVALID_RESULT_REASON);
        return (TReason)value;
    }

    const Core::PropertyCollection<TPropKey>& GetProperties() const { return m_properties; }

protected:

    explicit ResultBase(SPXHANDLE result) :
        m_properties(result, [](auto handle, auto* properties) { return vision_result_properties_handle_get(handle, properties); }),
        m_result(result)
    {
    }

    explicit operator SPXHANDLE() { return m_result; }

private:

    DISABLE_DEFAULT_CTORS(ResultBase);

    SPXHANDLE m_result;
};

} } } } } // Azure::AI::Vision::Core::Details
