import { readFile } from "../util/file";

const searchSymbol = "@";

const search = (x: number, y: number, data: string[]) => {
    const searchArea: Array<[number, number]> = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];
    let count = 0;

    searchArea.forEach(([x1, y1]) => {
        const xSearch = x + x1;
        const ySearch = y + y1;

        if (xSearch < 0 || xSearch > data.length - 1 || ySearch < 0 || ySearch > data[xSearch].length - 1) return;

        const symbol = data[xSearch][ySearch];
        if (symbol === searchSymbol) {
            count++;
        }
    });

    return count;
};

const p1 = async (filePath: string) => {
    const result = await readFile(filePath);

    let answer = 0;

    result.forEach((row, rowIdx) => {
        row.split("").forEach((col, colIdx) => {
            if (col !== searchSymbol) return;

            const res = search(rowIdx, colIdx, result);
            if (res < 4) answer++;
        });
    });

    return answer;
};

const p2 = async (filePath: string) => {
    const result = await readFile(filePath);

    const doIt = (data: string[]): number => {
        const newData = [...data];

        let answer = 0;
        let changed = false;

        data.forEach((row, rowIdx) => {
            row.split("").forEach((col, colIdx) => {
                if (col !== "@") return;

                const res = search(rowIdx, colIdx, data);
                if (res < 4) {
                    answer++;
                    changed = true;
                    newData[rowIdx] =
                        newData[rowIdx].substring(0, colIdx) + "." + newData[rowIdx].substring(colIdx + 1);
                }
            });
        });

        if (!changed) {
            return answer;
        }

        return answer + doIt(newData);
    };

    const answer = doIt(result);

    return answer;
};

export const main = async (part: "1" | "2", test: Boolean) => {
    const filePath = test ? "04/test.txt" : "04/data.txt";
    switch (part) {
        case "1":
            return await p1(filePath);
        case "2":
            return await p2(filePath);
        default:
            throw new Error("Part can only be 1 or 2");
    }
};
