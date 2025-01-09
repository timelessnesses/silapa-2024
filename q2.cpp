#include <iostream>
#include <map>
#include <vector>
#include <string>
#include <algorithm>

int main() {
    std::map<int, int> analytics;
    std::cout << "Input:" << std::endl;
    int i;
    while (true) {
        std::cin >> i;
        if (i == -100) {
            break;
        }
        if (i < 0 || i > 9) {
            return 0;
        }
        analytics[i]++;
    }
    std::cout << std::endl;

    std::vector<std::pair<int, int>> sorted_analytics(analytics.begin(), analytics.end());

    std::sort(sorted_analytics.begin(), sorted_analytics.end(), [](const auto& a, const auto& b) {
        return a.second > b.second;
    });

    std::cout << "Character Analysis:" << std::endl;
    for (const auto& i : sorted_analytics) {
        std::string repeated(i.second, '*');
        std::cout << i.first << ": " << repeated << " (" << i.second << ")" << std::endl;
    }
}
