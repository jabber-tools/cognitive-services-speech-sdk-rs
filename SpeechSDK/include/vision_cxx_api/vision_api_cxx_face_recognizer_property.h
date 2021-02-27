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

enum class FaceRecognizerProperty // TODO: Move to it's own file
{
    TODO = -1,

    GroupId,
    GroupIdKind,
    SimilarMatchMode,
    RequestResultMaxFaceCandidates,
    RequestResultFaceLandmarks,
    RequestResultFaceAttributes,

    // TODO: What FaceRecognizerProperty's should we have?
};

} } } } // Azure::AI::Vision::Face

PRIVATE_PROPERTY_COLLECTION_STATICS(Azure::AI::Vision::Face::FaceRecognizerProperty, "face.recognizer")
