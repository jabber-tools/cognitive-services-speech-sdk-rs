//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_c_common.h>

SPXAPI_(bool) async_op_handle_is_valid(SPXHANDLE asyncOp);
SPXAPI async_op_handle_release(SPXHANDLE asyncOp);

SPXAPI async_op_wait_for(SPXHANDLE asyncOp, uint32_t milliseconds);
SPXAPI async_op_wait_for_result(SPXHANDLE asyncOp, uint32_t milliseconds, SPXHANDLE* result);

typedef void (*PASYNC_OP_CALLBACK_FUNC)(SPXHANDLE asyncOpCallback, void* context);
SPXAPI async_op_callback_handle_create(SPXHANDLE* handle, void* context, PASYNC_OP_CALLBACK_FUNC callback);

SPXAPI_(bool) async_op_callback_handle_is_valid(SPXHANDLE asyncOpCallback);
SPXAPI async_op_callback_handle_release(SPXHANDLE asyncOpCallback);
