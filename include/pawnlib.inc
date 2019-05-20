// remove built-in include guard
#if defined _inc_pawnlib
	#undef _inc_pawnlib
#endif
// custom include-guard
#if defined _pawnlib_included
	#endinput
#endif
#define _pawnlib_included

native Foo();