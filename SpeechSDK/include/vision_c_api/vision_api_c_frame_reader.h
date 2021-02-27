//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_c_common.h>

SPXAPI_(bool) vision_frame_reader_handle_is_valid(SPXHANDLE reader);
SPXAPI vision_frame_reader_handle_release(SPXHANDLE reader);

SPXAPI vision_frame_reader_properties_handle_get(SPXHANDLE reader, SPXHANDLE* properties);
// read method ...
