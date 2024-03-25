#pragma once
#include "decklink-cxx/src/main.rs.h"

class CXXInputCallback: public IDeckLinkInputCallback {
private:
	int32_t				m_refCount;
    RustInputCallback *callback;

public:
    CXXInputCallback(RustInputCallback *callback) {
        this->callback = callback;
    }

    HRESULT VideoInputFormatChanged(BMDVideoInputFormatChangedEvents notificationEvents, IDeckLinkDisplayMode* newDisplayMode, BMDDetectedVideoInputFormatFlags detectedSignalFlags) override;
    
    HRESULT VideoInputFrameArrived(IDeckLinkVideoInputFrame* videoFrame, IDeckLinkAudioInputPacket* audioPacket) override;

    HRESULT	STDMETHODCALLTYPE QueryInterface (REFIID iid, LPVOID *ppv) override;
	
	ULONG STDMETHODCALLTYPE AddRef() override;
	
	ULONG STDMETHODCALLTYPE Release() override;
};

class CXXOutputCallback: public IDeckLinkVideoOutputCallback {
private:
	int32_t				m_refCount;
    RustOutputCallback *callback;

public:
    CXXOutputCallback(RustOutputCallback *callback) {
        this->callback = callback;
    }

    HRESULT ScheduledFrameCompleted(IDeckLinkVideoFrame* completedFrame, BMDOutputFrameCompletionResult result) override;

    HRESULT ScheduledPlaybackHasStopped(void) override;

    HRESULT	STDMETHODCALLTYPE QueryInterface (REFIID iid, LPVOID *ppv) override;
	
	ULONG STDMETHODCALLTYPE AddRef() override;
	
	ULONG STDMETHODCALLTYPE Release() override;
};
