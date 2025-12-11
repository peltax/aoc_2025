let TEST_DATA = [
    "3-5",
    "10-14",
    "16-20",
    "12-18",
    "",
    "1",
    "5",
    "8",
    "11",
    "17",
    "32",
];

interface Range { start: number; end: number; }
function isValueInAnyRange(value: number, ranges: Array<Range>) {
    // part 1
    for (const range of ranges) {
        if (value >= range.start && value <= range.end) {
            return true;
        }
    }
    return false;
}

function sumOverlappingRangesDistance(ranges: Array<Range>) {
    // part 2
    if (ranges.length === 0) {
        return 0n;
    }

    // sort before comparison
    const sortedRanges = [...ranges].sort((a, b) => {
        if (a.start < b.start) return -1;
        if (a.end > b.end) return 1;
        return 0;
    });

    let distance: number = 0;
    let curStart = sortedRanges[0].start;
    let curEnd = sortedRanges[0].end;

    for (let i = 1; i < sortedRanges.length; i++) {
        const nextStart = sortedRanges[i].start;
        const nextEnd = sortedRanges[i].end;

        if (nextStart <= curEnd + 1) {
            if (nextEnd > curEnd) {
                curEnd = nextEnd;
            }
        } else {
            const curDist = curEnd - curStart + 1;
            distance += curDist;

            curStart = nextStart;
            curEnd = nextEnd;
        }
    }

    const lastDistance = curEnd - curStart + 1;
    distance += lastDistance;

    return distance;
}

function solve_day5(input: Array<string>) {
    let values = input
            .filter((x) => x.length > 0 && !x.includes('-'))
            .map((v) => Number(v));

    let ranges: Array<Range> = input
        .filter((x) => x.length > 0 && x.includes('-'))
        .map((x) => {
            const [rangeStart, rangeEnd] = x.split('-');
            return { start: Number(rangeStart), end: Number(rangeEnd) } as Range;
        });

    console.log('Unique distance: ', sumOverlappingRangesDistance(ranges));

   return values
       .map((x) => Number(isValueInAnyRange(x, ranges)))
       .reduce((acc, x) => acc + x);
}

console.log(solve_day5(TEST_DATA))
//console.log(solve_day5(DATA));
