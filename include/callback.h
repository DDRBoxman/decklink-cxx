#pragma once
#include "decklink-cxx/src/main.rs.h"

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
