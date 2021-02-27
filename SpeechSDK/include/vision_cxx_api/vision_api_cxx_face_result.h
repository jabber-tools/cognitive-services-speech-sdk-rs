//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_details_result_base.h>
#include <vision_api_cxx_face_result_property.h>
#include <vision_api_cxx_face_result_reason.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Face {
namespace Results {

/// <summary>
/// Represents the output an AI inteferening operation (e.g. detection, recognition, prediction, ...).
/// </summary>
class FaceResult : private Core::Details::ResultBase<FaceResultReason, FaceResultProperty>
{
private:

    using BaseResult = ResultBase<FaceResultReason, FaceResultProperty>;

public:

    /// <summary>
    /// Destructs an instance of the FaceResult class.
    /// </summary>
    ~FaceResult() = default;

    /// <summary>
    /// Gets the unique id for the Session from which this FaceResult originated.
    /// </summary>
    /// <returns>
    /// The Session Id string.
    /// </returns>
    SPXSTRING GetSessionId() const { return BaseResult::GetSessionId(); }

    /// <summary>
    /// Gets the unique FaceResult ID for this FaceResult.
    /// </summary>
    /// <returns>
    /// The unique FaceResult Id string.
    /// </returns>
    SPXSTRING GetResultId() const { return BaseResult::GetResultId(); }

    /// <summary>
    /// Gets the FaceResultReason for generation of this result.
    /// </summary>
    FaceResultReason GetReason() const { return BaseResult::GetReason(FaceResultReason::NoMatch, FaceResultReason::Recognized); }

    /// <summary>
    /// Gets a collection of additional inferencing operation properties.
    /// </summary>
    const Core::PropertyCollection<Results::FaceResultProperty>& Properties;

protected:

    static std::shared_ptr<FaceResult> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new FaceResult(handle);
        return std::shared_ptr<FaceResult>(ptr);
    }

    explicit FaceResult(SPXHANDLE result) :
        ResultBase(result),
        Properties(GetProperties())
    {
    }

    explicit operator SPXHANDLE() { return Core::Details::ProtectedAccess<BaseResult>::HandleFromPtr(this); }

private:

    DISABLE_DEFAULT_CTORS(FaceResult);
};

} } } } } // Azure::AI::Vision::Face::Results
