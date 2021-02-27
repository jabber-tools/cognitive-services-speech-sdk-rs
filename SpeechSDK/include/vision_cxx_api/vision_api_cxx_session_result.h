//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_details_result_base.h>
#include <vision_api_cxx_session_result_property.h>
#include <vision_api_cxx_session_result_reason.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Session {
namespace Results {

/// <summary>
/// Represents the output an AI inteferening operation (e.g. detection, recognition, prediction, ...).
/// </summary>
class SessionResult : private Core::Details::ResultBase<SessionResultReason, SessionResultProperty>
{
private:

    using BaseResult = ResultBase<SessionResultReason, SessionResultProperty>;

public:

    /// <summary>
    /// Destructs an instance of the SessionResult class.
    /// </summary>
    ~SessionResult() = default;

    /// <summary>
    /// Gets the unique id for the Session from which this SessionResult originated.
    /// </summary>
    /// <returns>
    /// The Session Id string.
    /// </returns>
    SPXSTRING GetSessionId() const { return BaseResult::GetSessionId(); }

    /// <summary>
    /// Gets the unique SessionResult ID for this SessionResult.
    /// </summary>
    /// <returns>
    /// The unique SessionResult Id string.
    /// </returns>
    SPXSTRING GetResultId() const { return BaseResult::GetResultId(); }

    /// <summary>
    /// Gets the SessionResultReason for generation of this result.
    /// </summary>
    SessionResultReason GetReason() const { return BaseResult::GetReason(SessionResultReason::NoMatch, SessionResultReason::Recognized); }

    /// <summary>
    /// Gets a collection of additional inferencing operation properties.
    /// </summary>
    const Core::PropertyCollection<SessionResultProperty>& Properties;

protected:

    static std::shared_ptr<SessionResult> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new SessionResult(handle);
        return std::shared_ptr<SessionResult>(ptr);
    }

    explicit SessionResult(SPXHANDLE result) :
        ResultBase(result),
        Properties(GetProperties())
    {
    }

    explicit operator SPXHANDLE() { return Core::Details::ProtectedAccess<BaseResult>::HandleFromPtr(this); }

private:

    DISABLE_DEFAULT_CTORS(SessionResult);
};

} } } } } } // Azure::AI::Vision::Core::Session::Results
