//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_properties.h>
#include <vision_api_cxx_session_option.h>
#include <vision_api_cxx_session_result_reason.h>
#include <vision_api_c_result.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Core {
namespace Session {
namespace Options {

/// <summary>
/// Represents the options and parameters used to initialize a VisionSession instance.
/// </summary>
class VisionSessionOptions
{
public:

    /// <summary>
    /// Represents advanced options and parameters used to initialize a VisionSession instance.
    /// </summary>
    class AdvancedOptions
    {
    private:

        template<typename Target> using ProtectedAccess = Azure::AI::Vision::Core::Details::ProtectedAccess<Target>;
        using PropertiesType = PropertyCollection<VisionSessionOption>;
        std::shared_ptr<PropertiesType> m_options;

    public:

        /// <summary>
        /// Sets the string value for the specified option.
        /// </summary>
        /// <param name="key">The string value's key</param>
        /// <param name="value">The new string value</param>
        void Set(VisionSessionOption option, const SPXSTRING& value)
        {
            m_options->Set(option, value);
        }

        /// <summary>
        /// Sets the property boolean value for the specified key value.
        /// </summary>
        /// <param name="key">The boolean value's key</param>
        /// <param name="value">The new boolean value</param>
        void Set(VisionSessionOption option, bool value)
        {
            Set(option, std::to_string(value));
        }

        /// <summary>
        /// Sets the property integer value for the specified key value.
        /// </summary>
        /// <param name="key">The integer value's key</param>
        /// <param name="value">The new integer value</param>
        void Set(VisionSessionOption option, int value)
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

        // TODO: Other AdvancedOptions strongly-typed setting methods ...

    protected:

        static std::shared_ptr<AdvancedOptions> Create()
        {
            auto ptr = new AdvancedOptions();
            return std::shared_ptr<AdvancedOptions>(ptr);
        }

        explicit AdvancedOptions() :
            m_options(PropertiesType::Create())
        {
        }

        explicit operator SPXHANDLE()
        {
            return ProtectedAccess<PropertiesType>::HandleFromPtr(m_options.get()); // TODO: Does this need to be ProtectedAccess now?
        }

    private:

        DISABLE_COPY_AND_MOVE(AdvancedOptions);
    };

private:

    template<typename Target> using ProtectedAccess = Azure::AI::Vision::Core::Details::ProtectedAccess<Target>;

    std::shared_ptr<AdvancedOptions> m_options;

public:

    // TODO: CONSIDER: `FromYyyy` factory pattern, not just create?

    /// <summary>
    /// Initializes a new instance of the VisionSessionOptions class.
    /// </summary>
    /// <returns>The newly created VisionSessionOptions wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<VisionSessionOptions> Create()
    {
        auto ptr = new VisionSessionOptions();
        return std::shared_ptr<VisionSessionOptions>(ptr);
    }

    /// <summary>
    /// Destructs an instance of the VisionSessionOptions class.
    /// </summary>
    ~VisionSessionOptions() = default;

    /// <summary>
    /// Advanced options and parameters
    /// </summary>
    AdvancedOptions& Advanced;

    // TODO: What strongly typed options go here?

protected:

    explicit VisionSessionOptions() :
        m_options(ProtectedAccess<AdvancedOptions>::Create()),
        Advanced(*m_options.get())
    {
    }

    explicit operator SPXHANDLE()
    {
        return ProtectedAccess<AdvancedOptions>::HandleFromPtr(m_options.get());
    }

private:

    DISABLE_COPY_AND_MOVE(VisionSessionOptions);
};

} } } } } } // Azure::AI::Vision::Core::Session::Options
