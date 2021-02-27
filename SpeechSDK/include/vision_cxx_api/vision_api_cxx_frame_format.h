//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once
#include <vision_api_cxx_common.h>
#include <vision_api_cxx_properties.h>
#include <vision_api_c_frame_format.h>

namespace Azure {
namespace AI {
namespace Vision {
namespace Input {
namespace Frames {

enum class FrameFormatProperty // TODO: Move to it's own file
{
    TODO = -1

    // TODO: What FrameFormatProperty's do we want?
};

/// <summary>
/// Represents a collection of image format properties (e.g. FOURCC, width, height, stride, ...)
/// </summary>
class FrameFormat
{
protected:

    template<typename Target> using ProtectedAccess = Core::Details::ProtectedAccess<Target>;
    Core::Details::PrivatePropertyCollection<FrameFormatProperty> m_properties;

public:

    /// <summary>
    /// Initializes a new instance of the FrameFormat class based on a FourCC value.
    /// </summary>
    /// <param name="ch1">FOURCC character 1</param>
    /// <param name="ch2">FOURCC character 2</param>
    /// <param name="ch3">FOURCC character 3</param>
    /// <param name="ch4">FOURCC character 4</param>
    /// <param name="width">The image format's pixel width</param>
    /// <param name="height">The image format's pixel height</param>
    /// <param name="stride">The image format's pixel stride</param>
    /// <returns>The newly created FrameFormat wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<FrameFormat> CreateFourCCFormat(char ch1, char ch2, char ch3, char ch4, int width = 0, int height = 0, int stride = 0)
    {
        auto properties = Core::PropertyCollection<>::Create();
        properties->Set("frame.format.image.width", std::to_string(width));
        properties->Set("frame.format.image.height", std::to_string(height));
        SPX_IFTRUE(stride > 0, properties->Set("frame.format.image.stride", std::to_string(stride)));

        SPXHANDLE handle = SPXHANDLE_INVALID;
        SPXHANDLE propertiesHandle = ProtectedAccess<Core::PropertyCollection<>>::HandleFromPtr(properties.get());
        SPX_THROW_ON_FAIL(vision_frame_format_handle_create(&handle, ch1, ch2, ch3, ch4, propertiesHandle));
        return FromHandle(handle);
    }

    /// <summary>
    /// Initializes a new instance of the FrameFormat class for uncompressed 32bit ARGB images.
    /// </summary>
    /// <param name="bitsPerPixel">The image format's bits per pixel (e.g. 8, 16, 24, 32, ...)</param>
    /// <param name="width">The image format's pixel width (e.g. 640)</param>
    /// <param name="height">The image format's pixel height (e.g. 480)</param>
    /// <param name="stride">The image format's pixel stride</param>
    /// <returns>The newly created FrameFormat wrapped inside a std::shared_ptr</returns>
    static std::shared_ptr<FrameFormat> CreateRGBFormat(int bitsPerPixel, int width, int height, int stride = 0)
    {
        auto format = CreateFourCCFormat('R', 'G', 'B', ' ', width, height, stride);
        format->m_properties.Set("frame.format.image.bits.per.pixel", std::to_string(bitsPerPixel));
        return format;
    }

    /// <summary>
    /// Destructs an instance of the FrameFormat class.
    /// </summary>
    virtual ~FrameFormat()
    {
        if (vision_frame_format_handle_is_valid(m_format))
        {
            ::vision_frame_format_handle_release(m_format);
            m_format = SPXHANDLE_INVALID;
        }
    };

    /// <summary>
    /// Gets the image format's FOURCC value
    /// </summary>
    /// <returns>
    /// The FOURCC format code as a string
    /// </returns>
    SPXSTRING GetFourCC() const
    {
        return m_properties.Get("frame.format.image.fourcc");
    }

    /// <summary>
    /// Gets the image format's FOURCC value
    /// </summary>
    /// <param name="ch1">Pointer to char to receive FOURCC character 1</param>
    /// <param name="ch2">Pointer to char to receive FOURCC character 2</param>
    /// <param name="ch3">Pointer to char to receive FOURCC character 3</param>
    /// <param name="ch4">Pointer to char to receive FOURCC character 4</param>
    void GetFourCC(char* ch1, char* ch2, char* ch3, char* ch4) const
    {
        auto code = m_properties.Get("frame.format.image.fourcc.1");
        *ch1 = code[0];
        code = m_properties.Get("frame.format.image.fourcc.2");
        *ch2 = code[0];
        code = m_properties.Get("frame.format.image.fourcc.3");
        *ch3 = code[0];
        code = m_properties.Get("frame.format.image.fourcc.4");
        *ch4 = code[0];
    }

    /// <summary>
    /// Gets the image format's pixel width.
    /// </summary>
    /// <returns>
    /// The image pixel width.
    /// </returns>
    int GetWidth() const
    {
        auto width = m_properties.Get("frame.format.image.width");
        return std::stoi(width);
    }

    /// <summary>
    /// Gets the image format's pixel height.
    /// </summary>
    /// <returns>
    /// The image pixel height.
    /// </returns>
    int GetHeight() const
    {
        auto height = m_properties.Get("frame.format.image.height");
        return std::stoi(height);
    }

    /// <summary>
    /// Gets the image format's pixel stride.
    /// </summary>
    /// <returns>
    /// The image pixel stride.
    /// </returns>
    int GetStride() const
    {
        auto width = m_properties.Get("frame.format.image.stride");
        return std::stoi(width);
    }

    /// <summary>
    /// Sets the image format's bits per pixel value.
    /// </summary>
    /// <param name="bitsPerPixel">The image's bits per pixel value.</param>
    void SetBitsPerPixel(int bitsPerPixel)
    {
        m_properties.Set("frame.format.image.bits.per.pixel", std::to_string(bitsPerPixel));
    }

    /// <summary>
    /// Gets the image format's bits per pixel value.
    /// </summary>
    /// <returns>
    /// The image's bits per pixel value.
    /// </returns>
    int GetBitsPerPixel() const
    {
        auto bitsPerPixel = m_properties.Get("frame.format.image.bits.per.pixel");
        return std::stoi(bitsPerPixel);
    }

    /// <summary>
    /// Gets a collection of additional FrameFormat properties.
    /// </summary>
    Core::PropertyCollection<FrameFormatProperty>& Properties;

protected:

    static std::shared_ptr<FrameFormat> FromHandle(SPXHANDLE handle)
    {
        auto ptr = new FrameFormat(handle);
        return std::shared_ptr<FrameFormat>(ptr);
    }

    explicit FrameFormat(SPXHANDLE format) :
        m_properties(format, [](auto handle, auto* properties) { return vision_frame_format_properties_handle_get(handle, properties); }),
        Properties(m_properties),
        m_format(format)
    {
    }

    explicit operator SPXHANDLE() { return m_format; }

private:

    DISABLE_DEFAULT_CTORS(FrameFormat);

    SPXHANDLE m_format;
};

} } } } } // Azure::AI::Vision::Input::Frames

PRIVATE_PROPERTY_COLLECTION_STATICS(Azure::AI::Vision::Input::Frames::FrameFormatProperty, "frame.format")
