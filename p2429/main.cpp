#include <bit>
#include <print>
#include <string.h>

class Solution {
public:
    int minimizeXor(int num1, int num2) {
        size_t v = 0;

        size_t stream = num1;
        for (size_t n = 0; n < std::popcount(size_t(num2)); ++n) {
            if (stream != 0) {
                size_t lz = std::countl_zero(stream);
                size_t nth = sizeof(stream) * CHAR_BIT - lz - 1;
                v |= 1 << nth;
                stream &= ~(1 << nth);
            } else {
                v |= 1 << std::countr_one(v);
            }
        }

        return v;
    }
};

int main() {
    Solution s;
    std::println("Result = {}", s.minimizeXor(1, 12));
    std::println("Result = {}", s.minimizeXor(3, 5));
    std::println("Result = {}", s.minimizeXor(25, 72));
    std::println("Result = {}", s.minimizeXor(8, 100));
}
