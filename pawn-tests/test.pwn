#define RUN_TESTS

#include <a_samp>
#include <YSI\y_testing>

#include "../include/include.inc"

main() { }

Test:RunTest() {
	ASSERT(YourNativeFunctionName() == 1);
}