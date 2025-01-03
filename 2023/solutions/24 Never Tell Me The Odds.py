EXAMPLE_INPUT = '''\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
'''

import collections

def part_1(input):
    lines = []
    for line in input.splitlines():
        p, v = line.split(' @ ')
        px, py, pz = [int(x) for x in p.split(', ')]
        vx, vy, vz = [int(x) for x in v.split(', ')]
        lines.append((px, py, pz, vx, vy, vz))
    
    area_start = 200000000000000
    area_end   = 400000000000000

    def line_intersection(line1, line2):
        xdiff = (line1[0] - (line1[3] + line1[0]), line2[0] - (line2[3] + line2[0]))
        ydiff = (line1[1] - (line1[4] + line1[1]), line2[1] - (line2[4] + line2[1]))

        def det(a, b):
            return a[0] * b[1] - a[1] * b[0]

        div = det(xdiff, ydiff)
        if div == 0:
            return None

        d = (det(line1[:3], line1[3:]), det(line2[:3], line2[3:]))
        x = det(d, xdiff) / div
        y = det(d, ydiff) / div
        return x, y

    def in_past(line, p):
        return ((p[0] - line[0]) * line[3]) + ((p[1] - line[1]) * line[4]) < 0

    result = 0
    for i in range(len(lines)):
        for j in range(i + 1, len(lines)):
            a = lines[i]
            b = lines[j]
            r = line_intersection(a, b)
            if r:
                if r[0] >= area_start and r[0] <= area_end and r[1] >= area_start and r[1] <= area_end:
                    if not in_past(a, r) and not in_past(b, r):
                        result += 1

    return result


# pls forgive me
# https://topaz.github.io/paste/#XQAAAQCTCQAAAAAAAAA0m0pnuFI8c8h14kUamL+XYzBvHppm9lCBjoan1Q0sYYlqcLWQS0njG9969tZsWjQCla5prwqFlBf7NmX9kjiOCY7TX+bWvefIHPFw0kJtw27ueQjYL0mdn7Q6FDAoLATUFRdcl0NIZ7Ws0uhB+tljhHjpGsc1roo/acxal6l3MZPN/ALVENlwSdFKVmnz6EME/+g78MDO1aQ8PLZcU94Ji3CPqsDOgne3qQDPxCGIQSM/5ne9o/rfay+iN5g5flZBPyTc9wLxTYBt39aGtOJLshNTd0/FMJq5XtqBogaMLjwJ3Sx/DlVP5j+Q/eAqKcuJUAbYzIK2yEIutwdIWvWfAq9eZB5dH6a4Y7UUqYCgB0a1oZJzJ57mJBa7sB+hAmeGOjjDjaGdsrbt0kVL+nzHVzzuHElv8kZxp4nm/oi6nLKS+gfbt5v7tbjxI0vA6KNI7RTF0c7l/U0jGZinbC/TaPPPg/oicam2RfQ2HjlJwChFB6sMiEHJsXU3bP3rd6/Of6VfztN32WlUGSRCkPfyLZS+2XykU0fjYdd0gtU986YbX3tpWTduaOf+mgaaSt17D0VKdsxXV1AvaWDj+uCviXx4yTepZKV2/IpOlewmyC4njPHkcT/g364Zh5+UkgsA4aYPzAxGEUAxVEu2tb8eGJ6iwbHSsqXUdPa5sR82uDJxBfozM8avAcAgK/K40Ud7+zDdUXFjiDe4eq1hB9Qd13wI7lhY6s5bcxB5wtUkGsCBD3n+1xdk+ChVsAgBiasGl368HsiO8fvSO7g3B2VAiay5a9d5xNWjW2aDPkGzO9jJrnfYjluL162BwQX4q6xaaeR0NH4dNw5okgJXUJJGJhhbxPQ0jVbhtV4lRY6ewjZljRK4b+Un6yRmy6g21+8KtwbLoxwDDjzgmpmSypOP4ByIOsZJugqXnTl5g0wb1pAYWTzLAGRX7ReND+BE5uIHOCmBI/nxdEN5pH3TJHhBglmYOuSPmMPY2dKLbLz/n+B6KD/DX7FMlYuMzNZgfJqhQrRhq0mG4G28eM582M5GHGdZvmTfL3xKWtoiB8Xf5aSgSHvjHbuXL7yvU9S88o7fKy8uGNlaZlAWMbR8GzPRuEW8q6AepxqszNFpWNFq+TIpb1TkG7CgCCQ5PupBVRTKh5ap5yE8CjXWotfC7XtaKOSpRbTpHTNr9iQKICftUeSq1Lzse6YbwJ7r0t+9YPhiE6cPjRy+hstAPOZG55HdyQxRjeb8A3n1VnSo3uINnkDxYYhzYeELF3pmiVhDwSYhPJ9+FqAan+pTSEgpu4FXjprXy9aeJcXq1AeEaPpI/96yqPo=
def part_2(input):
    Vec3 = collections.namedtuple("Vec3", "x,y,z", defaults = (0, 0, 0))

    def parse_task(lines):
        pts = []
        vels = []
        for line in lines:
            p, _, v = line.partition("@")
            pts.append(Vec3(*map(int, p.split(","))))
            vels.append(Vec3(*map(int, v.split(","))))
        return (pts, vels)

    def solve(pts, vels):
        n = len(pts)

        p1, v1 = pts[0], vels[0]
        for i in range(1, n):
            if indep(v1, vels[i]):
                p2, v2 = pts[i], vels[i]
                break
        for j in range(i+1, n):
            if indep(v1, vels[j]) and indep(v2, vels[j]):
                p3, v3 = pts[j], vels[j]
                break

        rock, S = find_rock(p1, v1, p2, v2, p3, v3)
        return sum(rock) / S

    def find_rock(p1, v1, p2, v2, p3, v3):
        a, A = find_plane(p1, v1, p2, v2)
        b, B = find_plane(p1, v1, p3, v3)
        c, C = find_plane(p2, v2, p3, v3)

        w = lin(A, cross(b, c), B, cross(c, a), C, cross(a, b))
        t = dot(a, cross(b, c))
        # given that w is integer, so force it here to avoid carrying through
        # imprecision
        # rest of the computation is integer except the final division
        w = Vec3(round(w.x / t), round(w.y / t), round(w.z / t))

        w1 = sub(v1, w)
        w2 = sub(v2, w)
        ww = cross(w1, w2)

        E = dot(ww, cross(p2, w2))
        F = dot(ww, cross(p1, w1))
        G = dot(p1, ww)
        S = dot(ww, ww)

        rock = lin(E, w1, -F, w2, G, ww)
        return (rock, S)

    def find_plane(p1, v1, p2, v2):
        p12 = sub(p1, p2)
        v12 = sub(v1, v2)
        vv = cross(v1, v2)
        return (cross(p12, v12), dot(p12, vv))

    def cross(a, b):
        return Vec3(a.y*b.z - a.z*b.y, a.z*b.x - a.x*b.z, a.x*b.y - a.y*b.x)

    def dot(a, b):
        return a.x*b.x + a.y*b.y + a.z*b.z

    def sub(a, b):
        return Vec3(a.x-b.x, a.y-b.y, a.z-b.z)

    def lin(r, a, s, b, t, c):
        x = r*a.x + s*b.x + t*c.x
        y = r*a.y + s*b.y + t*c.y
        z = r*a.z + s*b.z + t*c.z
        return Vec3(x, y, z)

    def indep(a, b):
        return any(v != 0 for v in cross(a, b))
    
    task = parse_task(input.splitlines())
    return int(solve(*task))