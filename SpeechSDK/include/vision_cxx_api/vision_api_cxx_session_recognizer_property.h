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

enum class VisionSessionRecognizerProperty
{
    TODO = -1

    // TODO: What SessionRecognizerProperty's should we have?
};

} } } } } // Azure::AI::Vision::Core::Session

PRIVATE_PROPERTY_COLLECTION_STATICS(Azure::AI::Vision::Core::Session::VisionSessionRecognizerProperty, "session.recognizer")
