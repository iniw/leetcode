#include <bit>
#include <print>
#include <vector>

class Solution {
public:
    std::string convertToBase7(int num) {
        int result = 0;

        int count = 0;
        while (num != 0) {
            auto [quot, rem] = std::div(num, 7);
            result += rem * std::pow(10, count++);
            num = quot;
        }

        return std::to_string(result);
    }
};

int main() {
    Solution s;
    std::println("Result = {}", s.convertToBase7(100));
    std::println("Result = {}", s.convertToBase7(7));
    std::println("Result = {}", s.convertToBase7(-7));
    std::println("Result = {}", s.convertToBase7(10));
}
