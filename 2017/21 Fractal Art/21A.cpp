#include <iostream>
#include <sstream>
#include <vector>


namespace
{
	struct Grid;
	using GridPair = std::pair<Grid, Grid>;
	std::vector<GridPair> input;

	struct Grid
	{
		Grid() : size(0), data("")
		{}

		Grid(int size) : size(size), data("")
		{}

		Grid(int size, std::string data) : size(size), data(data)
		{}

		std::string data;
		int size;

		bool operator<(const Grid& other) const
		{
			return other.size < size;
		}

		const GridPair findRule() const
		{
			for (const auto& rule : input)
			{
				if (rule.first.size != size)
					continue;

				std::string rdata = rule.first.data;
				if (rdata == data)
					return rule;

				std::string ndata;

				for (int rotation = 0; rotation < 3; ++rotation)
				{
					ndata.resize(size * size, '@');

					for (int y = 0; y < size; ++y) for (int x = 0; x < size; ++x)
					{
						ndata[y * size + x] = rdata[x * size + y];
					}

					if (ndata == data)
						return rule;

					rdata = ndata;
					ndata.resize(size * size, '@');
					for (int y = 0; y < size; ++y) for (int x = 0; x < size; ++x)
					{
						ndata[y * size + x] = rdata[y * size + (size - x - 1)];
					}

					if (ndata == data)
						return rule;

					rdata = ndata;
					ndata.resize(size * size, '@');
					for (int y = 0; y < size; ++y) for (int x = 0; x < size; ++x)
					{
						ndata[y * size + x] = rdata[(size - y - 1) * size + x];
					}
				}
			}
			return {};
		}

		void print() const
		{
			for (int y = 0; y < size; ++y)
				std::cout << data.substr(y * size, size) << std::endl;
		}
	};
}

void execute(int iterations)
{
	// Parse input
	{
		std::string line;
		while (std::getline(std::cin, line))
		{
			std::stringstream cin(line);

			Grid grid1;

			char c;
			while (cin >> c)
			{
				if (c == '/')
				{
					++grid1.size;
					continue;
				}
				if (c == '=')
				{
					++grid1.size;
					cin.ignore(2);
					break;
				}
				grid1.data += c;
			}

			Grid grid2;

			while (cin >> c)
			{
				if (c == '/')
				{
					++grid2.size;
					continue;
				}
				grid2.data += c;
			}

			++grid2.size;
			input.push_back({ grid1, grid2 });
		}
	}

	Grid image(3, ".#...####");

	// Process
	for (int iteration = 0; iteration < iterations; ++iteration)
	{
		int size = image.size % 2 == 0 ? 2 : 3;
		int divisions = image.size / size;
		
		std::vector<Grid> sections;
		sections.resize(divisions * divisions);

		for (int y = 0; y < image.size; ++y)
		{
			int ysection = y / size;
			int yoff = y % size;
			for (int x = 0; x < divisions; ++x)
			{
				sections[ysection * divisions + x].data += image.data.substr(y * image.size + x * size, size);
				sections[ysection * divisions + x].size = size;
			}
		}

		for (auto& section : sections)
		{
			GridPair rule = section.findRule();
			section.data = rule.second.data;
			section.size = rule.second.size;
		}

		++size;
		image.size = size * divisions;
		image.data = "";

		for (int y = 0; y < image.size; ++y)
		{
			int ysection = y / size;
			int yoff = y % size;
			for (int x = 0; x < divisions; ++x)
				image.data += sections[ysection * divisions + x].data.substr(yoff * size, size);
		}
	}

	int solution = 0;

	for (const char c : image.data)
		if (c == '#')
			++solution;

	std::cout << solution << std::endl;
}

void _()
{
	execute(5);
}