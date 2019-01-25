// NOTE:change the name of include to {{crate_name}}
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