#pragma once

struct Instruction
{
	Instruction(bool value, bool right, int state)
		: Value(value), MoveRight(right), NextState(state)
	{}

	bool Value;
	bool MoveRight;
	int NextState;
};

const int TotalSteps = 12425180;

Instruction States[6][2] =
{
	{ { true,  true,  1}, { false, true,  5 } },
	{ { false, false, 1}, { true,  false, 2 } },
	{ { true,  false, 3}, { false, true,  2 } },
	{ { true,  false, 4}, { true,  true,  0 } },
	{ { true,  false, 5}, { false, false, 3 } },
	{ { true,  true,  0}, { false, false, 4 } },
};
