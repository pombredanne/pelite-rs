rem Copy the output of dummy to the tests/bin location.
rem Make sure to run me from the parent directory!

copy /Y /B  "dummy\Debug\dummy.dll"        "tests\bin\dummyd.dll"
copy /Y /B  "dummy\Release\dummy.dll"      "tests\bin\dummy.dll"
copy /Y /B  "dummy\x64\Debug\dummy.dll"    "tests\bin\dummy64d.dll"
copy /Y /B  "dummy\x64\Release\dummy.dll"  "tests\bin\dummy64.dll"
