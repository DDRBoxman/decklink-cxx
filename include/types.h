#pragma once

class CXXInputCallback;
class CXXOutputCallback;
class ConversionFrame;

typedef long c_long;

#ifdef _WIN32
typedef HRESULT c_hresult;
typedef ULONG c_ulong;
typedef LONGLONG c_longlong;
typedef BMDDeckLinkAPIInformationID c_BMDDeckLinkAPIInformationID;
typedef BMDDisplayMode c_BMDDisplayMode;
typedef BMDPixelFormat c_BMDPixelFormat;
typedef BMDVideoInputFlags c_BMDVideoInputFlags;
typedef BMDVideoOutputFlags c_BMDVideoOutputFlags;
typedef BMDAncillaryPacketFormat c_BMDAncillaryPacketFormat;
#elif __APPLE__
#include <CoreFoundation/CFPlugInCOM.h>
typedef HRESULT c_hresult;
typedef ULONG c_ulong;
typedef int64_t c_longlong;
typedef uint32_t c_BMDDeckLinkAPIInformationID;
typedef uint32_t c_BMDDisplayMode;
typedef uint32_t c_BMDPixelFormat;
typedef uint32_t c_BMDVideoInputFlags;
typedef uint32_t c_BMDVideoOutputFlags;
typedef uint32_t c_BMDAncillaryPacketFormat;
#elif __linux
typedef int c_hresult;
typedef ULONG c_ulong;
typedef int64_t c_longlong;
typedef uint32_t c_BMDDeckLinkAPIInformationID;
typedef uint32_t c_BMDDisplayMode;
typedef uint32_t c_BMDPixelFormat;
typedef uint32_t c_BMDVideoInputFlags;
typedef uint32_t c_BMDVideoOutputFlags;
typedef uint32_t c_BMDAncillaryPacketFormat;
#endif