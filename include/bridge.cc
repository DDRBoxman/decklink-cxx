#include "bridge.h"

#ifdef _WIN32
#include <comdef.h>

rust::String GetDisplayName(IDeckLink *deckLink) {
    BSTR name;
    deckLink->GetDisplayName(&name);

	return _bstr_t(name, false);
}

rust::String GetDisplayModeName(IDeckLinkDisplayMode *displayMode) {
	BSTR name;
    displayMode->GetName(&name);

    return _bstr_t(name, false);
}
#elif __APPLE__
#include <CoreFoundation/CoreFoundation.h>

rust::String GetDisplayName(IDeckLink *deckLink) {
    CFStringRef name;
    deckLink->GetDisplayName(&name);

    std::string returnString("");
	CFIndex stringSize = CFStringGetLength(name) + 1;
	char stringBuffer[stringSize];
	if (CFStringGetCString(name, stringBuffer, stringSize, kCFStringEncodingUTF8))
		returnString = stringBuffer;
	return returnString;
}

rust::String GetDisplayModeName(IDeckLinkDisplayMode *displayMode) {
	CFStringRef name;
    displayMode->GetName(&name);

    std::string returnString("");
	CFIndex stringSize = CFStringGetLength(name) + 1;
	char stringBuffer[stringSize];
	if (CFStringGetCString(name, stringBuffer, stringSize, kCFStringEncodingUTF8))
		returnString = stringBuffer;
	return returnString;
}
#elif __linux
rust::String GetDisplayName(IDeckLink *deckLink) {
    const char* name;
    deckLink->GetDisplayName(&name);

	return std::string(name);
}

rust::String GetDisplayModeName(IDeckLinkDisplayMode *displayMode) {
	const char* name;
    displayMode->GetName(&name);

    return std::string(name);
}
#endif

HRESULT GetInput(IDeckLink * deckLink, IDeckLinkInput** deckLinkInput) {
    return deckLink->QueryInterface(IID_IDeckLinkInput, (void**)deckLinkInput);
}

HRESULT GetOutput(IDeckLink * deckLink, IDeckLinkOutput** deckLinkOutput) {
    return deckLink->QueryInterface(IID_IDeckLinkOutput, (void**)deckLinkOutput);
}

HRESULT GetAncillaryPackets(IDeckLinkVideoFrame *videoFrame, IDeckLinkVideoFrameAncillaryPackets** videoFrameAncillaryPackets) {
	return videoFrame->QueryInterface(IID_IDeckLinkVideoFrame, (void**)videoFrameAncillaryPackets);
}

HRESULT GetAncillaryPacketBytes(IDeckLinkAncillaryPacket *packet, BMDAncillaryPacketFormat format, const uint8_t** data, uint32_t* size) {
	return packet->GetBytes(format, (const void**)data, size);
}

HRESULT GetFrameBytes(IDeckLinkVideoFrame *frame, uint8_t** buffer) {
	return frame->GetBytes((void**)buffer);
}

void Release(IUnknown *obj) {
    obj->Release();
}

long ConversionFrame::GetWidth(void) {
	return width;
}

long ConversionFrame::GetHeight(void) {
	return height;
}

long ConversionFrame::GetRowBytes(void) {
	return rowBytes;
}

BMDPixelFormat ConversionFrame::GetPixelFormat(void) {
	return pixelFormat;
}

HRESULT	STDMETHODCALLTYPE ConversionFrame::QueryInterface (REFIID iid, LPVOID *ppv)
{
    *ppv = NULL;
    return E_NOINTERFACE;
}

ULONG STDMETHODCALLTYPE ConversionFrame::AddRef()
{
    return ++m_refCount;
}

ULONG STDMETHODCALLTYPE ConversionFrame::Release()
{
    ULONG newRefValue = --m_refCount;
    if (newRefValue == 0)
        delete this;

    return newRefValue;
}

BMDFrameFlags ConversionFrame::GetFlags()
{
	return m_frameFlags;
}

HRESULT ConversionFrame::GetBytes(void **buffer)
{
	if (buffer == nullptr || m_frameData.empty())
		return E_FAIL;
	
	*buffer = (void*)m_frameData.data();

	return S_OK;
}

HRESULT ConversionFrame::GetTimecode(BMDTimecodeFormat format, IDeckLinkTimecode **timecode)
{
	(void)format;   // unused
	(void)timecode; // unused
	return E_NOTIMPL;
}

HRESULT ConversionFrame::GetAncillaryData(IDeckLinkVideoFrameAncillary **ancillary)
{
	(void)ancillary; // unused
	return E_NOTIMPL;
}

ConversionFrame* new_conversion_frame(
    long width,
    long height,
    long row_bytes,
    BMDPixelFormat pixel_format
) {
	return new ConversionFrame(width, height, row_bytes, pixel_format);
}