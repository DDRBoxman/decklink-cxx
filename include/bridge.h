#include "decklink-cxx/decklink/Mac/include/DeckLinkAPI.h"
#include <CoreFoundation/CoreFoundation.h>
#include "rust/cxx.h"


rust::String GetDisplayName(IDeckLink * deckLink) {
    CFStringRef name;
    deckLink->GetDisplayName(&name);

    std::string returnString("");
	CFIndex stringSize = CFStringGetLength(name) + 1;
	char stringBuffer[stringSize];
	if (CFStringGetCString(name, stringBuffer, stringSize, kCFStringEncodingUTF8))
		returnString = stringBuffer;
	return returnString;
}

HRESULT GetOutput(IDeckLink * deckLink, IDeckLinkOutput** deckLinkOutput) {
    return deckLink->QueryInterface(IID_IDeckLinkOutput, (void**)deckLinkOutput);
}

const uint32_t kFrameDuration = 1000;
const uint32_t kTimeScale = 25000;
const uint32_t kFrameWidth = 1920;
const uint32_t kFrameHeight = 1080;

// 10-bit YUV row bytes, ref. SDK Manual "2.7.4 Pixel Formats" / bmdFormat10BitYUV
const uint32_t kRowBytes = ((kFrameWidth + 47) / 48) * 128;


const uint32_t kBlueData[] = { 0x40aa298, 0x2a8a62a8, 0x298aa040, 0x2a8102a8 };

static void FillBlue(IDeckLinkMutableVideoFrame* theFrame)
{
	uint32_t* nextWord;
	uint32_t  wordsRemaining;
	
	theFrame->GetBytes((void**)&nextWord);
	wordsRemaining = (kRowBytes * kFrameHeight) / 4;
	
	while (wordsRemaining > 0)
	{
		*(nextWord++) = kBlueData[0];
		*(nextWord++) = kBlueData[1];
		*(nextWord++) = kBlueData[2];
		*(nextWord++) = kBlueData[3];
		wordsRemaining = wordsRemaining - 4;
	}
}

void Release(IUnknown *obj) {
    obj->Release();
}
