const TEST_INPUT: Array<number> = ("" +
    "..@@.@@@@.\n" +
    "@@@.@.@.@@\n" +
    "@@@@@.@.@@\n" +
    "@.@@@@..@.\n" +
    "@@.@@@@.@@\n" +
    ".@@@@@@@.@\n" +
    ".@.@.@.@@@\n" +
    "@.@@@.@@@@\n" +
    ".@@@@@@@@.\n" +
    "@.@.@@@.@.")
        .split('\n')
        .flatMap((c) => c.split('').map((c) => c === '@' ? 1 : 0));


// Path finding on a flattened 2D grid - considers nearest neighbours
function isToiletPaperAccessible(pointIndex: number, data: Array<number>, rowLength: number): boolean {
    if (!data[pointIndex]) {
        return false;
    }

    const dataLen = data.length;
    const dirToIndexMap = [
        -rowLength,      // North
        -rowLength + 1,  // North-East
        1,               // East
        +rowLength + 1,  // South-East
        +rowLength,      // South
        +rowLength - 1,  // South-West
        -1,              // West
        -rowLength - 1,  // North-West
    ];

    return (dirToIndexMap.map((d, d_idx) => {
        let idx = pointIndex + d;

        let colIdx = Math.floor(pointIndex % rowLength);
        let neighborColIdx = Math.floor(idx % rowLength);

        if (
            (neighborColIdx === rowLength - 1 && colIdx === 0) || // West wraps around
            (neighborColIdx === 0 && colIdx === rowLength - 1) || // East wraps around
            (idx < 0 || idx > dataLen - 1) // out of bounds
        ) {
            return 0;
        }

        return data[idx];
    }).reduce((acc, x) => acc + x)) < 4;
}

function solve_part1(data: Array<number>, rowLength: number) {
    let toiletSum = 0;
    let shadowData = [...data]; // for visualizing
    for (let i = 0; i < data.length; i++) {
        let suitableToiletPaper = isToiletPaperAccessible(i, data, rowLength);
        if (suitableToiletPaper) {
            shadowData[i] = 2;
            toiletSum++;
        }
    }

    let reconstructed = '';
    let restored = shadowData.map((x, i) => x === 2 ? 'x' : x === 1 ? '@' : '.').join('');
    for (let i = 0; i < shadowData.length / rowLength; i++) {
        reconstructed += restored.slice(i * rowLength, i * rowLength + rowLength);
        reconstructed += '\n';
    }
    console.log(reconstructed);

    return toiletSum;
}

function solve_part2(data: Array<number>, rowLength: number) {
    let shadowData = [...data];
    let removals = [];
    let toiletCount = 0;
    do {
        while (removals.length > 0) {
            const index = removals.pop();
            shadowData[index] = 0;
            toiletCount++;
        }

        removals = [];
        for (let i = 0; i < shadowData.length; i++) {
            if (isToiletPaperAccessible(i, shadowData, rowLength)) {
                removals.push(i);
            }
        }
    } while (removals.length > 0);

    return toiletCount;
}

const result_pt1 = solve_part1(TEST_INPUT, 10);
const result_pt2 = solve_part2(TEST_INPUT, 10);
console.log('Result part1', result_pt1, 'part2', result_pt2);
