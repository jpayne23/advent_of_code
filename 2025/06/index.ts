import { readFile } from "../util/file";

const p1 = async (filePath: string) => {
    const result = await readFile(filePath);

    const problems: { operation: string; values: number[] }[] = [];

    result[result.length - 1]
        .match(/(\+|\*)/g)
        ?.filter(op => op !== "")
        .forEach(op => {
            problems.push({ operation: op, values: [] });
        });

    // Treat last line separately for operations
    for (let i = 0; i < result.length - 1; i++) {
        result[i]
            .match(/\d*/g)
            ?.filter(v => v !== "")
            .forEach((v, j) => {
                problems[j].values.push(Number(v));
            });
    }

    return problems.reduce((acc, { operation, values }) => {
        if (operation === "*") {
            return (
                acc +
                values.reduce((acc1, curr) => {
                    return acc1 * curr;
                }, 1)
            );
        }

        if (operation === "+") {
            return (
                acc +
                values.reduce((acc1, curr) => {
                    return acc1 + curr;
                }, 0)
            );
        }

        return acc;
    }, 0);
};

const p2 = async (filePath: string) => {
    const result = await readFile(filePath);

    const problems: { operation: string; operationindex: number; values: string[] }[] = [];

    // Get position of each op as that is always the left most index of problem
    result[result.length - 1].split("").forEach((op, i) => {
        if (op.trim() !== "") {
            problems.push({ operation: op, operationindex: i, values: [] });
        }
    });

    for (let i = 0; i < problems.length; i++) {
        const startIdx = problems[i].operationindex;
        // Handle when out of range
        const endIdx = i + 1 >= problems.length ? null : problems[i + 1].operationindex;

        for (let j = 0; j < result.length - 1; j++) {
            // -1 as there is always a column of whitespace
            if (endIdx === null) {
                problems[i].values.push(result[j].substring(startIdx).split("").reverse().join(""));
                continue;
            }
            problems[i].values.push(
                result[j]
                    .substring(startIdx, endIdx - 1)
                    .split("")
                    .reverse()
                    .join("")
            );
        }
    }

    let cols = [];

    problems.forEach(problem => {
        cols = [];
        const maxLength = problem.values[0].length;

        for (let i = 0; i < maxLength; i++) {
            let newVal = "";
            problem.values.forEach(p => {
                newVal = newVal + (p[i] || "");
            });

            cols.push(newVal.trim());
        }

        problem.values = cols;
    });

    return problems.reduce((acc, { operation, values }) => {
        if (operation === "*") {
            return (
                acc +
                values.reduce((acc1, curr) => {
                    return acc1 * Number(curr);
                }, 1)
            );
        }

        if (operation === "+") {
            return (
                acc +
                values.reduce((acc1, curr) => {
                    return acc1 + Number(curr);
                }, 0)
            );
        }

        return acc;
    }, 0);
};

export const main = async (part: "1" | "2", test: Boolean) => {
    const filePath = test ? "06/test.txt" : "06/data.txt";
    switch (part) {
        case "1":
            return await p1(filePath);
        case "2":
            return await p2(filePath);
        default:
            throw new Error("Part can only be 1 or 2");
    }
};
