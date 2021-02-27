//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_properties.h>
#include <vision_api_c_frame_writer.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Input {
namespace Frames {

// TODO: What to do about stream parameter? (feedback from ryan)

/// <summary>
/// Represents the ability to write image frame data, for use as input w/ Vision AI scenario operations.
/// </summary>
class FrameWriter
{
public:

    /// <summary>
    /// Destructs an instance of the FrameWriter class.
    /// </summary>
    virtual ~FrameWriter()
    {
        if (vision_frame_writer_handle_is_valid(m_frameWriter))
        {
            vision_frame_writer_handle_release(m_frameWriter);
            m_frameWriter = SPXHANDLE_INVALID;
        }
    };

    void Close()
    {
        SPX_THROW_ON_FAIL(vision_frame_writer_close(m_frameWriter));
    }

    /// <summary>
    /// Gets the frame position for the stream specified.
    /// </summary>
    /// <param name="stream">The stream whose frame position is requested</param>
    /// <returns>The frame position for the stream specified</returns>
    /// <remarks>Frame positions start at 0, and increase by 1 for each call to WriteFrame.</remarks>
    uint64_t GetFramePos(uint32_t stream = 0) const
    {
        auto pos = vision_frame_writer_pos_get(m_frameWriter, stream, nullptr);
        SPX_IFTRUE_THROW_HR(pos == UINT64_MAX, SPXERR_UNHANDLED_EXCEPTION);
        return pos;
    }

    /// <summary>
    /// Writes a single frame of image data to the underlying FrameSource.
    /// </summary>
    /// <param name="stream">The stream to which to write the frame data</param>
    void WriteFrame(const uint8_t* data, size_t dataSizeInBytes, uint32_t stream = 0)
    {
        uint32_t size = (uint32_t)dataSizeInBytes; // idiomatic to accept size_t, but C API uses uint32_t
        SPX_THROW_ON_FAIL(vision_frame_writer_write(m_frameWriter, stream, data, size, nullptr, nullptr));
    }

    /// <summary>
    /// Writes a single property to the underlying FrameSource, using the current frame position.
    /// </summary>
    /// <param name="name">The property value's name</param>
    /// <param name="value">The property value's value</param>
    /// <param name="stream">The stream to which to write the property</param>
    void WriteProperty(const char* name, const char* value, uint32_t stream = 0)
    {
        auto pos = GetFramePos(stream);
        WriteProperty(name, value, stream, pos);
    }

    /// <summary>
    /// Writes a single property to the underlying FrameSource, at the specified frame position
    /// </summary>
    /// <param name="name">The property value's name</param>
    /// <param name="value">The property value's value</param>
    /// <param name="stream">The stream to which to write the property</param>
    /// <param name="framePos">The position in the stream to which to write associate the property</param>
    void WriteProperty(const char* name, const char* value, uint32_t stream, uint64_t framePos)
    {
        SPX_THROW_ON_FAIL(vision_frame_writer_property_write(m_frameWriter, stream, name, value, framePos, nullptr));
    }

protected:

    static std::shared_ptr<FrameWriter> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new FrameWriter(handle);
        return std::shared_ptr<FrameWriter>(ptr);
    }

    explicit FrameWriter(SPXHANDLE frameWriter) :
        m_frameWriter(frameWriter)
    {
    }

    explicit operator SPXHANDLE() { return m_frameWriter; }

private:

    DISABLE_DEFAULT_CTORS(FrameWriter);

    SPXHANDLE m_frameWriter;
};

} } } } } // Azure::AI::Vision::Input::Frames
