//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_c_common.h>

SPXAPI_(bool) ai_core_properties_handle_is_valid(SPXHANDLE properties);
SPXAPI ai_core_properties_handle_create(SPXHANDLE* properties);
SPXAPI ai_core_properties_handle_release(SPXHANDLE properties);

SPXAPI__(const char*) ai_core_properties_string_get(SPXHANDLE properties, int id, const char* name, const char* defaultValue);
SPXAPI ai_core_properties_string_set(SPXHANDLE properties, int id, const char* name, const char* value);
SPXAPI ai_core_properties_string_free(const char* value);

SPXAPI ai_core_properties_copy(SPXHANDLE fromProperties, SPXHANDLE toProperties);
