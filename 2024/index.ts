import { Command } from "commander";
import { main as day1 } from "./01";
import { main as day2 } from "./02";
import { main as day3 } from "./03";

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
            await day1(part, !!test);
            break;
        case "2":
            await day2(part, !!test);
            break;
        case "3":
            await day3(part, !!test);
            break;

        default:
            break;
    }
};

(async () => await main())();
