import { readFile } from "../util/file";
import { logger } from "../util/logger";

const surrounding = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

const surrounding2 = [
    [-1, -1],
    [-1, 1],
    [1, -1],
    [1, 1],
];

const findSurroundingChar = (x: number, y: number, grid: string[][], char: string) => {
    const matches: [number, number][] = [];
    surrounding.forEach(([x1, y1]) => {
        if (grid[y + y1] && grid[y + y1][x + x1] === char) {
            matches.push([x1, y1]);
        }
    });

    return matches;
};

const findSurroundingChar2 = (x: number, y: number, grid: string[][], char: string) => {
    const matches: [number, number][] = [];
    surrounding2.forEach(([x1, y1]) => {
        if (grid[y + y1] && grid[y + y1][x + x1] === char) {
            matches.push([x1, y1]);
        }
    });

    return matches;
};

const p1 = async (filePath: string) => {
    const result = await readFile(filePath);

    const theMatrix = result.map(r => [...r]);

    let answer = 0;
    const final: Record<any, any>[] = [];

    theMatrix.forEach((r, y) => {
        r.forEach((c, x) => {
            if (c === "X") {
                // Look for an "M"
                const matches = findSurroundingChar(x, y, theMatrix, "M");

                matches.forEach(([x1, y1]) => {
                    if (
                        theMatrix[y + y1 * 2] &&
                        theMatrix[y + y1 * 2][x + x1 * 2] === "A" &&
                        theMatrix[y + y1 * 3] &&
                        theMatrix[y + y1 * 3][x + x1 * 3] === "S"
                    ) {
                        final.push({ x, y, x1, y1 });
                        answer += 1;
                    }
                });
            }
        });
    });

    logger.info({
        message: "Answer",
        data: {
            answer,
        },
    });
    return answer;
};

const p2 = async (filePath: string) => {
    const result = await readFile(filePath);

    const theMatrix = result.map(r => [...r]);

    let answer = 0;
    const final: Record<any, any>[] = [];

    theMatrix.forEach((r, y) => {
        r.forEach((c, x) => {
            if (c === "A") {
                // Look for an "M"
                const matches = findSurroundingChar2(x, y, theMatrix, "M");

                let isMatch = true;
                matches.forEach(([x1, y1]) => {
                    if (!theMatrix[y + y1 * -1] || theMatrix[y + y1 * -1][x + x1 * -1] !== "S") {
                        isMatch = false;
                    }
                });
                if (matches.length === 2 && isMatch) {
                    final.push({ x, y });
                    answer += 1;
                }
            }
        });
    });

    logger.info({
        message: "Answer",
        data: {
            answer,
        },
    });
    return answer;
};

export const main = async (part: "1" | "2", test: Boolean) => {
    const filePath = test ? `04/test.txt` : "04/data.txt";
    switch (part) {
        case "1":
            return await p1(filePath);
        case "2":
            return await p2(filePath);
        default:
            console.error("Part can only be 1 or 2");
            break;
    }
};
