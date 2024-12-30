#include <iostream>
#include <map>
#include "input.h"


void main()
{
	std::map<int, bool> tape;
	
	int pos = 0;
	int state = 0;

	for (int i = 0; i < TotalSteps; ++i)
	{
		const auto& instruction = States[state][tape[pos]];
		
		tape[pos] = instruction.Value;
		
		if (instruction.MoveRight) ++pos; else --pos;

		state = instruction.NextState;
	}

	int solution = 0;
	for (const auto& val : tape)
		if (val.second)
			++solution;

	std::cout << solution << std::endl;
}