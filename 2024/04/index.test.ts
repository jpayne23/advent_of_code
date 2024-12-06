import { main } from ".";

describe("01", () => {
    describe("01", () => {
        it("should work", async () => {
            const result = await main("1", true);
            expect(result).toBe(18);
        });
    });

    describe("02", () => {
        it("should work", async () => {
            const result = await main("2", true);
            expect(result).toBe(9);
        });
    });
});
