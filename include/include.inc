// NOTE:change the name of include to project name
// remove built-in include guard
#if defined _inc_INCLUDE
	#undef _inc_INCLUDE
#endif
// custom include-guard
#if defined _INCLUDE_included
	#endinput
#endif
#define _INCLUDE_included

native YourNativeFunctionName();