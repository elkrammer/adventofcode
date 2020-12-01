#include <iostream>
#include <fstream>
#include <vector>
#include <string>

unsigned int run_code(unsigned int x, unsigned int y, std::vector<unsigned int>& intcode) {
    // set first and second elements to puzzle requirements
    intcode[1] = x;
    intcode[2] = y;

    for (int i = 0; i < intcode.size(); i += 4) {
        int op = intcode[i];
        int pos1 = intcode[i + 1];
        int pos2 = intcode[i + 2];
        int res = intcode[i + 3];

        if (op == 1)
            intcode[res] = intcode[pos1] + intcode[pos2];
        else if (op == 2)
            intcode[res] = intcode[pos1] * intcode[pos2];
        else
            break;
    }
    return intcode[0];
}

int brute_force(std::vector<unsigned int>& intcode) {
    const int target = 19690720;
    for (int x = 0; x < 100; x++) {
        for (int y = 0; y < 100; y++) {
            std::vector<unsigned int> code = intcode;
            int result = run_code(x, y, code);
            if (result == target) {
                return 100 * x + y;
            }
        }
    }
    throw std::runtime_error("target not found");
}

int main(int argc, char **argv) {
    std::ifstream file("input.txt");
    std::string line;
    std::vector<unsigned int> inputcode;

    while (std::getline(file, line, ',')) {
        inputcode.push_back(std::stoi(line));
    }

    file.close();

    std::vector<unsigned int> part1 = inputcode;
    std::vector<unsigned int> part2 = inputcode;

    std::cout << "Part 1 = " << run_code(12, 2, part1) << std::endl;
    std::cout << "Part 2 = " << brute_force(part2) << std::endl;

    return 0;
}
