#include <algorithm>
#include <print>
#include <vector>

class Solution {
public:
    long long maximumTripletValue(std::vector<int>& nums) {
        int64_t max = 0;
        for (size_t i = 0; i < nums.size() - 2; ++i) {
            for (size_t j = i + 1; j < nums.size() - 1; ++j) {
                for (size_t k = j + 1; k < nums.size(); ++k) {
                    auto r = (int64_t(nums[i]) - int64_t(nums[j])) * int64_t(nums[k]);
                    max = std::max(r, max);
                }
            }
        }
        return max;
    }
};

int main() {
    Solution s;

    std::vector<int> e1 { 12, 6, 1, 2, 7 };
    std::println("Result = {}", s.maximumTripletValue(e1));

    std::vector<int> e2 { 1, 10, 3, 4, 19 };
    std::println("Result = {}", s.maximumTripletValue(e2));

    std::vector<int> e3 { 1, 2, 3 };
    std::println("Result = {}", s.maximumTripletValue(e3));

    std::vector<int> e4 { 10, 13, 6, 2 };
    std::println("Result = {}", s.maximumTripletValue(e4));
}
