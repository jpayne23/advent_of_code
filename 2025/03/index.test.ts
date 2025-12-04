import { main } from ".";

describe("03", () => {
    describe("03", () => {
        it("should work", async () => {
            const result = await main("1", true);
            expect(result).toBe(357);
        });
    });

    describe("03", () => {
        it("should work", async () => {
            const result = await main("2", true);
            expect(result).toBe(3121910778619);
        });
    });
});
