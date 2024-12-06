import { readFile } from "../util/file";
import { logger } from "../util/logger";

const p1 = async (filePath: string) => {
    const result = await readFile(filePath);

    const toSplit = result.findIndex(r => r === "");

    const orderings = result.slice(0, toSplit).map(o => o.split("|"));
    const list = result.slice(toSplit + 1).map(l => l.split(","));

    const newList = list.filter((l, i) => {
        return !orderings.some(([a, b]) => {
            const aIndex = l.findIndex(e => e === a);
            const bIndex = l.findIndex(e => e === b);

            if (aIndex !== -1 && bIndex !== -1 && aIndex > bIndex) {
                return true;
            }
        });
    });

    const answer = newList.reduce((acc, l) => {
        return (acc += parseInt(l[(l.length - 1) / 2]));
    }, 0);

    logger.info({
        message: "answer",
        data: { answer },
    });

    return answer;
};

const p2 = async (filePath: string) => {
    const result = await readFile(filePath);

    const toSplit = result.findIndex(r => r === "");

    const orderings = result.slice(0, toSplit).map(o => o.split("|"));
    const list = result.slice(toSplit + 1).map(l => l.split(","));

    const newList: string[][] = [];

    list.forEach(l => {
        let badOrder = false;
        let swapped = false;
        let iteration = 0;

        do {
            swapped = false;
            iteration += 1;
            orderings.forEach(([a, b]) => {
                const aIndex = l.findIndex(e => e === a);
                const bIndex = l.findIndex(e => e === b);

                if (aIndex !== -1 && bIndex !== -1 && aIndex > bIndex) {
                    const temp = l[aIndex];
                    l[aIndex] = l[bIndex];
                    l[bIndex] = temp;
                    badOrder = true;
                    swapped = true;
                }
            });
        } while (swapped);

        if (badOrder) {
            newList.push(l);
        }
    });

    const answer = newList.reduce((acc, l) => {
        return (acc += parseInt(l[(l.length - 1) / 2]));
    }, 0);

    logger.info({
        message: "answer",
        data: { answer },
    });

    return answer;
};

export const main = async (part: "1" | "2", test: Boolean) => {
    const filePath = test ? `05/test.txt` : "05/data.txt";
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
