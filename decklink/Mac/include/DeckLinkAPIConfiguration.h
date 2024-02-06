/* -LICENSE-START-
** Copyright (c) 2024 Blackmagic Design
**
** Permission is hereby granted, free of charge, to any person or organization
** obtaining a copy of the software and accompanying documentation covered by
** this license (the "Software") to use, reproduce, display, distribute,
** execute, and transmit the Software, and to prepare derivative works of the
** Software, and to permit third-parties to whom the Software is furnished to
** do so, all subject to the following:
**
** The copyright notices in the Software and this entire statement, including
** the above license grant, this restriction and the following disclaimer,
** must be included in all copies of the Software, in whole or in part, and
** all derivative works of the Software, unless such copies or derivative
** works are solely in the form of machine-executable object code generated by
** a source language processor.
**
** THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
** IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
** FITNESS FOR A PARTICULAR PURPOSE, TITLE AND NON-INFRINGEMENT. IN NO EVENT
** SHALL THE COPYRIGHT HOLDERS OR ANYONE DISTRIBUTING THE SOFTWARE BE LIABLE
** FOR ANY DAMAGES OR OTHER LIABILITY, WHETHER IN CONTRACT, TORT OR OTHERWISE,
** ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
** DEALINGS IN THE SOFTWARE.
** -LICENSE-END-
*/

#ifndef BMD_DECKLINKAPICONFIGURATION_H
#define BMD_DECKLINKAPICONFIGURATION_H


#ifndef BMD_CONST
    #if defined(_MSC_VER)
        #define BMD_CONST __declspec(selectany) static const
    #else
        #define BMD_CONST static const
    #endif
#endif

#ifndef BMD_PUBLIC
	#define BMD_PUBLIC
#endif

// Type Declarations


// Interface ID Declarations

BMD_CONST REFIID IID_IDeckLinkConfiguration                       = /* 912F634B-2D4E-40A4-8AAB-8D80B73F1289 */ { 0x91,0x2F,0x63,0x4B,0x2D,0x4E,0x40,0xA4,0x8A,0xAB,0x8D,0x80,0xB7,0x3F,0x12,0x89 };
BMD_CONST REFIID IID_IDeckLinkEncoderConfiguration                = /* 138050E5-C60A-4552-BF3F-0F358049327E */ { 0x13,0x80,0x50,0xE5,0xC6,0x0A,0x45,0x52,0xBF,0x3F,0x0F,0x35,0x80,0x49,0x32,0x7E };

/* Enum BMDDeckLinkConfigurationID - DeckLink Configuration ID */

typedef uint32_t BMDDeckLinkConfigurationID;
enum _BMDDeckLinkConfigurationID {

    /* Serial port Flags */

    bmdDeckLinkConfigSwapSerialRxTx                              = /* 'ssrt' */ 0x73737274,

    /* Video Input/Output Integers */

    bmdDeckLinkConfigHDMI3DPackingFormat                         = /* '3dpf' */ 0x33647066,
    bmdDeckLinkConfigBypass                                      = /* 'byps' */ 0x62797073,
    bmdDeckLinkConfigClockTimingAdjustment                       = /* 'ctad' */ 0x63746164,

    /* Audio Input/Output Flags */

    bmdDeckLinkConfigAnalogAudioConsumerLevels                   = /* 'aacl' */ 0x6161636C,
    bmdDeckLinkConfigSwapHDMICh3AndCh4OnInput                    = /* 'hi34' */ 0x68693334,
    bmdDeckLinkConfigSwapHDMICh3AndCh4OnOutput                   = /* 'ho34' */ 0x686F3334,

    /* Video Output Flags */

