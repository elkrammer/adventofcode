#include <iostream>
#include <fstream>
#include <string>
#include <vector>

int main(int argc, char **argv) {
    std::ifstream file("input.txt");
    std::string line;
    std::vector<std::string> directions;

    while (std::getline(file, line, ',')) {
        directions.push_back(line);
    }

    int x = 0;
    int y = 0;

    std::cout << directions.size() << std::endl;

    for (int i = 0; i < directions.size(); i++) {

        //std::cout << "x: " << x << std::endl;
        //std::cout << "y: " << y << std::endl;
        if (directions[i][0] == 'L')
            x -= 1;
        if (directions[i][0] == 'R')
            x += 1;
        if (directions[i][0] == 'D')
            y -= 1;
        if (directions[i][0] == 'U')
            y += 1;
    }

    std::cout << "x: " << x << std::endl;
    std::cout << "y: " << y << std::endl;

    return 0;
}
