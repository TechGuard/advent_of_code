EXAMPLE_INPUT = '''\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
'''

def parse_input(input):
    inputWorkflows, inputRatings = input.split('\n\n')

    workflows = {}
    for inputWorkflow in inputWorkflows.splitlines():
        name, inputRules = inputWorkflow.split('{')
        rules = []
        for rule in inputRules[:-1].split(','):
            if ':' in rule:
                cmp, result = rule.split(':')
                rules.append([cmp[0], cmp[1], int(cmp[2:]), result])
            else:
                rules.append([rule])
        workflows[name] = rules

    ratings = []
    for inputRating in inputRatings.splitlines():
        rating = {}
        for part in inputRating[1:-1].split(','):
            name, value = part.split('=')
            rating[name] = int(value)
        ratings.append(rating)

    return workflows, ratings


def part_1(input):
    workflows, ratings = parse_input(input)
    result = 0
    for rating in ratings:
        next = 'in'
        while next not in 'AR':
            for rule in workflows[next]:
                def f(rule, rating):
                    if len(rule) == 1:
                        return rule[0]
                    else:
                        (part, cmp, value, target) = rule
                        if cmp == '<' and rating[part] < value:
                            return target
                        elif cmp == '>' and rating[part] > value:
                            return target
                next = f(rule, rating)
                if next:
                    break
        if next == 'A':
            result += sum(rating.values())
    return result


def part_2(input):
    workflows, _ = parse_input(input)

    def f(rating, name):
        if name == 'R':
            return 0
        if name == 'A':
            result = 1
            for (r0, r1) in rating.values():
                result *= r1 - r0 + 1
            return result
        
        result = 0
        for rule in workflows[name]:
            if len(rule) == 1:
                result += f(rating.copy(), rule[0])
            else:
                (part, cmp, value, target) = rule
                (r0, r1) = rating[part]
                
                if cmp == '<':
                    before = (r0, min(r1, value - 1))
                    after = (max(r0, value), r1)
                else:
                    before = (max(r0, value + 1), r1)
                    after = (r0, min(r1, value))

                nRating = rating.copy()
                nRating[rule[0]] = before
                result += f(nRating, target)
                rating[rule[0]] = after

        return result

    return f({'x':(1,4000), 'm':(1,4000), 'a':(1,4000), 's':(1,4000)}, 'in')