    bmdDeckLinkConfigFieldFlickerRemoval                         = /* 'fdfr' */ 0x66646672,
    bmdDeckLinkConfigHD1080p24ToHD1080i5994Conversion            = /* 'to59' */ 0x746F3539,
    bmdDeckLinkConfig444SDIVideoOutput                           = /* '444o' */ 0x3434346F,
    bmdDeckLinkConfigBlackVideoOutputDuringCapture               = /* 'bvoc' */ 0x62766F63,
    bmdDeckLinkConfigLowLatencyVideoOutput                       = /* 'llvo' */ 0x6C6C766F,
    bmdDeckLinkConfigDownConversionOnAllAnalogOutput             = /* 'caao' */ 0x6361616F,
    bmdDeckLinkConfigSMPTELevelAOutput                           = /* 'smta' */ 0x736D7461,
    bmdDeckLinkConfigRec2020Output                               = /* 'rec2' */ 0x72656332,	// Ensure output is Rec.2020 colorspace
    bmdDeckLinkConfigQuadLinkSDIVideoOutputSquareDivisionSplit   = /* 'SDQS' */ 0x53445153,
    bmdDeckLinkConfigOutput1080pAsPsF                            = /* 'pfpr' */ 0x70667072,

    /* Video Output Integers */

    bmdDeckLinkConfigVideoOutputConnection                       = /* 'vocn' */ 0x766F636E,
    bmdDeckLinkConfigVideoOutputConversionMode                   = /* 'vocm' */ 0x766F636D,
    bmdDeckLinkConfigAnalogVideoOutputFlags                      = /* 'avof' */ 0x61766F66,
    bmdDeckLinkConfigReferenceInputTimingOffset                  = /* 'glot' */ 0x676C6F74,
    bmdDeckLinkConfigReferenceOutputMode                         = /* 'glOm' */ 0x676C4F6D,
    bmdDeckLinkConfigVideoOutputIdleOperation                    = /* 'voio' */ 0x766F696F,
    bmdDeckLinkConfigDefaultVideoOutputMode                      = /* 'dvom' */ 0x64766F6D,
    bmdDeckLinkConfigDefaultVideoOutputModeFlags                 = /* 'dvof' */ 0x64766F66,
    bmdDeckLinkConfigSDIOutputLinkConfiguration                  = /* 'solc' */ 0x736F6C63,
    bmdDeckLinkConfigHDMITimecodePacking                         = /* 'htpk' */ 0x6874706B,
    bmdDeckLinkConfigPlaybackGroup                               = /* 'plgr' */ 0x706C6772,

    /* Video Output Floats */

    bmdDeckLinkConfigVideoOutputComponentLumaGain                = /* 'oclg' */ 0x6F636C67,
    bmdDeckLinkConfigVideoOutputComponentChromaBlueGain          = /* 'occb' */ 0x6F636362,
    bmdDeckLinkConfigVideoOutputComponentChromaRedGain           = /* 'occr' */ 0x6F636372,
    bmdDeckLinkConfigVideoOutputCompositeLumaGain                = /* 'oilg' */ 0x6F696C67,
    bmdDeckLinkConfigVideoOutputCompositeChromaGain              = /* 'oicg' */ 0x6F696367,
    bmdDeckLinkConfigVideoOutputSVideoLumaGain                   = /* 'oslg' */ 0x6F736C67,
    bmdDeckLinkConfigVideoOutputSVideoChromaGain                 = /* 'oscg' */ 0x6F736367,

    /* Video Input Flags */

    bmdDeckLinkConfigVideoInputScanning                          = /* 'visc' */ 0x76697363,	// Applicable to H264 Pro Recorder only
    bmdDeckLinkConfigUseDedicatedLTCInput                        = /* 'dltc' */ 0x646C7463,	// Use timecode from LTC input instead of SDI stream
    bmdDeckLinkConfigSDIInput3DPayloadOverride                   = /* '3dds' */ 0x33646473,
    bmdDeckLinkConfigCapture1080pAsPsF                           = /* 'cfpr' */ 0x63667072,

    /* Video Input Integers */

