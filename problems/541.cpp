#include <algorithm>
#include <print>

class Solution {
public:
    std::string reverseStr(std::string s, int k) {
        for (size_t i = 0; i < s.size(); i += 2 * k) {
            size_t remaining = s.size() - i;
            size_t window = std::min(size_t(k), remaining);
            std::reverse(&s[i], &s[i] + window);
        }

        return s;
    }
};

int main() {
    Solution s;
    std::println("Result = {}", s.reverseStr("abcdefg", 2));
}
