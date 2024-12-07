import { Command } from "commander";
import { main as day1 } from "./01";
import { main as day2 } from "./02";
import { main as day3 } from "./03";
import { main as day4 } from "./04";
import { main as day5 } from "./05";
import { main as day6 } from "./06";

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
        case "4":
            await day4(part, !!test);
            break;
        case "5":
            await day5(part, !!test);
            break;
        case "6":
            await day6(part, !!test);
            break;

        default:
            break;
    }
};

(async () => await main())();
