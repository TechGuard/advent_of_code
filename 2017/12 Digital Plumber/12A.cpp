#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <unordered_map>


namespace
{
	std::unordered_map<int, struct Program> programs;

	struct Program
	{
		int index;
		std::vector<int> connections;

		void get_pipes(std::vector<int>& pipes)
		{
			for (int idx : connections)
			{
				auto itr = std::find(pipes.begin(), pipes.end(), idx);
				if (itr == pipes.end())
				{
					pipes.push_back(idx);
					programs[idx].get_pipes(pipes);
				}
			}
		}
	};
}

void _()
{
	// read programs
	std::string input;
	while (std::getline(std::cin, input))
	{
		std::stringstream cin(input);

		Program program{};
		cin >> program.index;

		char skip[5];
		cin >> skip;

		int index;
		while (cin >> index)
		{
			program.connections.push_back(index);

			char c;
			cin >> c;
		}

		programs.emplace(program.index, program);
	}
	
	std::vector<int> pipes;
	programs[0].get_pipes(pipes);

	std::cout << pipes.size() << std::endl;
}