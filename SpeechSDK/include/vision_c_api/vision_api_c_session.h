//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_c_common.h>

SPXAPI_(bool) vision_session_handle_is_valid(SPXHANDLE session);
SPXAPI vision_session_handle_create(SPXHANDLE* session, SPXHANDLE createOptions, SPXHANDLE source);
SPXAPI vision_session_handle_release(SPXHANDLE session);

SPXAPI vision_session_properties_handle_get(SPXHANDLE session, SPXHANDLE* properties);
SPXAPI vision_session_view_handle_get(SPXHANDLE session, SPXHANDLE* sessionView);

SPXAPI_(bool) vision_session_view_handle_is_valid(SPXHANDLE sessionView);
SPXAPI vision_session_view_handle_create(SPXHANDLE* sessionView, SPXHANDLE session, const char* viewKind, SPXHANDLE viewOptions);
SPXAPI vision_session_view_handle_release(SPXHANDLE sessionView);

SPXAPI vision_session_view_properties_handle_get(SPXHANDLE sessionView, SPXHANDLE* properties);

SPXAPI vision_session_view_single_shot_start(SPXHANDLE sessionView, SPXHANDLE startOptions, SPXHANDLE asyncOpCallback, SPXHANDLE* asyncOp);
SPXAPI vision_session_view_single_shot_stop(SPXHANDLE sessionView, SPXHANDLE stopOptions, SPXHANDLE asyncOpCallback, SPXHANDLE* asyncOp);

SPXAPI vision_session_view_continuous_start(SPXHANDLE sessionView, SPXHANDLE startOptions, SPXHANDLE asyncOpCallback, SPXHANDLE* asyncOp);
SPXAPI vision_session_view_continuous_stop(SPXHANDLE sessionView, SPXHANDLE stopOptions, SPXHANDLE asyncOpCallback, SPXHANDLE* asyncOp);

typedef void (*PEVENT_CALLBACK_FUNC)(SPXHANDLE handle, const char* name, void* context, SPXHANDLE eventArgs);
SPXAPI vision_session_view_event_callback_set(SPXHANDLE sessionView, const char* name, void* context, PEVENT_CALLBACK_FUNC callback);
SPXAPI vision_session_view_event_callbacks_clear(SPXHANDLE sessionView);
