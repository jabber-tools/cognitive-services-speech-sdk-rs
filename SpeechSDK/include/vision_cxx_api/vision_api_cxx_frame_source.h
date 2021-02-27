//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_properties.h>
#include <vision_api_cxx_frame_format.h>
#include <vision_api_cxx_frame_writer.h>
#include <vision_api_c_frame_source.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Input {
namespace Frames {

enum class FrameSourceProperty // TODO: Move to it's own file
{
    TODO = -1

    // TODO: What FrameSourceProperty's do we want?
};

/// <summary>
/// Represents a source of image frame data, used as input (or output) to (or from) Vision AI scenario operations.
/// </summary>
class FrameSource
{
protected:

    template<typename Target> using ProtectedAccess = ::Azure::AI::Vision::Core::Details::ProtectedAccess<Target>;
    Core::Details::PrivatePropertyCollection<FrameSourceProperty> m_properties;

public:

    using CallbackFunction_Type = ::std::function<void(const std::shared_ptr<FrameSource>& source, const std::shared_ptr<FrameWriter>& writer)>;

    /// <summary>
    /// Initializes a new instance of the FrameSource class.
    /// </summary>
    /// <param name="format">A FrameFormat obtained via FrameFormat::Create() or similar</param>
    /// <returns>The newly created FrameSource wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<FrameSource> FromFormat(const std::shared_ptr<FrameFormat>& format)
    {
        SPXHANDLE handle = SPXHANDLE_INVALID;
        SPXHANDLE formatHandle = ProtectedAccess<FrameFormat>::HandleFromPtr(format.get());
        SPX_THROW_ON_FAIL(::vision_frame_source_handle_create(&handle, nullptr, nullptr, formatHandle));
        return FromHandle(handle);
    }

    /// <summary>
    /// Initializes a new instance of the FrameSource class.
    /// </summary>
    /// <param name="format">A FrameFormat obtained via FrameFormat::Create() or similar</param>
    /// <param name="callback">TODO</param>
    /// <returns>The newly created FrameSource wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<FrameSource> FromCallback(const std::shared_ptr<FrameFormat>& format, CallbackFunction_Type callback)
    {
        SPXHANDLE handle = SPXHANDLE_INVALID;
        SPXHANDLE formatHandle = ProtectedAccess<FrameFormat>::HandleFromPtr(format.get());
        SPX_THROW_ON_FAIL(::vision_frame_source_handle_create(&handle, nullptr, nullptr, formatHandle));

        // TODO: ❓ need to spec, write, and use here: vision_frame_source_callback_set
        UNUSED(callback);

        return FromHandle(handle);
    }

    /// <summary>
    /// Destructs an instance of the FrameSource class.
    /// </summary>
    virtual ~FrameSource()
    {
        if (vision_frame_source_handle_is_valid(m_frameSource))
        {
            ::vision_frame_source_handle_release(m_frameSource);
            m_frameSource = SPXHANDLE_INVALID;
        }
    };

    /// <summary>
    /// Gets the FrameWriter used to write frame data into this FrameSource object
    /// </summary>
    /// <returns>The FrameWriter used to write frame data, wrapped inside a std::shared_ptr</returns>
    std::shared_ptr<FrameWriter> GetWriter()
    {
        SPXHANDLE handle = SPXHANDLE_INVALID;
        SPX_THROW_ON_FAIL(vision_frame_source_writer_handle_get(m_frameSource, &handle));
        return ProtectedAccess<FrameWriter>::FromHandle(handle);
    }

    /// <summary>
    /// Gets a collection of additional FrameSource properties.
    /// </summary>
    const Core::PropertyCollection<FrameSourceProperty>& Properties;

protected:

    static std::shared_ptr<FrameSource> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new FrameSource(handle);
        return std::shared_ptr<FrameSource>(ptr);
    }

    explicit FrameSource(SPXHANDLE frameSource) :
        m_properties(frameSource, [](auto handle, auto* properties) { return vision_frame_source_properties_handle_get(handle, properties); }),
        Properties(m_properties),
        m_frameSource(frameSource)
    {
    }

    explicit operator SPXHANDLE() { return m_frameSource; }

private:

    DISABLE_DEFAULT_CTORS(FrameSource);

    SPXHANDLE m_frameSource;
};

} } } } } // Azure::AI::Vision::Input::Frames

PRIVATE_PROPERTY_COLLECTION_STATICS(Azure::AI::Vision::Input::Frames::FrameSourceProperty, "frame.source")
