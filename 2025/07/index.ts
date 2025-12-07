import { readFile } from "../util/file";

const startSymbol = "S";
const splitterSymbol = "^";

const p1 = async (filePath: string) => {
    const result = await readFile(filePath);
    let beams: number[] = [result[0].indexOf(startSymbol)];

    let count = 0;
    // Ignore first and last row
    for (let i = 1; i < result.length - 1; i++) {
        const line = result[i];
        const nextRowBeams: number[] = [];

        beams.forEach(b => {
            if (line[b] === splitterSymbol) {
                count = count + 1;
                if (b - 1 > -1) {
                    nextRowBeams.push(b - 1);
                }
                if (b + 1 < line.length) {
                    nextRowBeams.push(b + 1);
                }
            } else {
                nextRowBeams.push(b);
            }
        });

        beams = [...new Set(nextRowBeams)];
    }

    return count;
};

const p2 = async (filePath: string) => {
    const result = await readFile(filePath);

    let beams: number[] = [result[0].indexOf(startSymbol)];

    let pathCountByIndex: { [idx: number]: number }[] = [{ [result[0].indexOf(startSymbol)]: 1 }];

    // Ignore first and last row
    for (let i = 1; i < result.length; i++) {
        const line = result[i];
        const nextRowBeams: number[] = [];

        pathCountByIndex.push({});

        beams.forEach(b => {
            if (line[b] === splitterSymbol) {
                if (b - 1 > -1) {
                    nextRowBeams.push(b - 1);
                    pathCountByIndex[i][b - 1] = pathCountByIndex[i][b - 1]
                        ? pathCountByIndex[i - 1][b] + (pathCountByIndex[i][b - 1] || 0)
                        : pathCountByIndex[i - 1][b];
                }
                if (b + 1 < line.length) {
                    nextRowBeams.push(b + 1);
                    pathCountByIndex[i][b + 1] = pathCountByIndex[i][b + 1]
                        ? pathCountByIndex[i - 1][b] + (pathCountByIndex[i][b + 1] || 0)
                        : pathCountByIndex[i - 1][b];
                }

                // At splitter so no paths contained here now
                pathCountByIndex[i][b] = 0;
            } else {
                // Just carry on down
                nextRowBeams.push(b);
                pathCountByIndex[i][b] = pathCountByIndex[i][b]
                    ? pathCountByIndex[i][b] + (pathCountByIndex[i - 1][b] || 0)
                    : pathCountByIndex[i - 1][b];
            }
        });

        beams = [...new Set(nextRowBeams)];
    }

    return Object.values(pathCountByIndex[pathCountByIndex.length - 1]).reduce((acc, curr) => acc + curr, 0);
};

export const main = async (part: "1" | "2", test: Boolean) => {
    const filePath = test ? "07/test.txt" : "07/data.txt";
    switch (part) {
        case "1":
            return await p1(filePath);
        case "2":
            return await p2(filePath);
        default:
            throw new Error("Part can only be 1 or 2");
    }
};
