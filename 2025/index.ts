import { Command } from "commander";
import { main as day1 } from "./01";
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
