import os
import sys
import string
import secrets
import argparse
import importlib.util

def gensym(length=32, prefix="gensym_"):
    """
    generates a fairly unique symbol, used to make a module name,
    used as a helper function for load_module

    :return: generated symbol
    """
    alphabet = string.ascii_uppercase + string.ascii_lowercase + string.digits
    symbol = "".join([secrets.choice(alphabet) for i in range(length)])

    return prefix + symbol

def load_module(source, module_name=None):
    """
    reads file source and loads it as a module

    :param source: file to load
    :param module_name: name of module to register in sys.modules
    :return: loaded module
    """

    if module_name is None:
        module_name = gensym()

    spec = importlib.util.spec_from_file_location(module_name, source)
    module = importlib.util.module_from_spec(spec)
    sys.modules[module_name] = module
    spec.loader.exec_module(module)

    return module

parser = argparse.ArgumentParser(prog='advent_of_code')
parser.add_argument('day', type=int)
parser.add_argument('-e', '--example', help='Run with example input', action='store_true')
args = parser.parse_args()

day_filename = os.path.join('solutions', 'day{:>02}.py'.format(args.day))
if not os.path.exists(day_filename):
    print('Day {:>02} is not implemented.'.format(args.day))
    exit(1)

day_module = load_module(day_filename)

if args.example:
    if not hasattr(day_module, 'EXAMPLE_INPUT') or not isinstance(day_module.EXAMPLE_INPUT, str):
        print('Missing `EXAMPLE_INPUT` in {}'.format(day_filename))
        exit(1)
    input = day_module.EXAMPLE_INPUT
else:
    input = sys.stdin.read()

print('Answer Part One: ', end='')
if hasattr(day_module, 'part_1') and callable(day_module.part_1):
    print('{}'.format(day_module.part_1(input)))
else:
    print('Not implemented')

print('Answer Part Two: ', end='')
if hasattr(day_module, 'part_2') and callable(day_module.part_2):
    print('{}'.format(day_module.part_2(input)))
else:
    print('Not implemented')