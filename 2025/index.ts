import { Command } from "commander";
import { main as day1 } from "./01";
import { main as day2 } from "./02";
import { main as day3 } from "./03";
import { main as day4 } from "./04";
import { logger } from "./util/logger";

const program = new Command();
program
    .name("Advent of Code 2024")
    .description("Execute any Advent of Code 2024 day and part")
    .option("-d, --day <day>", "Advent of Code day")
    .option("-p, --part <part>", "Advent of Code part")
    .option("-t, --test", "Run using test data")
    .parse(process.argv);

const options = program.opts();

const { day, part, test } = options;

const main = async () => {
    switch (day) {
        case "1":
            return await day1(part, !!test);
        case "2":
            return await day2(part, !!test);
        case "3":
            return await day3(part, !!test);
        case "4":
            return await day4(part, !!test);
        default:
            throw new Error("Invalid day specified");
    }
};

(async () => {
    try {
        const answer = await main();

        logger.info({
            message: "Answer",
            data: {
                answer,
            },
        });
    } catch (err) {
        logger.error({
            message: "Error running Advent of Code",
            data: { error: (err as Error).message },
        });
    }
})();
