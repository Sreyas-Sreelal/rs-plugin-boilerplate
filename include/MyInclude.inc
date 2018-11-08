// NOTE:change the name of include to project name
// remove built-in include guard
#if defined _inc_MyInclude
	#undef _inc_MyInclude
#endif
// custom include-guard
#if defined _MyInclude_included
	#endinput
#endif
#define _MyInclude_included

native YourNativeFunctionName();