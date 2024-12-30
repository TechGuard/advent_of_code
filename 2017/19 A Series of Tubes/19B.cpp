#include <iostream>
#include <sstream>
#include <vector>


void main()
{
	std::vector<std::string> input;

	std::string line;
	while (std::getline(std::cin, line)) input.push_back(line);

	int x = 0;
	int y = 0;

	for (char c : input[0])
	{
		if (c == '|') break;
		++x;
	}

	int dx = 0;
	int dy = 1;
	int steps = 0;

	while (true)
	{
		int ly = y;
		int lx = x;
		x += dx; y += dy;
		char c = input[y][x];

		++steps;

		if (c == ' ')
			break;

		if (c == '|' || c == '-')
			continue;

		if (c == '+')
		{
			dx = 0; dy = 0;
			if (y + 1 != ly && input[y + 1][x] != ' ') dy = 1;
			else if (y - 1 != ly && input[y - 1][x] != ' ') dy = -1;
			else if (x + 1 != lx && input[y][x + 1] != ' ') dx = 1;
			else if (x + 1 != lx && input[y][x - 1] != ' ') dx = -1;
		}
	}

	std::cout << steps << std::endl;
}