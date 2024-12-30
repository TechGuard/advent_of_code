#include <iostream>
#include <sstream>
#include <map>


namespace
{
	using i64 = std::int64_t;

	struct Coordinate
	{
		i64 x = 0;
		i64 y = 0;

		bool operator<(const Coordinate& other) const
		{
			if (x != other.x)
				return  x < other.x;
			return  y < other.y;
		}
	};

	std::map<Coordinate, bool> map;
}

void _()
{
	i64 y = 0;
	i64 x = 0;

	std::string line;
	while (std::getline(std::cin, line))
	{
		int center = line.length() / 2;
		x = 0;
		for (char c : line)
			map[{x++ - center, y - center}] = c == '#';
		++y;
	}


	x = 0;
	y = 0;

	i64 nx = 0;
	i64 ny = -1;

	int solution = 0;

	for (int burst = 0; burst < 10000; ++burst)
	{
		if (map[{x, y}])
		{
			map[{x, y}] = false;

			i64 tmp = nx;
			nx = -ny;
			ny = tmp;
		}
		else
		{
			map[{x, y}] = true;
			++solution;

			i64 tmp = nx;
			nx = ny;
			ny = -tmp;
		}

		x += nx;
		y += ny;
	}

	std::cout << solution << std::endl;
}