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
namespace Options {

enum class FaceRecognizerOption
{
    TODO = -1,

    GroupId,
    GroupIdKind,
    MatchMode,
    RequestResultMaxFaceCandidates,
    RequestResultFaceLandmarks,
    RequestResultFaceAttributes,

    // TODO: What FaceRecognizerOption's should we have?
};

} } } } } // Azure::AI::Vision::Face::Options

PRIVATE_PROPERTY_COLLECTION_STATICS(Azure::AI::Vision::Face::Options::FaceRecognizerOption, "face.recognizer")
