#include <iostream>
#include <sstream>
#include <string>
#include <algorithm>


void _()
{
	std::string input;
	std::getline(std::cin, input);

	std::string direction;
	std::stringstream cin(input);

	int x = 0;
	int y = 0;

	while (std::getline(cin, direction, ','))
	{
		if (direction == "n") y += 2;
		if (direction == "s") y -= 2;
		if (direction == "nw") { ++y; --x; }
		if (direction == "sw") { --y; --x; }
		if (direction == "ne") { ++y; ++x; }
		if (direction == "se") { --y; ++x; }
	}

	// absolute position
	x = std::abs(x);
	y = std::abs(y);
	int steps = 0;

	// move diagonal
	if (x > 0 && y > 0)
	{
		int offset = std::min(x, y);
		x -= offset;
		y -= offset;
		steps += offset;
	}

	// move horizontal
	if (x > 0)
		steps += x;

	// move vertical
	if (y > 0)
		steps += y / 2;

	std::cout << steps << std::endl;
}