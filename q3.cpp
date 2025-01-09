#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

std::string rtrim(std::string s);

int main() {
    int n;
    std::cin >> n;
    std::string s;
    for (int i = 1; i <= n; i++) {
        s += std::to_string(i);
    }
    std::vector<std::string> strings;
    while (!rtrim(s).empty()) {
        strings.push_back(s);
        s = rtrim(s);
        s.pop_back();
        s.append(n - s.size(), ' ');
    }
    std::reverse(strings.begin(), strings.end());
    for (const auto& i : strings) {
        std::string reversed(i.rbegin(), i.rend());
        std::cout << i << reversed << std::endl;
    }
}
inline std::string rtrim(std::string s) {
    s.erase(std::find_if(s.rbegin(), s.rend(), [](unsigned char ch) {
        return !std::isspace(ch);
    }).base(), s.end());
    return s;
}