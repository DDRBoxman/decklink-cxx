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

void FillBlue(IDeckLinkMutableVideoFrame* theFrame);

void Release(IUnknown *obj);

CXXInputCallback* new_input_callback(RustInputCallback *callback);

CXXOutputCallback* new_output_callback(RustOutputCallback *callback);

