#ifdef _WIN32
#include <decklink_win.h>
#elif __APPLE__
#include <decklink-cxx/decklink/Mac/include/DeckLinkAPI.h>
#elif __linux
#include <decklink-cxx/decklink/Linux/include/DeckLinkAPI.h>
#endif

#ifdef _WIN32
    IDeckLinkIterator* CreateDeckLinkIteratorInstance(void);
    IDeckLinkDiscovery* CreateDeckLinkDiscoveryInstance(void);
    IDeckLinkAPIInformation* CreateDeckLinkAPIInformationInstance(void);
    IDeckLinkGLScreenPreviewHelper* CreateOpenGLScreenPreviewHelper(void);
    IDeckLinkGLScreenPreviewHelper* CreateOpenGL3ScreenPreviewHelper(void);	// Requires OpenGL 3.2 support and provides improved performance and color handling
    IDeckLinkVideoConversion* CreateVideoConversionInstance(void);
    IDeckLinkVideoFrameAncillaryPackets* CreateVideoFrameAncillaryPacketsInstance(void);	// For use when creating a custom IDeckLinkVideoFrame without wrapping IDeckLinkOutput::CreateVideoFrame
#endif