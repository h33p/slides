#include <iostream>

extern void example_norestrict(int& a, int& b, int c) {
	a += c;
	b *= c;
	a += c;
}

void example(int& __restrict__ a, int& __restrict__ b, int c) {
	a += c;
	b *= c;
	a += c;
}

int main() {
    int a = 5;

    example(a, a, 100);

    std::cout << a << std::endl;
}
