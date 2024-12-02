import fs from "node:fs";
import readline from "node:readline";
import { once } from "node:events";
import { logger } from "./logger";

export const readFile = async (filePath: string): Promise<string[]> => {
    try {
        const rows: string[] = [];

        const rl = readline.createInterface({
            input: fs.createReadStream(filePath),
            crlfDelay: Infinity,
        });

        rl.on("line", line => {
            rows.push(line);
        });

        await once(rl, "close");

        return rows;
    } catch (err) {
        logger.error({
            message: "Failed to read file",
            data: {
                error: err,
            },
        });
        throw new Error("Failed to read file");
    }
};
