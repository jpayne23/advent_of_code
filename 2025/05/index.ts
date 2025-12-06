import { readFile } from "../util/file";

const p1 = async (filePath: string) => {
    const result = await readFile(filePath);

    const breaklineIndex = result.findIndex(line => line.trim() === "");

    const extractStartEnd = /(\d*)-(\d*)/;

    const ranges = result.slice(0, breaklineIndex).map(line => {
        const res = line.match(extractStartEnd) || [];
        return [Number(res[1]), Number(res[2])];
    });
    const inputs = result.slice(breaklineIndex + 1).map(line => Number(line.trim()));

    let answer = 0;

    inputs.forEach(i => {
        if (ranges.some(([start, end]) => i >= start && i <= end)) {
            answer = answer + 1;
        }
    });

    return answer;
};

const p2 = async (filePath: string) => {
    const result = await readFile(filePath);

    const breaklineIndex = result.findIndex(line => line.trim() === "");

    const extractStartEnd = /(\d*)-(\d*)/;

    const ranges = result.slice(0, breaklineIndex).map(line => {
        const res = line.match(extractStartEnd) || [];
        return [Number(res[1]), Number(res[2])];
    });

    // Combine ranges to remove overlaps
    // ignore self
    // if s <= s1 and e >= s1 then new range is s - max(e,e1)
    // if s <= e1 and e >= e1 then new range is min(s,s1) - e
    // let combined: number[][] = ranges.sort((a, b) => b[0] - a[0]);
    let combined: number[][] = [...ranges];
    let changed = false;

    let init: number[][] = [];

    do {
        init = [...combined];
        combined = [];

        changed = false;
        for (let i = 0; i < init.length; i++) {
            const [s, e] = init[i];
            changed = false;
            for (let j = 0; j < init.length; j++) {
                if (i === j) continue;
                const [s1, e1] = init[j];
                if (s <= s1 && e >= s1) {
                    init = init.filter(([sc, ec]) => !(sc === s1 && ec === e1) && !(sc === s && ec === e));
                    changed = true;
                    init.push([s, Math.max(e, e1)]);
                    break;
                }
                if (s <= e1 && e >= e1) {
                    init = init.filter(([sc, ec]) => !(sc === s1 && ec === e1) && !(sc === s && ec === e));
                    changed = true;
                    init.push([Math.min(s, s1), e]);
                    break;
                }
            }
            if (changed) break;
        }

        combined = [...init];
    } while (changed);

    return combined.reduce((acc, [s, e]) => acc + (e - s + 1), 0);
};

export const main = async (part: "1" | "2", test: Boolean) => {
    const filePath = test ? "05/test.txt" : "05/data.txt";
    switch (part) {
        case "1":
            return await p1(filePath);
        case "2":
            return await p2(filePath);
        default:
            throw new Error("Part can only be 1 or 2");
    }
};
