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

	enum State
	{
		Clean,
		Weakened,
		Infected,
		Flagged,
	};

	std::map<Coordinate, State> map;
}

void main()
{
	i64 y = 0;
	i64 x = 0;

	std::string line;
	while (std::getline(std::cin, line))
	{
		int center = line.length() / 2;
		x = 0;
		for (char c : line)
			map[{x++ - center, y - center}] = (c == '#') ? Infected : Clean;
		++y;
	}


	x = 0;
	y = 0;

	i64 nx = 0;
	i64 ny = -1;

	int solution = 0;

	for (int burst = 0; burst < 10'000'000; ++burst)
	{
		auto state = map[{x, y}];
		switch (state)
		{
			case Clean:
			{
				map[{x, y}] = Weakened;
				i64 tmp = nx;
				nx = ny;
				ny = -tmp;
				break;
			}
			case Weakened:
			{
				map[{x, y}] = Infected;
				++solution;
				break;
			}
			case Infected:
			{
				map[{x, y}] = Flagged;
				i64 tmp = nx;
				nx = -ny;
				ny = tmp;
				break;
			}
			case Flagged:
			{
				map[{x, y}] = Clean;
				nx = -nx;
				ny = -ny;
				break;
			}
		}

		x += nx;
		y += ny;
	}

	std::cout << solution << std::endl;
}