//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_details_result_event_args_base.h>
#include <vision_api_cxx_face_result.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Face {
namespace Events {

/// <summary>
/// Represents inference recognition event arguments.
/// </summary>
class FaceResultEventArgs : private Core::Details::ResultEventArgsBase<Results::FaceResult, int>
{
private:

    using BaseResultEventArgs = Core::Details::ResultEventArgsBase<Results::FaceResult, int>;

public:

    /// <summary>
    /// Gets the Result of an AI inteferening operation
    /// </summary>
    /// <returns>The FaceResult wrapped inside a std::shared_ptr</returns>
    std::shared_ptr<Results::FaceResult> GetResult() const { return BaseResultEventArgs::GetResult(); }

protected:

    static std::shared_ptr<FaceResultEventArgs> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new FaceResultEventArgs(handle);
        return std::shared_ptr<FaceResultEventArgs>(ptr);
    }

    explicit FaceResultEventArgs(SPXHANDLE eventArgs) : ResultEventArgsBase(eventArgs)
    {
    }

private:

    DISABLE_DEFAULT_CTORS(FaceResultEventArgs);
};

} } } } } // Azure::AI::Vision::Face::Results
