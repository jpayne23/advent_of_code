import { readFile } from "../util/file";
import { logger } from "../util/logger";

const jumpTooFar = (a: number, b: number) => Math.abs(a - b) < 1 || Math.abs(a - b) > 3;

const checksAndBalances = (curr: number, prev: number, isDescending: boolean) => {
    if (jumpTooFar(curr, prev)) {
        return false;
    }

    if (isDescending) {
        if (curr > prev) return false;
    } else {
        if (curr < prev) return false;
    }

    return true;
};

export const isSafe = (input: number[], depth: number): Boolean => {
    let rescued = 0;
    logger.info({
        message: "input",
        data: {
            input,
            depth,
        },
    });

    const isDescending = input[0] > input[1];

    const result = input.some((curr, i, a) => {
        const prev = a[i - 1];
        if (i === 0) return false;

        if (checksAndBalances(curr, prev, isDescending)) return false;

        if (depth > 0) {
            const tryAgain = [...input];
            tryAgain.splice(i, 1);

            const recursiveSafe = isSafe(tryAgain, depth - 1);

            if (recursiveSafe) rescued += 1;

            return !recursiveSafe;
        }
        return true;
    });

    logger.info({
        message: "output",
        data: {
            safe: !result,
            input,
            depth,
            rescued,
        },
    });

    return !result;
};

const p1 = async (filePath: string) => {
    const result = await readFile(filePath);

    const answer = result.reduce((acc, r) => {
        const formatted = r.split(" ").map(v => parseInt(v));
        const safe = isSafe(formatted, 0);
        if (safe) {
            return (acc += 1);
        }

        return acc;
    }, 0);

    logger.info({
        message: "answer",
        data: {
            answer,
        },
    });

    return answer;
};

// 354
const p2 = async (filePath: string) => {
    const result = await readFile(filePath);

    const answer = result.reduce((acc, r) => {
        const formatted = r.split(" ").map(v => parseInt(v));
        const safe = isSafe(formatted, 0);

        if (safe) {
            logger.info({
                message: "another safe landing",
                data: {
                    formatted,
                },
            });

            return (acc += 1);
        }

        const trying = formatted.some((curr, i, a) => {
            const tryAgain = [...formatted];
            tryAgain.splice(i, 1);
            if (isSafe(tryAgain, 0)) {
                return true;
            }
            return false;
        });

        if (trying) return (acc += 1);

        return acc;
    }, 0);

    logger.info({
        message: "answer",
        data: {
            answer,
        },
    });

    return answer;
};

export const main = async (part: "1" | "2", test: Boolean) => {
    const filePath = test ? "02/test.txt" : "02/data.txt";
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