    bmdDeckLinkConfigVideoInputConnection                        = /* 'vicn' */ 0x7669636E,
    bmdDeckLinkConfigAnalogVideoInputFlags                       = /* 'avif' */ 0x61766966,
    bmdDeckLinkConfigVideoInputConversionMode                    = /* 'vicm' */ 0x7669636D,
    bmdDeckLinkConfig32PulldownSequenceInitialTimecodeFrame      = /* 'pdif' */ 0x70646966,
    bmdDeckLinkConfigVANCSourceLine1Mapping                      = /* 'vsl1' */ 0x76736C31,
    bmdDeckLinkConfigVANCSourceLine2Mapping                      = /* 'vsl2' */ 0x76736C32,
    bmdDeckLinkConfigVANCSourceLine3Mapping                      = /* 'vsl3' */ 0x76736C33,
    bmdDeckLinkConfigCapturePassThroughMode                      = /* 'cptm' */ 0x6370746D,
    bmdDeckLinkConfigCaptureGroup                                = /* 'cpgr' */ 0x63706772,

    /* Video Input Floats */

    bmdDeckLinkConfigVideoInputComponentLumaGain                 = /* 'iclg' */ 0x69636C67,
    bmdDeckLinkConfigVideoInputComponentChromaBlueGain           = /* 'iccb' */ 0x69636362,
    bmdDeckLinkConfigVideoInputComponentChromaRedGain            = /* 'iccr' */ 0x69636372,
    bmdDeckLinkConfigVideoInputCompositeLumaGain                 = /* 'iilg' */ 0x69696C67,
    bmdDeckLinkConfigVideoInputCompositeChromaGain               = /* 'iicg' */ 0x69696367,
    bmdDeckLinkConfigVideoInputSVideoLumaGain                    = /* 'islg' */ 0x69736C67,
    bmdDeckLinkConfigVideoInputSVideoChromaGain                  = /* 'iscg' */ 0x69736367,

    /* Keying Integers */

    bmdDeckLinkConfigInternalKeyingAncillaryDataSource           = /* 'ikas' */ 0x696B6173,

    /* Audio Input Flags */

    bmdDeckLinkConfigMicrophonePhantomPower                      = /* 'mphp' */ 0x6D706870,

    /* Audio Input Integers */

    bmdDeckLinkConfigAudioInputConnection                        = /* 'aicn' */ 0x6169636E,

    /* Audio Input Floats */

    bmdDeckLinkConfigAnalogAudioInputScaleChannel1               = /* 'ais1' */ 0x61697331,
    bmdDeckLinkConfigAnalogAudioInputScaleChannel2               = /* 'ais2' */ 0x61697332,
    bmdDeckLinkConfigAnalogAudioInputScaleChannel3               = /* 'ais3' */ 0x61697333,
    bmdDeckLinkConfigAnalogAudioInputScaleChannel4               = /* 'ais4' */ 0x61697334,
    bmdDeckLinkConfigDigitalAudioInputScale                      = /* 'dais' */ 0x64616973,
    bmdDeckLinkConfigMicrophoneInputGain                         = /* 'micg' */ 0x6D696367,

    /* Audio Output Integers */

    bmdDeckLinkConfigAudioOutputAESAnalogSwitch                  = /* 'aoaa' */ 0x616F6161,

    /* Audio Output Floats */

    bmdDeckLinkConfigAnalogAudioOutputScaleChannel1              = /* 'aos1' */ 0x616F7331,
    bmdDeckLinkConfigAnalogAudioOutputScaleChannel2              = /* 'aos2' */ 0x616F7332,
    bmdDeckLinkConfigAnalogAudioOutputScaleChannel3              = /* 'aos3' */ 0x616F7333,
    bmdDeckLinkConfigAnalogAudioOutputScaleChannel4              = /* 'aos4' */ 0x616F7334,
    bmdDeckLinkConfigDigitalAudioOutputScale                     = /* 'daos' */ 0x64616F73,
    bmdDeckLinkConfigHeadphoneVolume                             = /* 'hvol' */ 0x68766F6C,

