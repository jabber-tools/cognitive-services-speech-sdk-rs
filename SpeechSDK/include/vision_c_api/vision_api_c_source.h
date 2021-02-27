//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_c_common.h>

SPXAPI_(bool) vision_source_handle_is_valid(SPXHANDLE source);
SPXAPI vision_source_handle_create(SPXHANDLE* source, const char* optionName, const char* optionValue, SPXHANDLE extra);
SPXAPI vision_source_handle_release(SPXHANDLE source);

SPXAPI vision_source_properties_handle_get(SPXHANDLE source, SPXHANDLE* properties);
