// NOTE:change the name of include from pawnlib.inc to {{crate_name}}.inc
// remove built-in include guard
#if defined _inc_{{crate_name}}
	#undef _inc_{{crate_name}}
#endif
// custom include-guard
#if defined _{{crate_name}}_included
	#endinput
#endif
#define _{{crate_name}}_included

native Foo();