    /* Network Flags */

    bmdDeckLinkConfigEthernetUseDHCP                             = /* 'DHCP' */ 0x44484350,
    bmdDeckLinkConfigEthernetPTPFollowerOnly                     = /* 'PTPf' */ 0x50545066,
    bmdDeckLinkConfigEthernetPTPUseUDPEncapsulation              = /* 'PTPU' */ 0x50545055,

    /* Network Integers */

    bmdDeckLinkConfigEthernetPTPPriority1                        = /* 'PTP1' */ 0x50545031,
    bmdDeckLinkConfigEthernetPTPPriority2                        = /* 'PTP2' */ 0x50545032,
    bmdDeckLinkConfigEthernetPTPDomain                           = /* 'PTPD' */ 0x50545044,

    /* Network Strings */

    bmdDeckLinkConfigEthernetStaticLocalIPAddress                = /* 'nsip' */ 0x6E736970,
    bmdDeckLinkConfigEthernetStaticSubnetMask                    = /* 'nssm' */ 0x6E73736D,
    bmdDeckLinkConfigEthernetStaticGatewayIPAddress              = /* 'nsgw' */ 0x6E736777,
    bmdDeckLinkConfigEthernetStaticPrimaryDNS                    = /* 'nspd' */ 0x6E737064,
    bmdDeckLinkConfigEthernetStaticSecondaryDNS                  = /* 'nssd' */ 0x6E737364,
    bmdDeckLinkConfigEthernetVideoOutputAddress                  = /* 'noav' */ 0x6E6F6176,
    bmdDeckLinkConfigEthernetAudioOutputAddress                  = /* 'noaa' */ 0x6E6F6161,
    bmdDeckLinkConfigEthernetAncillaryOutputAddress              = /* 'noaA' */ 0x6E6F6141,
    bmdDeckLinkConfigEthernetAudioOutputChannelOrder             = /* 'caco' */ 0x6361636F,

    /* Device Information Strings */

    bmdDeckLinkConfigDeviceInformationLabel                      = /* 'dila' */ 0x64696C61,
    bmdDeckLinkConfigDeviceInformationSerialNumber               = /* 'disn' */ 0x6469736E,
    bmdDeckLinkConfigDeviceInformationCompany                    = /* 'dico' */ 0x6469636F,
    bmdDeckLinkConfigDeviceInformationPhone                      = /* 'diph' */ 0x64697068,
    bmdDeckLinkConfigDeviceInformationEmail                      = /* 'diem' */ 0x6469656D,
    bmdDeckLinkConfigDeviceInformationDate                       = /* 'dida' */ 0x64696461,

    /* Deck Control Integers */

    bmdDeckLinkConfigDeckControlConnection                       = /* 'dcco' */ 0x6463636F
};

/* Enum BMDDeckLinkEncoderConfigurationID - DeckLink Encoder Configuration ID */

typedef uint32_t BMDDeckLinkEncoderConfigurationID;
enum _BMDDeckLinkEncoderConfigurationID {

    /* Video Encoder Integers */

    bmdDeckLinkEncoderConfigPreferredBitDepth                    = /* 'epbr' */ 0x65706272,
    bmdDeckLinkEncoderConfigFrameCodingMode                      = /* 'efcm' */ 0x6566636D,

    /* HEVC/H.265 Encoder Integers */

    bmdDeckLinkEncoderConfigH265TargetBitrate                    = /* 'htbr' */ 0x68746272,

    /* DNxHR/DNxHD Compression ID */

    bmdDeckLinkEncoderConfigDNxHRCompressionID                   = /* 'dcid' */ 0x64636964,

    /* DNxHR/DNxHD Level */

    bmdDeckLinkEncoderConfigDNxHRLevel                           = /* 'dlev' */ 0x646C6576,

    /* Encoded Sample Decriptions */

