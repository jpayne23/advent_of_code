import { readFile } from "../util/file";

const p1 = async (filePath: string) => {
    const result = await readFile(filePath);

    let answer = 0;
    result.forEach(line => {
        let firstDigitPos = -1;
        let firstDigit = -1;
        for (let i = 9; i > 0; i--) {
            const idx = line.split("").findIndex(v => Number(v) === i);
            if (idx !== -1 && idx < line.length - 1) {
                firstDigitPos = idx;
                firstDigit = i;
                break;
            }
        }

        const second = line.substring(firstDigitPos + 1);

        let secondDigit = -1;
        for (let i = 9; i > 0; i--) {
            const idx = second.split("").findIndex(v => Number(v) === i);
            if (idx !== -1) {
                secondDigit = i;
                break;
            }
        }

        const res = Number(firstDigit.toString() + secondDigit.toString());

        answer = answer + res;
    });

    return answer;
};

const p2 = async (filePath: string) => {
    const result = await readFile(filePath);

    const getBiggestJoltage = (line: string, remainingDigits: number, currentJoltage: string): string => {
        if (remainingDigits === 0) {
            return currentJoltage;
        }

        if (remainingDigits === line.length) {
            return currentJoltage + line;
        }

        for (let i = 9; i >= 0; i--) {
            const idx = line.split("").findIndex(v => Number(v) === i);
            if (idx !== -1 && idx <= line.length - remainingDigits) {
                const joltage = i.toString();
                return currentJoltage + getBiggestJoltage(line.substring(idx + 1), remainingDigits - 1, joltage);
            }
        }

        return currentJoltage;
    };

    let answer = 0;
    result.forEach(line => {
        const res = getBiggestJoltage(line, 12, "");

        answer = answer + Number(res);
    });

    return answer;
};

export const main = async (part: "1" | "2", test: Boolean) => {
    const filePath = test ? "03/test.txt" : "03/data.txt";
    switch (part) {
        case "1":
            return await p1(filePath);
        case "2":
            return await p2(filePath);
        default:
            throw new Error("Part can only be 1 or 2");
    }
};
