import { readFile } from "../util/file";
import { logger } from "../util/logger";

const p1 = async (filePath: string) => {
    const result = await readFile(filePath);

    const regex = /mul\((\d{1,4}),(\d{1,4})\)/g;

    const answer = result.reduce((a, r) => {
        const matches = r.matchAll(regex);
        const row = [...matches].reduce((acc, m, i) => {
            const left = parseInt(m[1]);
            const right = parseInt(m[2]);

            return (acc += left * right);
        }, 0);
        return (a += row);
    }, 0);

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

    let enabled = true;

    const regex = /do\(\)|don't\(\)|mul\((\d{1,4}),(\d{1,4})\)/g;

    const answer = result.reduce((a, r) => {
        const matches = r.matchAll(regex);
        const row = [...matches].reduce((acc, m, i) => {
            if (m[0] === "don't()") {
                enabled = false;
                return acc;
            }

            if (m[0] === "do()") {
                enabled = true;
                return acc;
            }

            const left = parseInt(m[1]);
            const right = parseInt(m[2]);

            return enabled ? (acc += left * right) : acc;
        }, 0);
        return (a += row);
    }, 0);

    logger.info({
        message: "Answer",
        data: {
            answer,
        },
    });
    return answer;
};

export const main = async (part: "1" | "2", test: Boolean) => {
    const filePath = test ? `03/test${part}.txt` : "03/data.txt";
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
