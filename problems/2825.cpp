#include <print>

class Solution {
public:
    bool canMakeSubsequence(std::string str1, std::string str2) {
        size_t start = 0;
        for (auto c2 : str2) {
            bool matched = false;

            for (size_t i = start; i < str1.size(); ++i) {
                char c1 = str1[i];

                auto alpha = [](char c) {
                    return c - 'a';
                };

                if (c1 == c2 || ((alpha(c1) + 1) % (alpha('z') + 1)) == alpha(c2)) {
                    start = i + 1;
                    matched = true;
                    break;
                }
            }

            if (!matched)
                return false;
        }

        return true;
    }
};

int main() {
    Solution s;
    std::println("Result = {}", s.canMakeSubsequence("abc", "ad"));
    std::println("Result = {}", s.canMakeSubsequence("zc", "ad"));
    std::println("Result = {}", s.canMakeSubsequence("ab", "d"));
}
