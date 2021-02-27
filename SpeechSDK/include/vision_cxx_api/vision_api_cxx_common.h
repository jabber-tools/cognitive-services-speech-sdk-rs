//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once

#include <spxdebug.h> // TODO: fully re-factor/duplicate/whatever <spxdebug.h> to have vision sdk not rely directly on public headers from speech sdk
#include <speechapi_cxx_common.h> // TODO: fully re-factor/duplicate/whatever <speechapi_cxx_common.h> to have vision sdk not rely directly on public headers from speech sdk

#include <vision_api_cxx_string_helpers.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Details {

template <class T>
class ProtectedAccess : public T
{
public:

    static SPXHANDLE HandleFromPtr(T* ptr) {
        SPX_IFTRUE_RETURN_X(ptr == nullptr, nullptr);
        auto access = static_cast<ProtectedAccess*>(ptr);
        return (SPXHANDLE)(*access);
    }

    static std::shared_ptr<T> FromHandle(SPXHANDLE handle) {
        return T::FromHandle(handle);
    }

    template<typename... Args>
    static std::shared_ptr<T> Create(Args&&... args) {
        return T::Create(std::forward<Args&&>(args)...);
    }

};

} } } } } // Azure::AI::Vision::Core::Details

