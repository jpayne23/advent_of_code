import { readFile } from "../util/file";

const p1 = async (filePath: string) => {
    const result = await readFile(filePath);

    const rows = result[0].split(",").map(l => ({ start: l.split("-")[0], end: l.split("-")[1] }));

    let answer = 0;
    rows.forEach(({ start, end }) => {
        if (start.length === end.length && start.length % 2 === 1) {
            return;
        }

        for (let i = Number(start); i <= Number(end); i++) {
            const asStr = i.toString();
            if (asStr.length % 2 === 0) {
                if (asStr.substring(0, asStr.length / 2) === asStr.substring(asStr.length / 2)) {
                    answer = answer + i;
                }
            }
        }
    });

    return answer;
};

const p2 = async (filePath: string) => {
    const result = await readFile(filePath);

    const rows = result[0].split(",").map(l => ({ start: l.split("-")[0], end: l.split("-")[1] }));

    let answer = 0;
    rows.forEach(({ start, end }) => {
        for (let i = Number(start); i <= Number(end); i++) {
            const asStr = i.toString();

            for (let j = 0; j <= asStr.length / 2 - 1; j++) {
                const str = asStr.substring(0, j + 1);
                const arr = [];

                for (let k = 0; k < asStr.length; k = k + str.length) {
                    arr.push(asStr.substring(k, k + str.length));
                }

                if (arr.every(a => a == str)) {
                    answer = answer + i;
                    break;
                }
            }
        }
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
            throw new Error("Part can only be 1 or 2");
    }
};
