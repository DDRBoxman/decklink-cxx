#pragma once

struct RustInputCallback;
struct RustOutputCallback;

#include "rust/cxx.h"
#include "decklink-cxx/src/bridge.rs.h"

rust::String GetDisplayName(IDeckLink *deckLink);
rust::String GetDisplayModeName(IDeckLinkDisplayMode *displayMode);

HRESULT GetInput(IDeckLink * deckLink, IDeckLinkInput** deckLinkInput);

HRESULT GetOutput(IDeckLink * deckLink, IDeckLinkOutput** deckLinkOutput);

HRESULT GetAncillaryPackets(IDeckLinkVideoFrame *videoFrame, IDeckLinkVideoFrameAncillaryPackets** videoFrameAncillaryPackets);

HRESULT GetAncillaryPacketBytes(IDeckLinkAncillaryPacket *packet, BMDAncillaryPacketFormat format, const uint8_t** data, uint32_t* size);

HRESULT GetFrameBytes(IDeckLinkVideoFrame *frame, uint8_t** buffer);

void Release(IUnknown *obj);

CXXInputCallback* new_input_callback(RustInputCallback *callback);

CXXOutputCallback* new_output_callback(RustOutputCallback *callback);

ConversionFrame* new_conversion_frame(
    long width,
    long height,
    long row_bytes,
    BMDPixelFormat pixel_format
);

class ConversionFrame: public IDeckLinkVideoFrame {
private:
    long width;
    long height;
    long rowBytes;
    BMDPixelFormat pixelFormat;
    int32_t	m_refCount;
    BMDFrameFlags		m_frameFlags;
	std::vector<char>	m_frameData;
	BMDTimeValue		m_frameTime;
	BMDTimeValue		m_frameDuration;
	BMDTimeScale		m_timeScale;

    static const BMDTimeScale kDefaultConversionTimeScale = 100000;


public:    
    ConversionFrame(long width, long height, long rowBytes, BMDPixelFormat pixelFormat) : 
        width(width),
        height(height),
        rowBytes(rowBytes),
        pixelFormat(pixelFormat),
        m_frameFlags(bmdFrameFlagDefault),
        m_frameTime(0),
        m_frameDuration(0),
        m_timeScale(kDefaultConversionTimeScale),
        m_refCount(1) {

            try
            {
                m_frameData.resize(rowBytes * height);
            }
            catch(...)
            {
            }

    }

    long GetWidth(void) override;
    long GetHeight(void) override;
    long GetRowBytes(void) override;
    BMDPixelFormat GetPixelFormat(void) override;
    BMDFrameFlags GetFlags() override;
	HRESULT GetBytes(void **buffer) override;
	HRESULT GetTimecode(BMDTimecodeFormat format, IDeckLinkTimecode **timecode) override;
	HRESULT GetAncillaryData(IDeckLinkVideoFrameAncillary **ancillary) override;

    HRESULT	STDMETHODCALLTYPE QueryInterface (REFIID iid, LPVOID *ppv) override;
	
	ULONG STDMETHODCALLTYPE AddRef() override;
	
	ULONG STDMETHODCALLTYPE Release() override;
};