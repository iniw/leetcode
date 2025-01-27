#include <print>

class Solution {
public:
    std::string longestPrefix(std::string s) {
        for (size_t i = s.size() - 1; i-- > 0;) {
            std::string_view prefix { &s[0], i + 1 };
            std::string_view suffix { &s[s.size() - i - 1], i + 1 };

            if (prefix == suffix)
                return std::string(prefix);
        }

        return "";
    }
};

int main() {
    Solution s;
    std::println("Result = {}", s.longestPrefix("ababab"));
}
