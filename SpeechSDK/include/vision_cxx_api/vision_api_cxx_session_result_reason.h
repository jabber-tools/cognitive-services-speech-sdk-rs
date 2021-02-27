//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Session {
namespace Results {

/// <summary>
/// Specifies the possible reasons a recognition result might be generated.
/// </summary>
enum class SessionResultReason
{
    /// <summary>
    /// Indicates the requested inference was not found. More details can be found using the NoMatchDetails::FromResult method.
    /// </summary>
    NoMatch = 0,

    /// <summary>
    /// Indicates the requested operation was stopped. More details can be found using the StoppedDetails::FromResult method.
    /// </summary>
    Stopped = 1,

    /// <summary>
    /// Indicates preliminary or partial detection results are available.
    /// </summary>
    Detecting = 2,

    /// <summary>
    /// Indicates final and complete detection results are available.
    /// </summary>
    Detected = 3,

    /// <summary>
    /// Indicates preliminary or partial recognition results are available.
    /// </summary>
    Recognizing = 4,

    /// <summary>
    /// Indicates final and complete inference results are available.
    /// </summary>
    Recognized = 5
};

} } } } } } // Azure::AI::Vision::Core::Session::Results
