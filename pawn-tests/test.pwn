#define RUN_TESTS

#include <a_samp>
#include <YSI\y_testing>

#include "../include/MyInclude.inc"

main() { }

Test:RunTest() {
	ASSERT(YourNativeFunctionName() == 1);
}