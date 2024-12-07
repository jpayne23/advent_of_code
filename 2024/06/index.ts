import { readFile } from "../util/file";
import { logger } from "../util/logger";

const guardSymbols = ["<", "^", ">", "v"];
const obstacleSymbols = ["#"];

const getPlusOne = (direction: string, position: number[]) => {
    let positionDiff = [0, 0];
    switch (direction) {
        case "^":
            positionDiff = [-1, 0];
            break;
        case ">":
            positionDiff = [0, 1];
            break;
        case "<":
            positionDiff = [0, -1];
            break;
        case "v":
            positionDiff = [1, 0];
            break;
        default:
            break;
    }

    return {
        coords: [position[0] + positionDiff[0], position[1] + positionDiff[1]],
    };
};

const nextStep = (map: string[][], direction: string, position: number[]) => {
    let newDirection = direction;

    // Check +1 in guard direction
    let nextPosition = getPlusOne(direction, position);

    let symbol = map[nextPosition.coords[0]] && map[nextPosition.coords[0]][nextPosition.coords[1]];

    if (!symbol) {
        map[position[0]][position[1]] = "X";
        return {
            end: true,
        };
    }

    // Obstacle? Turn 90deg right
    if (obstacleSymbols.includes(symbol)) {
        const p = guardSymbols.findIndex(g => g === direction);

        newDirection = !guardSymbols[p + 1] ? guardSymbols[0] : guardSymbols[p + 1];

        nextPosition = getPlusOne(newDirection, position);
    }

    // Move +1
    map[position[0]][position[1]] = "X";

    map[nextPosition.coords[0]][nextPosition.coords[1]] = newDirection;

    return {
        direction: newDirection,
        position: nextPosition.coords,
    };
};

const p1 = async (filePath: string) => {
    const result = await readFile(filePath);

    const map = result.map(r => [...r]);

    let onMap = true;
    // y,x
    let guard = [0, 0];
    let direction = "";

    map.forEach((r, i) => {
        const position = r.findIndex(c => guardSymbols.includes(c));
        if (position > -1) {
            guard = [i, position];
            direction = r[position];
            return;
        }
    });

    do {
        const nextState = nextStep(map, direction, guard);

        if (nextState.end) {
            onMap = false;
        } else {
            guard = nextState.position || [-1, -1];
            direction = nextState.direction || "";
        }
    } while (onMap);

    const answer = map.reduce((acc, m) => {
        return (acc += m.reduce((c, n) => {
            return n === "X" ? (c += 1) : c;
        }, 0));
    }, 0);

    logger.info({
        message: "Parsed",
        data: {
            result,
            guard,
            direction,
            map: map.map(m => JSON.stringify(m)),
            answer,
        },
    });

    return answer;
};

const p2 = async (filePath: string) => {
    const result = await readFile(filePath);
};

export const main = async (part: "1" | "2", test: Boolean) => {
    const filePath = test ? `06/test.txt` : "06/data.txt";
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
