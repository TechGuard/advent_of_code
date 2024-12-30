#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <map>
#include <algorithm>


namespace
{
	std::map<std::string, struct Program> programs;

	struct Program
	{
		Program() {}
		Program(std::string name, int weight) : name(name), weight(weight) {}

		std::string name;
		int weight;
		Program* parent = nullptr;
		std::vector<Program*> discs;

		std::string disc_connections;

		// Parse connections and find references
		void connectDiscs()
		{
			if (disc_connections.empty())
				return;

			std::stringstream f(disc_connections);
			std::string disc;
			bool trim = false;

			while (std::getline(f, disc, ','))
			{
				if (trim)
					disc = disc.c_str() + 1;
				trim = true;

				auto& program = programs[disc];
				program.parent = this;
				discs.push_back(&program);
			}

			disc_connections.clear();
		}

		// Calculate total weight of all children
		int totalWeight()
		{
			int total = weight;
			for (const auto& disc : discs) total += disc->totalWeight();
			return total;
		}

		// Check if all weights are balanced, search in children first.
		Program* findWrongWeights()
		{
			// Look in children first
			for (const auto& disc : discs)
			{
				if (auto* result = disc->findWrongWeights())
					return result;
			}

			if (discs.size() < 2) return nullptr;

			int weight = discs[0]->totalWeight();
			for (const auto& disc : discs)
			{
				if (disc->totalWeight() != weight)
					return this;
			}

			return nullptr;
		}
	};
}

void main()
{
	// Parse input
	std::string input;
	while (std::getline(std::cin, input))
	{
		char name[12];
		int weight;
		sscanf_s(input.c_str(), "%s (%i)", name, (unsigned int)sizeof name, &weight);

		programs[name] = { name, weight };

		size_t offset = input.find_first_of("->");
		if (offset != std::string::npos)
		{
			programs[name].disc_connections = input.c_str() + offset + 3;
		}
	}

	// Connect discs
	for (auto& kp : programs)
	{
		kp.second.connectDiscs();
	}

	// Find root
	Program* root = nullptr;
	for (auto& kp : programs)
	{
		if (kp.second.parent == nullptr)
		{
			root = &kp.second;
			break;
		}
	}

	// Find the program that is not balanced
	Program* invalid_program = root->findWrongWeights();

	// Count weights
	std::map<int, int> weights;
	for (auto* disc : invalid_program->discs) weights[disc->totalWeight()]++;

	// Find which value is wrong
	int bad_weight;
	int good_weight;
	for(const auto& kp : weights)
	{
		if (kp.second == 1)
			bad_weight = kp.first;
		else
			good_weight = kp.first;
	}

	// Find disc with wrong weight and calculate new weight
	for (auto* disc : invalid_program->discs)
	{
		if (disc->totalWeight() == bad_weight)
		{
			int min = std::min(good_weight, bad_weight);
			int max = std::max(good_weight, bad_weight);
			int diff = max - min;

			std::cout << (disc->weight - diff) << std::endl;
		}
	}
}