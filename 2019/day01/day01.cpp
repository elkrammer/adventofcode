#include <iostream>
#include <fstream>
#include <string>
#include <cmath>

int calculateFuel(int mass) {
    return (std::floor(mass) / 3 - 2);
}

int main(int argc, char **argv) {
    std::ifstream input_file("input.txt");
    std::string mass;
    int part1 = 0;
    int part2 = 0;

    while (std::getline(input_file, mass)) {
        double moduleFuel = calculateFuel(stod(mass));
        double fuelSum = 0;
        part1 += moduleFuel;

        while (moduleFuel >= 0) {
            fuelSum += moduleFuel;
            moduleFuel = calculateFuel(moduleFuel);
        }

        part2 += fuelSum;
    }

    std::cout << "Part One: " << part1 << "\n";
    std::cout << "Part Two: " << part2 << "\n";

    return 0;
}
