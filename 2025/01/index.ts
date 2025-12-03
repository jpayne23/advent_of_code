import { readFile } from "../util/file";

const p1 = async (filePath: string) => {
    const result = await readFile(filePath);
    const initialPosition = 50;

    const instructions = result.map(r => {
        if (r.startsWith("R")) {
            return Number(r.slice(1).trim());
        }

        return Number(r.slice(1).trim()) * -1;
    });

    let answer = 0;

    instructions.reduce((acc, r) => {
        acc = acc + r;

        if (acc < 0) {
            acc = 100 * Math.floor(Math.abs(acc) / 100) + acc;
        }

        if (acc > 99) {
            acc = acc - 100 * Math.floor(acc / 100);
        }

        if (acc === 0) {
            answer = answer + 1;
        }

        return acc;
    }, initialPosition);

    return answer;
};

const p2 = async (filePath: string) => {
    const result = await readFile(filePath);
    const initialPosition = 50;

    const instructions = result.map(r => {
        if (r.startsWith("R")) {
            return Number(r.slice(1).trim());
        }

        return Number(r.slice(1).trim()) * -1;
    });

    let answer = 0;

    instructions.reduce((acc, r) => {
        let prev = acc;
        acc = acc + r;

        if (acc < 0) {
            const loops = Math.floor(Math.abs(acc) / 100) + 1;
            acc = 100 * loops + acc;

            answer = answer + loops;

            if (prev === 0) {
                answer = answer - 1;
            }
        }

        if (acc > 99) {
            const loops = Math.floor(Math.abs(acc) / 100);
            const pre = acc;
            acc = acc - 100 * loops;

            answer = answer + loops;

            if (pre % 100 === 0) {
                answer = answer - 1;
            }

            // if (prev === 0) {
            //     answer = answer - 1;
            // }
        }

        if (acc === 0) {
            answer = answer + 1;
        }

        return acc;
    }, initialPosition);

    // Somewhere between 6908 and 7042
    return answer;
};

export const main = async (part: "1" | "2", test: Boolean) => {
    const filePath = test ? "01/test.txt" : "01/data.txt";
    switch (part) {
        case "1":
            return await p1(filePath);
        case "2":
            return await p2(filePath);
        default:
            throw new Error("Part can only be 1 or 2");
    }
};
