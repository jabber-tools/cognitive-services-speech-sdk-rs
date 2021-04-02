#[derive(Debug)]
pub enum SpeechSynthesisOutputFormat {
    /// Raw8Khz8BitMonoMULaw stands for raw-8khz-8bit-mono-mulaw
    Raw8Khz8BitMonoMULaw = 1,

    /// Riff16Khz16KbpsMonoSiren stands for riff-16khz-16kbps-mono-siren
    Riff16Khz16KbpsMonoSiren = 2,

    /// Audio16Khz16KbpsMonoSiren stands for audio-16khz-16kbps-mono-siren
    Audio16Khz16KbpsMonoSiren = 3,

    /// Audio16Khz32KBitRateMonoMp3 stands for audio-16khz-32kbitrate-mono-mp3
    Audio16Khz32KBitRateMonoMp3 = 4,

    /// Audio16Khz128KBitRateMonoMp3 stands for audio-16khz-128kbitrate-mono-mp3
    Audio16Khz128KBitRateMonoMp3 = 5,

    /// Audio16Khz64KBitRateMonoMp3 stands for audio-16khz-64kbitrate-mono-mp3
    Audio16Khz64KBitRateMonoMp3 = 6,

    /// Audio24Khz48KBitRateMonoMp3 stands for audio-24khz-48kbitrate-mono-mp3
    Audio24Khz48KBitRateMonoMp3 = 7,

    /// Audio24Khz96KBitRateMonoMp3 stands for audio-24khz-96kbitrate-mono-mp3
    Audio24Khz96KBitRateMonoMp3 = 8,

    /// Audio24Khz160KBitRateMonoMp3 stands for audio-24khz-160kbitrate-mono-mp3
    Audio24Khz160KBitRateMonoMp3 = 9,

    /// Raw16Khz16BitMonoTrueSilk stands for raw-16khz-16bit-mono-truesilk
    Raw16Khz16BitMonoTrueSilk = 10,

    /// Riff16Khz16BitMonoPcm stands for riff-16khz-16bit-mono-pcm
    Riff16Khz16BitMonoPcm = 11,

    /// Riff8Khz16BitMonoPcm stands for riff-8khz-16bit-mono-pcm
    Riff8Khz16BitMonoPcm = 12,

    /// Riff24Khz16BitMonoPcm stands for riff-24khz-16bit-mono-pcm
    Riff24Khz16BitMonoPcm = 13,

    /// Riff8Khz8BitMonoMULaw stands for riff-8khz-8bit-mono-mulaw
    Riff8Khz8BitMonoMULaw = 14,

    /// Raw16Khz16BitMonoPcm stands for raw-16khz-16bit-mono-pcm
    Raw16Khz16BitMonoPcm = 15,

    /// Raw24Khz16BitMonoPcm stands for raw-24khz-16bit-mono-pcm
    Raw24Khz16BitMonoPcm = 16,

    /// Raw8Khz16BitMonoPcm stands for raw-8khz-16bit-mono-pcm
    Raw8Khz16BitMonoPcm = 17,
}
