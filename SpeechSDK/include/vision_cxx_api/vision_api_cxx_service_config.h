//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_properties.h>
#include <vision_api_cxx_session_result_reason.h>
#include <vision_api_c_result.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Service {

enum class VisionServiceOption // TODO: CONSIDER: break this into it's own file?
{
    Endpoint = 0,
    HttpProxy = 1,
    HttpProxyUserName = 2,
    HttpProxyPassword = 3

    // TODO: What VisionServiceOption's should we have?
};

/// <summary>
/// Represents advanced options and parameters on the VisionService object.
/// </summary>
class VisionServiceAdvancedOptions // TODO: CONSIDER: break this into it's own file?
{
private:

    template<typename Target> using ProtectedAccess = Azure::AI::Vision::Core::Details::ProtectedAccess<Target>;
    using PropertiesType = Core::PropertyCollection<VisionServiceOption>;
    std::shared_ptr<PropertiesType> m_options;

public:

    /// <summary>
    /// Sets the string value for the specified option.
    /// </summary>
    /// <param name="key">The string value's key</param>
    /// <param name="value">The new string value</param>
    void Set(VisionServiceOption option, const SPXSTRING& value)
    {
        m_options->Set(option, value);
    }

    /// <summary>
    /// Sets the property boolean value for the specified key value.
    /// </summary>
    /// <param name="key">The boolean value's key</param>
    /// <param name="value">The new boolean value</param>
    void Set(VisionServiceOption option, bool value)
    {
        Set(option, std::to_string(value));
    }

    /// <summary>
    /// Sets the property integer value for the specified key value.
    /// </summary>
    /// <param name="key">The integer value's key</param>
    /// <param name="value">The new integer value</param>
    void Set(VisionServiceOption option, int value)
    {
        Set(option, std::to_string(value));
    }

    /// <summary>
    /// Sets the string value for the specified option.
    /// </summary>
    /// <param name="key">The string value's key</param>
    /// <param name="value">The new string value</param>
    void Set(const SPXSTRING& option, const SPXSTRING& value)
    {
        m_options->Set(option, value);
    }

    /// <summary>
    /// Sets the property boolean value for the specified key value.
    /// </summary>
    /// <param name="key">The boolean value's key</param>
    /// <param name="value">The new boolean value</param>
    void Set(const SPXSTRING& option, bool value)
    {
        Set(option, std::to_string(value));
    }

    /// <summary>
    /// Sets the property integer value for the specified key value.
    /// </summary>
    /// <param name="key">The integer value's key</param>
    /// <param name="value">The new integer value</param>
    void Set(const SPXSTRING& option, int value)
    {
        Set(option, std::to_string(value));
    }

    // TODO: Other VisionServiceAdvancedOptions strongly-typed setting methods ...

protected:

    static std::shared_ptr<VisionServiceAdvancedOptions> Create()
    {
        auto ptr = new VisionServiceAdvancedOptions();
        return std::shared_ptr<VisionServiceAdvancedOptions>(ptr);
    }

    explicit VisionServiceAdvancedOptions() :
        m_options(PropertiesType::Create())
    {
    }

    explicit operator SPXHANDLE()
    {
        return ProtectedAccess<PropertiesType>::HandleFromPtr(m_options.get());
    }


private:

    DISABLE_COPY_AND_MOVE(VisionServiceAdvancedOptions);
};

/// <summary>
/// Represents the service configuration options and parameters used to connect to network attached
/// AI inferencing technologies over IP based protocols.
/// </summary>
/// <remarks>Use VisionServiceConfig::FromEndpoint() or similar to instantiate</remarks>
class VisionServiceConfig
{
private:

    template<typename Target> using ProtectedAccess = ::Azure::AI::Vision::Core::Details::ProtectedAccess<Target>;
    std::shared_ptr<VisionServiceAdvancedOptions> m_options;

public:

    /// <summary>
    /// Initializes a new instance of the VisionServiceConfig class used to connect to the specified URL endpoint.
    /// </summary>
    /// <param name="endpoint">The vision service endpoint to connect to.</param>
    /// <returns>The newly created VisionServiceConfig wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<VisionServiceConfig> FromEndpoint(const SPXSTRING& endpoint)
    {
        auto ptr = new VisionServiceConfig();

        auto utf8 = Microsoft::CognitiveServices::Speech::Utils::ToUTF8(endpoint);
        ptr->Advanced.Set("service.endpoint", utf8.c_str());

        return std::shared_ptr<VisionServiceConfig>(ptr);
    }

    /// <summary>
    /// Sets the authorization token to be used to connect to the service.
    /// </summary>
    /// <param name="token">The authorization token</param>
    void SetAuthorizationToken(const SPXSTRING& token)
    {
        Advanced.Set("service.auth.token", token);
    }

    /// <summary>
    /// Destructs an instance of the VisionServiceConfig class.
    /// </summary>
    virtual ~VisionServiceConfig() = default;

    /// <summary>
    /// Advanced options and parameters
    /// </summary>
    VisionServiceAdvancedOptions& Advanced;

    // TODO: What strongly typed methods go here?

protected:

    explicit VisionServiceConfig() :
        m_options(ProtectedAccess<VisionServiceAdvancedOptions>::Create()),
        Advanced(*m_options.get())
    {
    }

    explicit operator SPXHANDLE()
    {
        return ProtectedAccess<VisionServiceAdvancedOptions>::HandleFromPtr(m_options.get());
    }

private:

    DISABLE_COPY_AND_MOVE(VisionServiceConfig);
};

} } } } // Azure::AI::Vision::Service

PRIVATE_PROPERTY_COLLECTION_STATICS(Azure::AI::Vision::Service::VisionServiceOption, "service")
