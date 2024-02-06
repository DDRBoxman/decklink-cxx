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