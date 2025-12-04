import { main } from ".";

describe("04", () => {
    describe("04", () => {
        it("should work", async () => {
            const result = await main("1", true);
            expect(result).toBe(13);
        });
    });

    describe("04", () => {
        it("should work", async () => {
            const result = await main("2", true);
            expect(result).toBe(43);
        });
    });
});
