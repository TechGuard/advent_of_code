#include <iostream>
#include <sstream>
#include <string>
#include <algorithm>


void main()
{
	std::string input;
	std::getline(std::cin, input);

	std::string direction;
	std::stringstream cin(input);

	int x = 0;
	int y = 0;
	int max_steps = 0;

	while (std::getline(cin, direction, ','))
	{
		if (direction == "n") y += 2;
		if (direction == "s") y -= 2;
		if (direction == "nw") { ++y; --x; }
		if (direction == "sw") { --y; --x; }
		if (direction == "ne") { ++y; ++x; }
		if (direction == "se") { --y; ++x; }

		int x_abs = std::abs(x);
		int y_abs = std::abs(y);
		int steps = 0;

		// move diagonal
		if (x_abs > 0 && y_abs > 0)
		{
			int offset = std::min(x_abs, y_abs);
			x_abs -= offset;
			y_abs -= offset;
			steps += offset;
		}

		// move horizontal
		if (x_abs > 0)
			steps += x_abs;

		// move vertical
		if (y_abs > 0)
			steps += y_abs / 2;

		if (steps > max_steps)
			max_steps = steps;
	}

	std::cout << max_steps << std::endl;
}