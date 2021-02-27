//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_details_result_event_args_base.h>
#include <vision_api_cxx_details_event_args_base.h>
#include <vision_api_cxx_session_result.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Session {
namespace Events {

/// <summary>
/// Represents inference recognition event arguments.
/// </summary>
class SessionResultEventArgs : private Core::Details::ResultEventArgsBase<Results::SessionResult, int>
{
private:

    using BaseResultEventArgs = Core::Details::ResultEventArgsBase<Results::SessionResult, int>;

public:

    /// <summary>
    /// Gets the Result of an AI inteferening operation
    /// </summary>
    /// <returns>The SessionResult wrapped inside a std::shared_ptr</returns>
    std::shared_ptr<Results::SessionResult> GetResult() const { return BaseResultEventArgs::GetResult(); }

protected:

    static std::shared_ptr<SessionResultEventArgs> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new SessionResultEventArgs(handle);
        return std::shared_ptr<SessionResultEventArgs>(ptr);
    }

    explicit SessionResultEventArgs(SPXHANDLE eventArgs) : ResultEventArgsBase(eventArgs)
    {
    }

private:

    DISABLE_DEFAULT_CTORS(SessionResultEventArgs);
};

} } } } } } // Azure::AI::Vision::Core::Session::Results
