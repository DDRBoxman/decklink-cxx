#include "platform.h"

IDeckLinkIterator* CreateDeckLinkIteratorInstance(void) {
    HRESULT result = S_OK;

    IDeckLinkIterator *deckLinkIterator;
    result = CoCreateInstance(CLSID_CDeckLinkIterator, NULL, CLSCTX_ALL, IID_IDeckLinkIterator, (void**)deckLinkIterator);
    if (FAILED(result))
	{
        return nullptr;
	}
    return deckLinkIterator;
}

IDeckLinkAPIInformation* CreateDeckLinkAPIInformationInstance(void) {
    HRESULT result = S_OK;

    IDeckLinkAPIInformation *deckLinkAPIInformation;
    result = CoCreateInstance(CLSID_CDeckLinkAPIInformation, NULL, CLSCTX_ALL, IID_IDeckLinkAPIInformation, (void**)deckLinkAPIInformation);
    if (FAILED(result))
	{
        return nullptr;
	}
    return deckLinkAPIInformation;
}

/*
IDeckLinkDiscovery* CreateDeckLinkDiscoveryInstance(void);
IDeckLinkGLScreenPreviewHelper* CreateOpenGLScreenPreviewHelper(void);
IDeckLinkGLScreenPreviewHelper* CreateOpenGL3ScreenPreviewHelper(void);	// Requires OpenGL 3.2 support and provides improved performance and color handling
IDeckLinkVideoConversion* CreateVideoConversionInstance(void);
IDeckLinkVideoFrameAncillaryPackets* CreateVideoFrameAncillaryPacketsInstance(void);	// For use when creating a custom IDeckLinkVideoFrame without wrapping IDeckLinkOutput::CreateVideoFrame
*/
