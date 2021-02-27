//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_frame_source.h>
#include <vision_api_cxx_properties.h>
#include <vision_api_c_source.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Input {

enum class VisionSourceProperty // TODO: Move to it's own file
{
    TODO = -1

    // TODO: What VisionSourceProperty's do we want?
};

/// <summary>
/// Represents a source of vision data, used as input Vision AI scenario operations.
/// </summary>
class VisionSource
{
private:

    template<typename Target> using ProtectedAccess = Azure::AI::Vision::Core::Details::ProtectedAccess<Target>;
    Core::Details::PrivatePropertyCollection<VisionSourceProperty> m_properties;

public:

    /// <summary>
    /// Initializes a new instance of the VisionSource class using the default capture device.
    /// </summary>
    /// <returns>The newly created VisionSource wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<VisionSource> FromDefaultCaptureDevice()
    {
        SPXHANDLE handle = SPXHANDLE_INVALID;
        SPX_THROW_ON_FAIL(::vision_source_handle_create(&handle, nullptr, nullptr, nullptr));
        return FromHandle(handle);
    }

    /// <summary>
    /// Initializes a new instance of the VisionSource class using the capture device specified.
    /// </summary>
    /// <param name="deviceAttributes">A string that specifies the attributes of the device (e.g. "front=true;", ...)</param>
    /// <returns>The newly created VisionSource wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<VisionSource> FromCaptureDevice(const SPXSTRING& deviceAttributes)
    {
        SPXHANDLE handle = SPXHANDLE_INVALID;
        SPX_THROW_ON_FAIL(::vision_source_handle_create(&handle, "source.device.attributes", Microsoft::CognitiveServices::Speech::Utils::ToUTF8(deviceAttributes).c_str(), nullptr));
        return FromHandle(handle);
    }

    /// <summary>
    /// Initializes a new instance of the VisionSource class using the locally accessible file specified.
    /// </summary>
    /// <param name="fileName">A string that specifies the locally accessible file</param>
    /// <returns>The newly created VisionSource wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<VisionSource> FromFile(const SPXSTRING& fileName)
    {
        SPXHANDLE handle = SPXHANDLE_INVALID;
        SPX_THROW_ON_FAIL(::vision_source_handle_create(&handle, "source.file.name", Microsoft::CognitiveServices::Speech::Utils::ToUTF8(fileName).c_str(), nullptr));
        return FromHandle(handle);
    }

    /// <summary>
    /// Initializes a new instance of the VisionSource class using the internet accessible URL specified.
    /// </summary>
    /// <param name="url">A string that specifies the internet accessible URL</param>
    /// <returns>The newly created VisionSource wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<VisionSource> FromUrl(const SPXSTRING& url)
    {
        SPXHANDLE handle = SPXHANDLE_INVALID;
        SPX_THROW_ON_FAIL(::vision_source_handle_create(&handle, "source.url", Microsoft::CognitiveServices::Speech::Utils::ToUTF8(url).c_str(), nullptr));
        return FromHandle(handle);
    }

    /// <summary>
    /// Initializes a new instance of the VisionSource class with a FrameSource w/ a FrameWriter.
    /// </summary>
    /// <param name="frameSource">A FrameSource obtained via FrameSource::FromFormat() or FrameSource::FromCallback()</param>
    /// <returns>The newly created VisionSource wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<VisionSource> FromFrameSource(const std::shared_ptr<Frames::FrameSource>& frameSource)
    {
        SPXHANDLE handle = SPXHANDLE_INVALID;
        SPXHANDLE frameSourceHandle = ProtectedAccess<Frames::FrameSource>::HandleFromPtr(frameSource.get());
        SPX_THROW_ON_FAIL(::vision_source_handle_create(&handle, nullptr, nullptr, frameSourceHandle));
        return FromHandle(handle);
    }

    /// <summary>
    /// Destructs an instance of the VisionSource class.
    /// </summary>
    virtual ~VisionSource()
    {
        if (vision_source_handle_is_valid(m_source))
        {
            ::vision_source_handle_release(m_source);
            m_source = SPXHANDLE_INVALID;
        }
    };

    /// <summary>
    /// Gets a collection of additional VisionSource properties.
    /// </summary>
    Core::PropertyCollection<VisionSourceProperty>& Properties;

protected:

    static std::shared_ptr<VisionSource> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new VisionSource(handle);
        return std::shared_ptr<VisionSource>(ptr);
    }

    explicit VisionSource(SPXHANDLE source) :
        m_properties(source, [](auto handle, auto* properties) { return vision_source_properties_handle_get(handle, properties); }),
        Properties(m_properties),
        m_source(source)
    {
    }

    explicit operator SPXHANDLE() { return m_source; }

private:

    DISABLE_DEFAULT_CTORS(VisionSource);

    SPXHANDLE m_source;
};

} } } } // Azure::AI::Vision::Input

PRIVATE_PROPERTY_COLLECTION_STATICS(Azure::AI::Vision::Input::VisionSourceProperty, "vision.source")
