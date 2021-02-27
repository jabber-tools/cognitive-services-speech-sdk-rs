//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_c_common.h>

SPXAPI_(bool) vision_result_handle_is_valid(SPXHANDLE result);
SPXAPI vision_result_handle_release(SPXHANDLE result);

SPXAPI vision_result_properties_handle_get(SPXHANDLE result, SPXHANDLE* properties);
