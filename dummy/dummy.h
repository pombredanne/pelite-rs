#pragma once

// The following ifdef block is the standard way of creating macros which make exporting 
// from a DLL simpler. All files within this DLL are compiled with the DUMMY_EXPORTS
// symbol defined on the command line. This symbol should not be defined on any project
// that uses this DLL. This way any other project whose source files include this file see 
// DUMMY_API functions as being imported from a DLL, whereas this DLL sees symbols
// defined with this macro as being exported.
#ifdef DUMMY_EXPORTS
// Using .DEF file
#define DUMMY_API /*__declspec(dllexport)*/
#else
#define DUMMY_API __declspec(dllimport)
#endif

// This class is exported from the dummy.dll
class DUMMY_API CDummy {
public:
	CDummy(void);
	// TODO: add your methods here.
};

extern DUMMY_API int nDummy;

DUMMY_API int fnDummy(void);
