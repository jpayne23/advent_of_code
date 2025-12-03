import { main } from ".";

describe("02", () => {
    describe("02", () => {
        it("should work", async () => {
            const result = await main("1", true);
            expect(result).toBe(1227775554);
        });
    });

    describe("02", () => {
        it("should work", async () => {
            const result = await main("2", true);
            expect(result).toBe(4174379265);
        });
    });
});
