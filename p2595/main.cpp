#include <bit>
#include <print>
#include <vector>

constexpr uint32_t explode_byte(uint8_t byte) {
    return (uint32_t(byte) << 0) | (uint32_t(byte) << 8) | (uint32_t(byte) << 16) | (uint32_t(byte) << 24);
}

class Solution {
public:
    std::vector<int> evenOddBit(int n) {
        return { std::popcount(uint32_t(n & explode_byte(0b01010101))), std::popcount(uint32_t(n & explode_byte(0b10101010))) };
    }
};

int main() {
    Solution s;
    std::println("Result = {}", s.evenOddBit(50));
    std::println("Result = {}", s.evenOddBit(2));
    std::println("Result = {}", s.evenOddBit(256));
    std::println("Result = {}", s.evenOddBit(INT32_MAX));
}
