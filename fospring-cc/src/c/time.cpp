#include <chrono>
#include <stdint.h>
extern "C"
long long current_time_millis_func() {
    std::chrono::steady_clock::duration d = std::chrono::steady_clock::now().time_since_epoch();
    std::chrono::microseconds mic = std::chrono::duration_cast<std::chrono::microseconds>(d);
    return mic.count();
}