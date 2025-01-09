#include <iostream>
#include <vector>
#include <string>
#include <sstream>
#include <algorithm>

int main() {
    int n;
    std::cin >> n;
    std::cin.ignore();

    std::vector<std::vector<int>> matrix(n, std::vector<int>(n));
    for (int i = 0; i < n; i++) {
        std::string line;
        std::getline(std::cin, line);
        std::istringstream iss(line);
        std::string token;
        std::vector<int> row;
        while (std::getline(iss, token, ' ')) {
            row.push_back(std::stoi(token));
        }
        matrix[i] = row;
    }

    auto rotated = matrix;
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            rotated[i][j] = matrix[j][i];
        }
    }

    for (auto i: rotated) {
        std::reverse(i.begin(), i.end());
    }

    std::cout << std::endl;

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            std::cout << rotated[i][j] << " ";
        }
        std::cout << std::endl;
    }

    return 0;
}
