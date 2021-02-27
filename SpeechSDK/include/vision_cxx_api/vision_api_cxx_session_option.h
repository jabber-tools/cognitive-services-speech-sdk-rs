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
namespace Options {

enum class VisionSessionOption
{
    AdvancedInitializationParameters = 0,
    ApplicationName = 1,
    ApplicationContext = 2

    // TODO: What VisionSessionOption's should we have?
};

} } } } } } // Azure::AI::Vision::Core::Session::Options

PRIVATE_PROPERTY_COLLECTION_STATICS(Azure::AI::Vision::Core::Session::Options::VisionSessionOption, "session")
