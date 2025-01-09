#include <iostream>

int main() {
    double price, discount, paid;
    std::cout << "Input:" << std::endl;
    std::cout << "Price: ";
    std::cin >> price;
    std::cout << "Discount: ";
    std::cin >> discount;
    std::cout << "Paid: ";
    std::cin >> paid;
    std::cout << std::endl;
    std::cout << "Output:" << std::endl;
    std::cout << "Price: " << price << std::endl;
    std::cout << "Discount: " << discount << std::endl;
    std::cout << "Discounted: " << price * (discount / 100.0) << std::endl;
    std::cout << "Net Price: " << price - price * (discount / 100.0) << std::endl;
    std::cout << "Paid: " << paid << std::endl;
    std::cout << "Change: " << paid - price + price * (discount / 100.0) << std::endl;
}