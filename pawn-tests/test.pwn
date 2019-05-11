#define RUN_TESTS

#include <a_samp>
#include <YSI_Core\y_testing>

#include "../include/{{crate_name}}.inc"

main() { }

Test:RunTest() {
	ASSERT(Foo() == 1);
}
