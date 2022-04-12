#define CATCH_CONFIG_MAIN
#include "catch2.h"

using namespace std;

// Change me or remove me to fit your application
TEST_CASE("Test case 1 description goes here", "[tag1]") {
	int x = 10;
	CHECK(x - 4 == 6.);

	SECTION("Section 1") {
		CHECK(!false);
		CHECK(x == 10);
	}

	SECTION("Section 2") {
		CHECK_FALSE(false);
		CHECK(x != 1);
	}
}

// Change me or remove me to fit your application
TEST_CASE("Test case 2 description goes here", "[tag2]") {
	CHECK(true);
}

// Change me or remove me to fit your application
TEST_CASE("Test case 3 description goes here", "[tag2]") {
	CHECK_FALSE(false);
}