//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_c_common.h>

SPXAPI_(bool) vision_frame_format_handle_is_valid(SPXHANDLE format);
SPXAPI vision_frame_format_handle_create(SPXHANDLE* format, char ch1, char ch2, char ch3, char ch4, SPXHANDLE moreOptions);
SPXAPI vision_frame_format_handle_release(SPXHANDLE format);

SPXAPI vision_frame_format_properties_handle_get(SPXHANDLE format, SPXHANDLE* properties);
