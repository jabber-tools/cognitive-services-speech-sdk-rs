//
// Copyright (c) Microsoft. All rights reserved.
// See https://aka.ms/azai/vision/license202012 for the full license information.
//

#pragma once

#include <vision_api_cxx.h>

#ifdef VISION_HELLO_WORLD_SAMPLE

#include <cstdio>
#include <vector>

using namespace Azure::AI;
using namespace Azure::AI::Vision::Input;
using namespace Azure::AI::Vision::Input::Frames;
using namespace Azure::AI::Vision::Face;
using namespace Azure::AI::Vision::Face::Options;
using namespace Azure::AI::Vision::Service;

namespace Azure {
namespace AI {
namespace Vision {
namespace _Prototypes {
namespace Samples {

std::vector<uint8_t> GetFrameData(int bits, int width, int height, int sleepBetween = 0)
{
    std::this_thread::sleep_for(std::chrono::milliseconds(sleepBetween));
    auto size = width * height * bits / 8;
    auto data = std::vector<uint8_t>(size);
    return std::move(data);
}

std::string vision_sample_option_from_name(const std::string& name, int argc, const char** argv)
{
    for (int i = 0; i < argc - 1; i++)
    {
        if (name == argv[i])
        {
            return argv[i+1];
        }
    }
    return "";
}

int vision_hello_world_sample(int argc, const char** argv)
{
    auto service = [=]() {
        auto proxy = vision_sample_option_from_name("--proxy", argc, argv);
        auto proxyUser = vision_sample_option_from_name("--proxy.user", argc, argv);
        auto proxyPassword = vision_sample_option_from_name("--proxy.endpoint", argc, argv);
        auto endpoint = vision_sample_option_from_name("--endpoint", argc, argv);
        auto token = vision_sample_option_from_name("--auth.token", argc, argv);

        auto config = VisionServiceConfig::FromEndpoint(endpoint);

        // set the auth token, and other "strongly-typed" configuration properties
        config->SetAuthorizationToken(token);

        // set the proxy, optionally, and other "less used" configuration properties
        config->Advanced.Set(VisionServiceOption::HttpProxy, proxy);
        config->Advanced.Set(VisionServiceOption::HttpProxyPassword, proxyUser);
        config->Advanced.Set(VisionServiceOption::HttpProxyUserName, proxyPassword);

        // set other rarely used things here ... including http headers
        config->Advanced.Set("rare.use.cases.go.here", "value");
        config->Advanced.Set("http.header.X-blahblabhblah", "whatever");
        config->Advanced.Set("http.header.query.string.foobar", "whatever2");

        return config;
    }();

    auto options = [=]() {

        auto options = FaceRecognizerOptions::Create();

        options->Advanced.Set(FaceRecognizerOption::RequestResultMaxFaceCandidates, 5);
        options->Advanced.Set("rare.use.cases.go.here", "value");

        return options;
    }();

    auto input = [=]() {

        if (argc >= 2 && std::string("--device.default") == argv[1]) {
            return VisionSource::FromDefaultCaptureDevice();
        }
        else if (argc >= 3 && std::string("--device.attributes") == argv[1]) {
            return VisionSource::FromCaptureDevice(argv[2]);
        }
        else if (argc >= 3 && std::string("--file") == argv[1]) {
            return VisionSource::FromFile(argv[2]);
        }
        else if (argc >= 3 && std::string("--url") == argv[1]) {
            return VisionSource::FromUrl(argv[2]);
        }
        else if (argc >= 6 && std::string("--frames.push") == argv[1]) {

            auto bits = std::stoi(argv[2]);
            auto width = std::stoi(argv[3]);
            auto height = std::stoi(argv[4]);
            auto frames = std::stoi(argv[5]);

            auto format = FrameFormat::CreateRGBFormat(bits, width, height);
            auto source = FrameSource::FromFormat(format);

            auto writer = source->GetWriter();
            while (true) {

                auto data = GetFrameData(bits, width, height);
                writer->WriteFrame(data.data(), data.size());

                if (--frames <= 0) {
                    writer->Close();
                    break;
                }
            }

            return VisionSource::FromFrameSource(source);
        }
        else if (argc >= 7 && std::string("--frames.push.async") == argv[1]) {

            auto bits = std::stoi(argv[2]);
            auto width = std::stoi(argv[3]);
            auto height = std::stoi(argv[4]);
            auto frames = std::stoi(argv[5]);
            auto framesPtr = std::make_shared<int>(frames);
            auto sleepBetween = std::stoi(argv[6]);

            auto format = FrameFormat::CreateRGBFormat(bits, width, height);
            auto source = FrameSource::FromFormat(format);

            auto future = std::async(std::launch::async, [=] {
                auto& frames = *framesPtr.get();

                auto writer = source->GetWriter();
                while (true) {

                    auto data = GetFrameData(bits, width, height, sleepBetween);
                    writer->WriteFrame(data.data(), data.size());

                    if (--frames <= 0) {
                        writer->Close();
                        break;
                    }
                }
            });

            return VisionSource::FromFrameSource(source);
        }
        else if (argc >= 7 && std::string("--frames.callback") == argv[1]) {

            auto bits = std::stoi(argv[2]);
            auto width = std::stoi(argv[3]);
            auto height = std::stoi(argv[4]);
            auto frames = std::stoi(argv[5]);
            auto framesPtr = std::make_shared<int>(frames);
            auto sleepBetween = std::stoi(argv[6]);

            auto format = FrameFormat::CreateRGBFormat(bits, width, height);
            auto source = FrameSource::FromCallback(format, [=](auto& /* source */, auto& writer) {
                auto &frames = *framesPtr.get();

                auto data = GetFrameData(bits, width, height, sleepBetween);
                writer->WriteFrame(data.data(), data.size());

                if (--frames <= 0) {
                    writer->Close();
                }
            });

            return VisionSource::FromFrameSource(source);
        }
        else if (argc >= 7 && std::string("--frames.callback.async") == argv[1]) {

            auto bits = std::stoi(argv[2]);
            auto width = std::stoi(argv[3]);
            auto height = std::stoi(argv[4]);
            auto frames = std::stoi(argv[5]);
            auto framesPtr = std::make_shared<int>(frames);
            auto sleepBetween = std::stoi(argv[6]);

            auto format = FrameFormat::CreateRGBFormat(bits, width, height);
            auto source = FrameSource::FromCallback(format, [=](auto& /*source*/, auto& writer) {

                auto future = std::async(std::launch::async, [=] {
                    auto& frames = *framesPtr.get();

                    auto data = GetFrameData(bits, width, height, sleepBetween);
                    writer->WriteFrame(data.data(), data.size());

                    if (--frames <= 0) {
                        writer->Close();
                    }
                });
            });

            return VisionSource::FromFrameSource(source);
        }

        return VisionSource::FromDefaultCaptureDevice();
    }();

    auto recognizer = Face::FaceRecognizer::Create(service, input, options);

    auto id = recognizer->GetSessionId();
    printf("session.id=%s\n", id.c_str());

    recognizer->SessionStarted += [](const auto& e) {
        printf("SESSION STARTED: session.id=%s\n", e.GetSessionId().c_str());
    };

    recognizer->SessionStopped += [](const auto& e) {
        auto reason = e.GetReason();
        printf("SESSION STOPPED: session.id=%s, reason=%d\n", e.GetSessionId().c_str(), (int)reason);
    };

    recognizer->Recognized += [](const auto& e) {

        auto result = e.GetResult();

        auto reason = result->GetReason();
        auto resultId = result->GetResultId();
        printf("RECOGNIZED(EVENT): result.id=%s, result.reason=%d\n", resultId.c_str(), (int)reason);

        auto json = result->Properties.Get(Results::FaceResultProperty::Json);
        printf("RECOGNIZED(EVENT): json=%s\n", json.c_str());
    };

    auto once = argc >= 2 && std::string("--once") == argv[argc-1];
    if (once) {

        auto result = recognizer->RecognizeOnce();

        auto reason = result->GetReason();
        auto resultId = result->GetResultId();
        printf("RECOGNIZED(ONCE): result.id=%s, result.reason=%d\n", resultId.c_str(), (int)reason);

        auto json = result->Properties.Get(Results::FaceResultProperty::Json);
        printf("RECOGNIZED(ONCE): json=%s\n", json.c_str());
    }

    auto onceAsync = argc >= 2 && std::string("--once.async") == argv[argc-1];
    if (onceAsync) {

        auto recognized = recognizer->RecognizeOnceAsync();
        auto status = recognized.wait_for(std::chrono::seconds(60));
        SPX_DBG_ASSERT(status == std::future_status::ready); UNUSED(status);

        auto result = recognized.get();
        auto reason = result->GetReason();
        auto resultId = result->GetResultId();
        printf("RECOGNIZED(ONCE)(ASYNC): result.id=%s, result.reason=%d\n", resultId.c_str(), (int)reason);

        auto json = result->Properties.Get(Results::FaceResultProperty::Json);
        printf("RECOGNIZED(ONCE)(ASYNC): json=%s\n", json.c_str());
    }

    auto continuous = argc >= 2 && std::string("--continuous") == argv[argc-1];
    if (continuous) {

        recognizer->StartContinuousRecognition();
        auto stopped = recognizer->WaitForStop(std::chrono::seconds(60));
        if (!stopped) recognizer->StopContinuousRecognition();
    }

    auto continuousAsync = argc >= 2 && std::string("--continuous.async") == argv[argc-1];
    if (continuousAsync) {

        auto started = recognizer->StartContinuousRecognitionAsync();
        auto status = started.wait_for(std::chrono::seconds(600));
        SPX_TRACE_WARNING_IF(status != std::future_status::ready, "StartContinuousRecognitionAsync wasn't ready in 600 seconds");

        auto stopped = recognizer->WaitForStopAsync();
        status = stopped.wait_for(std::chrono::seconds(600));
        SPX_TRACE_WARNING_IF(status != std::future_status::ready, "WaitForStopAsync wasn't ready in 600 seconds");

        auto ensureStopped = recognizer->StopContinuousRecognitionAsync();
        status = ensureStopped.wait_for(std::chrono::seconds(600));
        SPX_TRACE_WARNING_IF(status != std::future_status::ready, "StopContinuousRecognitionAsync wasn't ready in 600 seconds");
    }

    return 1;
}


} } } } } // Azure::AI::Vision::Prototypes::Samples

#endif
