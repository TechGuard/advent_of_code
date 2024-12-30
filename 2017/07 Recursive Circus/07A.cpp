#include <iostream>
#include <sstream>
#include <string>
#include <vector>


void _()
{
	std::vector<std::string> programs;
	std::vector<std::string> connections;

	std::string input;
	while (std::getline(std::cin, input))
	{
		std::stringstream cin(input);
		std::string name;
		cin >> name;

		programs.push_back(name);

 		size_t offset = input.find_first_of("->");
 		if (offset != std::string::npos)
			connections.push_back(input.c_str() + offset + 2);
	}

	for (auto input : connections)
	{
		std::stringstream f(input);
		std::string disc;

		while (std::getline(f, disc, ','))
		{
			auto itr = std::find(programs.begin(), programs.end(), disc.c_str() + 1);
			if (itr != programs.end())
				programs.erase(itr);
		}
	}

	std::cout << programs[0] << std::endl;
}