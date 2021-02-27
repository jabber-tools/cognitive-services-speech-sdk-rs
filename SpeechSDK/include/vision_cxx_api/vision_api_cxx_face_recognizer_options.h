//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_properties.h>
#include <vision_api_cxx_face_recognizer_option.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Face {
namespace Options {

/// <summary>
/// Represents the options and parameters used to initialize a VisionSession instance.
/// </summary>
class FaceRecognizerOptions
{
public:

    /// <summary>
    /// Represents advanced options and parameters used to initialize a VisionSession instance.
    /// </summary>
    class AdvancedOptions
    {
    private:

        template<typename Target> using ProtectedAccess = Azure::AI::Vision::Core::Details::ProtectedAccess<Target>;
        using PropertiesType = Core::PropertyCollection<Options::FaceRecognizerOption>;
        std::shared_ptr<PropertiesType> m_options;

    public:

        /// <summary>
        /// Sets the string value for the specified option.
        /// </summary>
        /// <param name="key">The string value's key</param>
        /// <param name="value">The new string value</param>
        void Set(FaceRecognizerOption option, const SPXSTRING& value)
        {
            m_options->Set(option, value);
        }

        /// <summary>
        /// Sets the property boolean value for the specified key value.
        /// </summary>
        /// <param name="key">The boolean value's key</param>
        /// <param name="value">The new boolean value</param>
        void Set(FaceRecognizerOption option, bool value)
        {
            Set(option, std::to_string(value));
        }

        /// <summary>
        /// Sets the property integer value for the specified key value.
        /// </summary>
        /// <param name="key">The integer value's key</param>
        /// <param name="value">The new integer value</param>
        void Set(FaceRecognizerOption option, int value)
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
    /// Initializes a new instance of the FaceRecognizerOptions class.
    /// </summary>
    /// <returns>The newly created FaceRecognizerOptions wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<FaceRecognizerOptions> Create()
    {
        auto ptr = new FaceRecognizerOptions();
        return std::shared_ptr<FaceRecognizerOptions>(ptr);
    }

    /// <summary>
    /// Destructs an instance of the FaceRecognizerOptions class.
    /// </summary>
    ~FaceRecognizerOptions() = default;

    /// <summary>
    /// Advanced options and parameters
    /// </summary>
    AdvancedOptions& Advanced;

    // TODO: What strongly typed options go here?

protected:

    explicit FaceRecognizerOptions() :
        m_options(ProtectedAccess<AdvancedOptions>::Create()),
        Advanced(*m_options.get())
    {
    }

    explicit operator SPXHANDLE()
    {
        return ProtectedAccess<AdvancedOptions>::HandleFromPtr(m_options.get());
    }

private:

    DISABLE_COPY_AND_MOVE(FaceRecognizerOptions);
};

} } } } } // Azure::AI::Vision::Face::Options
