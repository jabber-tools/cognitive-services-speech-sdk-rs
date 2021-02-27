//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_c_common.h>

SPXAPI_(bool) vision_frame_source_handle_is_valid(SPXHANDLE source);
SPXAPI vision_frame_source_handle_create(SPXHANDLE* source, const char* optionName, const char* optionValue, SPXHANDLE format);
SPXAPI vision_frame_source_handle_release(SPXHANDLE source);

SPXAPI vision_frame_source_properties_handle_get(SPXHANDLE source, SPXHANDLE* properties);
SPXAPI vision_frame_source_writer_handle_get(SPXHANDLE source, SPXHANDLE* writer);
SPXAPI vision_frame_source_reader_handle_get(SPXHANDLE source, SPXHANDLE* reader);

// TODO: ❓ need to spec and write vision_frame_source_callback_set