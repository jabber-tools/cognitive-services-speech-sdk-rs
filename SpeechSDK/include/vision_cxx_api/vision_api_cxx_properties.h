//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_enums.h>
#include <vision_api_cxx_string_helpers.h>
#include <vision_api_c_properties.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {

namespace Details {

template <typename T>
struct PropertyCollectionKeyKind {
    static constexpr const char* GetKind()
    {
        static_assert(std::is_same<T, int>::value, "Didn't declare kind... See: PRIVATE_PROPERTY_COLLECTION_STATICS");
        return "int";
    }
};

} // Details


/// <summary>
/// Represents a collection of properties and their string values.
/// </summary>
template <typename TPropKey = int>
class PropertyCollection
{
public:

    /// <summary>
    /// Initializes a new instance of the PropertyCollection class.
    /// </summary>
    static std::shared_ptr<PropertyCollection<TPropKey>> Create()
    {
        SPXHANDLE handle = SPXHANDLE_INVALID;
        SPX_THROW_ON_FAIL(ai_core_properties_handle_create(&handle));
        return FromHandle(handle);
    }

    /// <summary>
    /// Destructs an instance of the PropertyCollection class.
    /// </summary>
    ~PropertyCollection()
    {
        if (ai_core_properties_handle_is_valid(m_properties))
        {
            ai_core_properties_handle_release(m_properties);
            m_properties = SPXHANDLE_INVALID;
        }
    }

    /// <summary>
    /// Sets the property string value for the specified key value.
    /// </summary>
    /// <param name="key">The string value's key</param>
    /// <param name="value">The new string value</param>
    void Set(TPropKey key, const char* value)
    {
        auto kind = Details::PropertyCollectionKeyKind<TPropKey>::GetKind();
        SPX_THROW_ON_FAIL(ai_core_properties_string_set(m_properties, (int)key, kind, value));
    }

    /// <summary>
    /// Sets the property string value for the specified key value.
    /// </summary>
    /// <param name="key">The string value's key</param>
    /// <param name="value">The new string value</param>
    void Set(TPropKey key, const std::string& value) { Set(key, value.c_str()); }

    /// <summary>
    /// Sets the property string value for the specified key value.
    /// </summary>
    /// <param name="key">The string value's key</param>
    /// <param name="value">The new string value</param>
    void Set(TPropKey key, const std::wstring& value) { Set(key, Microsoft::CognitiveServices::Speech::Utils::ToUTF8(value)); }; 

    /// <summary>
    /// Sets the property boolean value for the specified key value.
    /// </summary>
    /// <param name="key">The boolean value's key</param>
    /// <param name="value">The new boolean value</param>
    void Set(TPropKey key, bool value) { Set(key, std::to_string(value)); }

    /// <summary>
    /// Sets the property integer value for the specified key value.
    /// </summary>
    /// <param name="key">The integer value's key</param>
    /// <param name="value">The new integer value</param>
    void Set(TPropKey key, int value) { Set(key, std::to_string(value)); }

    /// <summary>
    /// Sets the property string value for the specified key value.
    /// </summary>
    /// <param name="key">The string value's key</param>
    /// <param name="value">The new string value</param>
    void Set(const char* key, const char* value)
    {
        SPX_THROW_ON_FAIL(ai_core_properties_string_set(m_properties, 0, key, value));
    }

    /// <summary>
    /// Sets the property string value for the specified key value.
    /// </summary>
    /// <param name="key">The string value's key</param>
    /// <param name="value">The new string value</param>
    void Set(const std::string& key, const std::string& value) { Set(key.c_str(), value.c_str()); }

    /// <summary>
    /// Sets the property string value for the specified key value.
    /// </summary>
    /// <param name="key">The string value's key</param>
    /// <param name="value">The new string value</param>
    void Set(const std::wstring& key, const std::wstring& value) { Set(Microsoft::CognitiveServices::Speech::Utils::ToUTF8(key), Microsoft::CognitiveServices::Speech::Utils::ToUTF8(value)); }

