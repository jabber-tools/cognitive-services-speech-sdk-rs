//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_c_common.h>

SPXAPI_(bool) vision_frame_writer_handle_is_valid(SPXHANDLE writer);
SPXAPI vision_frame_writer_handle_release(SPXHANDLE writer);

SPXAPI vision_frame_writer_properties_handle_get(SPXHANDLE writer, SPXHANDLE* properties);

SPXAPI_(uint64_t) vision_frame_writer_pos_get(SPXHANDLE writer, uint32_t stream, const char* reserved);
SPXAPI vision_frame_writer_write(SPXHANDLE writer, uint32_t stream, const uint8_t* data, uint32_t size, const char* propName, const char* propValue);
SPXAPI vision_frame_writer_property_write(SPXHANDLE writer, uint32_t stream, const char* name, const char* value, uint64_t pos, const char* reserved);

SPXAPI vision_frame_writer_close(SPXHANDLE writer);
