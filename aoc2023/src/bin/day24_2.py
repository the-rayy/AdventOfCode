from z3 import *

if __name__ == '__main__':
    hails = []
    with open('data/day24.txt') as f:
        data = f.read().splitlines()

        for line in data:
            line = line.replace(" @", ",")
            split = [float(x.strip()) for x in line.split(", ")]
            hails.append({
                'pos': [split[0], split[1], split[2]],
                'vel': [split[3], split[4], split[5]],
            })

    tpx = Real('tpx')
    tpy = Real('tpy')
    tpz = Real('tpz')
    tvx = Real('tvx')
    tvy = Real('tvy')
    tvz = Real('tvz')
    tt1 = Real('tt1')
    tt2 = Real('tt2')
    tt3 = Real('tt3')

    s = Solver()
    s.add(tpx + tvx * tt1 == hails[0]['pos'][0] + hails[0]['vel'][0] * tt1)
    s.add(tpy + tvy * tt1 == hails[0]['pos'][1] + hails[0]['vel'][1] * tt1)
    s.add(tpz + tvz * tt1 == hails[0]['pos'][2] + hails[0]['vel'][2] * tt1)
    s.add(tpx + tvx * tt2 == hails[1]['pos'][0] + hails[1]['vel'][0] * tt2)
    s.add(tpy + tvy * tt2 == hails[1]['pos'][1] + hails[1]['vel'][1] * tt2)
    s.add(tpz + tvz * tt2 == hails[1]['pos'][2] + hails[1]['vel'][2] * tt2)
    s.add(tpx + tvx * tt3 == hails[2]['pos'][0] + hails[2]['vel'][0] * tt3)
    s.add(tpy + tvy * tt3 == hails[2]['pos'][1] + hails[2]['vel'][1] * tt3)
    s.add(tpz + tvz * tt3 == hails[2]['pos'][2] + hails[2]['vel'][2] * tt3)

    s.check()
    res = s.model()[tpx].as_long() + s.model()[tpy].as_long() + s.model()[tpz].as_long()
    print(res)
