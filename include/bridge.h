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