    /// <summary>
    /// Sets the property boolean value for the specified key value.
    /// </summary>
    /// <param name="key">The boolean value's key</param>
    /// <param name="value">The new boolean value</param>
    void Set(const std::string& key, bool value) { Set(key, std::to_string(value)); }

    /// <summary>
    /// Sets the property boolean value for the specified key value.
    /// </summary>
    /// <param name="key">The boolean value's key</param>
    /// <param name="value">The new boolean value</param>
    void Set(const std::wstring& key, bool value) { Set(key, std::to_wstring(value)); }

    /// <summary>
    /// Sets the property integer value for the specified key value.
    /// </summary>
    /// <param name="key">The integer value's key</param>
    /// <param name="value">The new integer value</param>
    void Set(const std::string& key, int value) { Set(key, std::to_string(value)); }

    /// <summary>
    /// Sets the property integer value for the specified key value.
    /// </summary>
    /// <param name="key">The integer value's key</param>
    /// <param name="value">The new integer value</param>
    void Set(const std::wstring& key, int value) { Set(key, std::to_wstring(value)); }

    /// <summary>
    /// Gets the property string value for the specified key value.
    /// </summary>
    /// <param name="key">The string value's key value</param>
    /// <param name="defaultValue">A default string value, returned if the key value is not found</param>
    /// <returns>The property string value for the specified key value, or the default value if the key value is not found.</returns>
    const std::string Get(TPropKey key, const std::string& defaultValue = "") const
    {
        auto kind = Details::PropertyCollectionKeyKind<TPropKey>::GetKind();
        auto ptr = ai_core_properties_string_get(m_properties, (int)key, kind, defaultValue.c_str());
        auto value = Microsoft::CognitiveServices::Speech::Utils::ToSPXString(ptr);
        SPX_THROW_ON_FAIL(ai_core_properties_string_free(ptr));
        return value;
    }

    /// <summary>
    /// Gets the property string value for the specified key value.
    /// </summary>
    /// <param name="key">The string value's key value</param>
    /// <param name="defaultValue">A default string value, returned if the key value is not found</param>
    /// <returns>The property string value for the specified key value, or the default value if the key value is not found.</returns>
    const std::string Get(const std::string& key, const std::string& defaultValue = "") const
    {
        auto ptr = ai_core_properties_string_get(m_properties, 0, key.c_str(), defaultValue.c_str());
        auto value = std::string(ptr);
        SPX_THROW_ON_FAIL(ai_core_properties_string_free(ptr));
        return value;
    }

protected:

    static std::shared_ptr<PropertyCollection<TPropKey>> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new PropertyCollection<TPropKey>(handle);
        return std::shared_ptr<PropertyCollection<TPropKey>>(ptr);
    }

    explicit PropertyCollection(SPXHANDLE handle) :
        m_properties(handle)
    {
    }

    explicit operator SPXHANDLE() { return m_properties; }

private:

    DISABLE_COPY_AND_MOVE(PropertyCollection);

    SPXHANDLE m_properties;
};

namespace Details {

template <typename TPropKey = int>
class PrivatePropertyCollection : public Azure::AI::Vision::Core::PropertyCollection<TPropKey>
{
public:
    PrivatePropertyCollection(SPXHANDLE handle, std::function<SPXHR(SPXHANDLE, SPXHANDLE*)> fn) :
        PropertyCollection<TPropKey>( [&]() {
            SPXHANDLE properties = SPXHANDLE_INVALID;
            SPX_THROW_ON_FAIL(fn(handle, &properties));
            return properties;
        }())
    {
    }
};

} // Details

} } } } // Azure::AI::Vision::Core

#define PRIVATE_PROPERTY_COLLECTION_STATICS(x, name) template<> \
constexpr const char* Azure::AI::Vision::Core::Details::PropertyCollectionKeyKind<x>::GetKind() { return name; }
