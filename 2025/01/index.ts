import { readFile } from "../util/file";

const separator = "  ";

const p1 = async (filePath: string) => {
    const result = await readFile(filePath);

    const left = result.map(r => parseInt(r.split(separator)[0].trim())).sort();
    const right = result.map(r => parseInt(r.split(separator)[1].trim())).sort();

    const answer = left.reduce((acc, r, i) => {
        return (acc += Math.abs(r - right[i]));
    }, 0);

    return answer;
};

const p2 = async (filePath: string) => {
    const result = await readFile(filePath);

    const left = result.map(r => parseInt(r.split(separator)[0].trim())).sort();
    const right = result.map(r => parseInt(r.split(separator)[1].trim())).sort();

    const answer = left.reduce((acc, r, i) => {
        return (acc += r * right.filter(v => r === v).length);
    }, 0);

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
