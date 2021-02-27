//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_c_common.h>

SPXAPI_(bool) vision_event_args_handle_is_valid(SPXHANDLE eventArgs);
SPXAPI vision_event_args_handle_release(SPXHANDLE eventArgs);

SPXAPI vision_event_args_properties_handle_get(SPXHANDLE eventArgs, SPXHANDLE* properties);

SPXAPI vision_event_args_result_handle_get(SPXHANDLE eventArgs, SPXHANDLE* result);
