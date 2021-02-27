//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Face {
namespace Results {

enum class FaceResultProperty
{
    Json = 0

    // TODO: What FaceResultProperty's should we have?
};

} } } } } // Azure::AI::Vision::Face::Results

PRIVATE_PROPERTY_COLLECTION_STATICS(Azure::AI::Vision::Face::Results::FaceResultProperty, "face.result")
