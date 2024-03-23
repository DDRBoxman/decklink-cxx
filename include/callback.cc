#include "decklink-cxx/include/callback.h"

#include <iostream>

HRESULT CXXInputCallback::VideoInputFormatChanged (BMDVideoInputFormatChangedEvents notificationEvents, IDeckLinkDisplayMode* newDisplayMode, BMDDetectedVideoInputFormatFlags detectedSignalFlags) {
    this->callback->video_input_format_changed();
    return S_OK;
}

HRESULT CXXInputCallback::VideoInputFrameArrived(IDeckLinkVideoInputFrame* videoFrame, IDeckLinkAudioInputPacket* audioPacket) {
    this->callback->video_input_frame_arrived();
    return S_OK;
}

HRESULT	STDMETHODCALLTYPE CXXInputCallback::QueryInterface (REFIID iid, LPVOID *ppv)
{
    *ppv = NULL;
    return E_NOINTERFACE;
}

ULONG STDMETHODCALLTYPE CXXInputCallback::AddRef()
{
    return ++m_refCount;
}

ULONG STDMETHODCALLTYPE CXXInputCallback::Release()
{
    ULONG newRefValue = --m_refCount;
    if (newRefValue == 0)
        delete this;
    
    return newRefValue;
}

CXXInputCallback* new_input_callback(RustInputCallback *callback) {
  return new CXXInputCallback(callback);
}


HRESULT CXXOutputCallback::ScheduledFrameCompleted(IDeckLinkVideoFrame* completedFrame, BMDOutputFrameCompletionResult result) {
    this->callback->scheduled_frame_completed();
    return S_OK;
}

HRESULT CXXOutputCallback::ScheduledPlaybackHasStopped(void) {
    this->callback->scheduled_playback_has_stopped();
    return S_OK;
}

HRESULT	STDMETHODCALLTYPE CXXOutputCallback::QueryInterface (REFIID iid, LPVOID *ppv)
{
    *ppv = NULL;
    return E_NOINTERFACE;
}

ULONG STDMETHODCALLTYPE CXXOutputCallback::AddRef()
{
    return ++m_refCount;
}

ULONG STDMETHODCALLTYPE CXXOutputCallback::Release()
{
    ULONG newRefValue = --m_refCount;
    if (newRefValue == 0)
        delete this;
    
    return newRefValue;
}

CXXOutputCallback* new_output_callback(RustOutputCallback *callback) {
  return new CXXOutputCallback(callback);
}
