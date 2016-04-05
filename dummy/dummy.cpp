// dummy.cpp : Defines the exported functions for the DLL application.
//

#include "stdafx.h"
#include "dummy.h"


// This is an example of an exported variable
DUMMY_API int nDummy=0;

// This is an example of an exported function.
DUMMY_API int fnDummy(void)
{
    return 42;
}

// This is the constructor of a class that has been exported.
// see dummy.h for the class definition
CDummy::CDummy()
{
    return;
}
