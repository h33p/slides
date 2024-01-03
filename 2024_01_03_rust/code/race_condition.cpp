#include <iostream>
#include <thread>

int sharedVariable = 0;

void increment() {
    for (int i = 0; i < 10000; ++i) {
        ++sharedVariable;
    }
}

int main() {
    std::thread thread1(increment);
    std::thread thread2(increment);

    thread1.join();
    thread2.join();

    std::cout << "Expected: 20000, Actual: " << sharedVariable << std::endl;

    return 0;
}