    bmdDeckLinkEncoderConfigMPEG4SampleDescription               = /* 'stsE' */ 0x73747345,	// Full MPEG4 sample description (aka SampleEntry of an 'stsd' atom-box). Useful for MediaFoundation, QuickTime, MKV and more
    bmdDeckLinkEncoderConfigMPEG4CodecSpecificDesc               = /* 'esds' */ 0x65736473	// Sample description extensions only (atom stream, each with size and fourCC header). Useful for AVFoundation, VideoToolbox, MKV and more
};

#if defined(__cplusplus)

// Forward Declarations

class IDeckLinkConfiguration;
class IDeckLinkEncoderConfiguration;

/* Interface IDeckLinkConfiguration - DeckLink Configuration interface */

class BMD_PUBLIC IDeckLinkConfiguration : public IUnknown
{
public:
    virtual HRESULT SetFlag (/* in */ BMDDeckLinkConfigurationID cfgID, /* in */ bool value) = 0;
    virtual HRESULT GetFlag (/* in */ BMDDeckLinkConfigurationID cfgID, /* out */ bool* value) = 0;
    virtual HRESULT SetInt (/* in */ BMDDeckLinkConfigurationID cfgID, /* in */ int64_t value) = 0;
    virtual HRESULT GetInt (/* in */ BMDDeckLinkConfigurationID cfgID, /* out */ int64_t* value) = 0;
    virtual HRESULT SetFloat (/* in */ BMDDeckLinkConfigurationID cfgID, /* in */ double value) = 0;
    virtual HRESULT GetFloat (/* in */ BMDDeckLinkConfigurationID cfgID, /* out */ double* value) = 0;
    virtual HRESULT SetString (/* in */ BMDDeckLinkConfigurationID cfgID, /* in */ CFStringRef value) = 0;
    virtual HRESULT GetString (/* in */ BMDDeckLinkConfigurationID cfgID, /* out */ CFStringRef* value) = 0;
    virtual HRESULT WriteConfigurationToPreferences (void) = 0;

protected:
    virtual ~IDeckLinkConfiguration () {} // call Release method to drop reference count
};

/* Interface IDeckLinkEncoderConfiguration - DeckLink Encoder Configuration interface. Obtained from IDeckLinkEncoderInput */

class BMD_PUBLIC IDeckLinkEncoderConfiguration : public IUnknown
{
public:
    virtual HRESULT SetFlag (/* in */ BMDDeckLinkEncoderConfigurationID cfgID, /* in */ bool value) = 0;
    virtual HRESULT GetFlag (/* in */ BMDDeckLinkEncoderConfigurationID cfgID, /* out */ bool* value) = 0;
    virtual HRESULT SetInt (/* in */ BMDDeckLinkEncoderConfigurationID cfgID, /* in */ int64_t value) = 0;
    virtual HRESULT GetInt (/* in */ BMDDeckLinkEncoderConfigurationID cfgID, /* out */ int64_t* value) = 0;
    virtual HRESULT SetFloat (/* in */ BMDDeckLinkEncoderConfigurationID cfgID, /* in */ double value) = 0;
    virtual HRESULT GetFloat (/* in */ BMDDeckLinkEncoderConfigurationID cfgID, /* out */ double* value) = 0;
    virtual HRESULT SetString (/* in */ BMDDeckLinkEncoderConfigurationID cfgID, /* in */ CFStringRef value) = 0;
    virtual HRESULT GetString (/* in */ BMDDeckLinkEncoderConfigurationID cfgID, /* out */ CFStringRef* value) = 0;
    virtual HRESULT GetBytes (/* in */ BMDDeckLinkEncoderConfigurationID cfgID, /* out */ void* buffer /* optional */, /* in, out */ uint32_t* bufferSize) = 0;

protected:
    virtual ~IDeckLinkEncoderConfiguration () {} // call Release method to drop reference count
};

/* Functions */

extern "C" {


}



#endif /* defined(__cplusplus) */
#endif /* defined(BMD_DECKLINKAPICONFIGURATION_H) */
