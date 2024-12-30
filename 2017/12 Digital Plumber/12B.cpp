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

void main()
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
	
	// find groups
	int groups = 0;
	while (programs.size() > 0)
	{
		std::vector<int> pipes;
		programs.begin()->second.get_pipes(pipes);

		for (int idx : pipes)
			programs.erase(idx);

		++groups;
	};

	std::cout << groups << std::endl;
}