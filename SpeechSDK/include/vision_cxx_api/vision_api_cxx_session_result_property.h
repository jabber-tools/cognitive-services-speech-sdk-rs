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

enum class SessionResultProperty
{
    Json = 0

    // TODO: What SessionResultProperty's should we have?
};

} } } } } } // Azure::AI::Vision::Core::Session::Results

PRIVATE_PROPERTY_COLLECTION_STATICS(Azure::AI::Vision::Core::Session::Results::SessionResultProperty, "session.result")
