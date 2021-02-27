//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <future>
#include <unordered_map>
#include <vision_api_cxx_common.h>
#include <vision_api_c_async_op.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Details {

template <class T>
class PromiseCallbackHelper // TODO: Put PromiseCallbackHelper into proper namespace
{
public:

    class Item
    {
    public:

        using PromiseType = std::shared_ptr<std::promise<void>>;
        using CallbackType = std::function<void(int id, SPXHANDLE handle)>;
        using ApiObjectType = std::weak_ptr<T>;

        Item(ApiObjectType&& object, int id, PromiseType&& promise, CallbackType&& callback) :
            m_id(id),
            m_promise(promise),
            m_callback(callback),
            m_apiObject(object)
        {
        }

        int GetId() const { return m_id; }

        void Complete(SPXHANDLE handle)
        {
            auto ptr = m_apiObject.lock();
            SPX_TRACE_WARNING_IF(ptr == nullptr, "ApiObject (0x%X) released prior to Complete(handle=0x%X)", ptr.get(), handle);

            auto promise = m_promise.get();
            SPX_IFTRUE(promise != nullptr, promise->set_value());

            bool valid = (bool)m_callback;
            SPX_IFTRUE(valid, m_callback(m_id, handle));
        }

    private:

        int m_id;
        PromiseType m_promise;
        CallbackType m_callback;
        ApiObjectType m_apiObject;
    };

    int PromiseCallback(std::shared_ptr<T>&& object, const char* name, std::shared_ptr<std::promise<void>>&& promise, std::function<void(int id, SPXHANDLE handle)> && callback = nullptr)
    {
        std::unique_lock<std::mutex> lock(m_mutex);
        auto id = m_nextId++;

        auto item = std::make_shared<Item>(std::move(object), id, std::move(promise), std::move(callback));
        SPX_IFTRUE(name != nullptr, m_namedCallbacks[name].push_back(item));

        m_idCallbacks[id] = item;
        return id;
    }

    SPXHANDLE PromiseAsyncOpCallback(std::shared_ptr<T>&& object, std::shared_ptr<std::promise<void>>&& promise, std::function<void(int id, SPXHANDLE handle)>&& callback = nullptr)
    {
        intptr_t id = PromiseCallback(std::move(object), nullptr, std::move(promise), std::move(callback));

        SPXHANDLE asyncOpCallback = SPXHANDLE_INVALID;
        SPX_THROW_ON_FAIL(async_op_callback_handle_create(&asyncOpCallback, (void*)id, AsyncOpCallbackHandler));

        return asyncOpCallback;
    }

    void CompletePromiseCallbacks(const char* name, SPXHANDLE handle)
    {
        auto callbacks = [=]() {
            std::unique_lock<std::mutex> lock(m_mutex);
            return m_namedCallbacks[name];
        }();

        for (auto& item : callbacks)
        {
            item->Complete(handle);
        }
    }

    void RemovePromiseCallback(int id, const char* name)
    {
        std::unique_lock<std::mutex> lock(m_mutex);
        m_idCallbacks.erase(id);

        for (auto& callback : m_namedCallbacks)
        {
            auto& list = callback.second;
            list.remove_if([=](auto item) { return item->GetId() == id && name != nullptr; });
        }
    }

    static void RemovePromiseCallback(int id)
    {
        std::unique_lock<std::mutex> lock(m_mutex);
        m_idCallbacks.erase(id);
    }

private:

    static std::shared_ptr<Item> ItemFromId(int id)
    {
        std::unique_lock<std::mutex> lock(m_mutex);
        return m_idCallbacks[id];
    }

    static void AsyncOpCallbackHandler(SPXHANDLE asyncOpCallback, void* context)
    {
        auto id = (int)reinterpret_cast<intptr_t>(context);

        auto item = ItemFromId(id);
        item->Complete(asyncOpCallback);

        RemovePromiseCallback(id);

        SPX_REPORT_ON_FAIL(async_op_callback_handle_release(asyncOpCallback));
    }

    std::unordered_map<std::string, std::list<std::shared_ptr<Item>>> m_namedCallbacks;

    static std::mutex m_mutex;
    static int m_nextId;
    static std::unordered_map<int, std::shared_ptr<Item>> m_idCallbacks;
};

} } } } } // Azure::AI::Vision::Core::Details

#define PROMISE_CALLBACK_HELPER_STATICS(x)  \
    template<> int Azure::AI::Vision::Core::Details::PromiseCallbackHelper<x>::m_nextId = 0; \
    template<> std::mutex Azure::AI::Vision::Core::Details::PromiseCallbackHelper<x>::m_mutex = { }; \
    template<> std::unordered_map<int, std::shared_ptr<Azure::AI::Vision::Core::Details::PromiseCallbackHelper<x>::Item>> Azure::AI::Vision::Core::Details::PromiseCallbackHelper<x>::m_idCallbacks = { };
